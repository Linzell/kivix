// governor.rs
use std::{net::{IpAddr, SocketAddr}, str::FromStr};
use actix_governor::{KeyExtractor, SimpleKeyExtractionError};
use actix_web::web;
use log::error;

use crate::utils::env::get_env_or;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Nginx configuration for IP address extraction
/// 
/// ## Fields
/// 
/// * `proxy_ip` is the IP address of the reverse proxy
/// * `whitelisted` is a list of whitelisted IP addresses
/// * `key_name` is the name of the key
pub(super) struct NginxIpKeyExctrator;

/// Macro rule to create a SimpleKeyExtractionError with a static string if IP address extraction fails
macro_rules! couldntExtract {
    () => {
        actix_governor::SimpleKeyExtractionError::new("Could not extract real IP Address from request")
    };
}

/// Implementation of KeyExtractor for NginxIpKeyExctrator
/// 
/// ## Fields
/// 
/// * `Key` is the IP address of the request
/// * `KeyExtractionError` is the error type for IP address extraction
/// 
/// ## Methods
/// 
/// * `name` returns the name of the key
/// * `extract` extracts the IP address from the request
/// * `whitelisted_keys` returns a list of whitelisted IP addresses
/// * `key_name` returns the name of the key
impl KeyExtractor for NginxIpKeyExctrator {
    type Key = IpAddr;

    type KeyExtractionError = SimpleKeyExtractionError<&'static str>;

    /// Returns the name of the key
    /// 
    /// ## Returns
    /// 
    /// Returns the name of the key
    fn name(&self) -> &'static str {
        "Proxy IP"
    }

    /// Extracts the IP address from the request
    /// 
    /// ## Parameters
    /// 
    /// * `req` is the request
    /// 
    /// ## Returns
    /// 
    /// Returns the IP address of the request
    /// 
    /// ## Errors
    /// 
    /// Returns an error if the IP address could not be extracted
    fn extract(&self, req: &actix_web::dev::ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        let proxy_ip = req
            .app_data::<web::Data<IpAddr>>()
            .map(|ip| ip.get_ref().to_owned())
            .unwrap_or_else(|| IpAddr::from_str("0.0.0.0").unwrap());

        let peer_ip = req.peer_addr().map(|socket| socket.ip());
        let connection_info = req.connection_info();

        match peer_ip {
            // request is from reverse proxy, so use 'Forwarded' or 'X-Forwarded-For' header
            Some(peer) if peer == proxy_ip => connection_info
                .realip_remote_addr()
                .ok_or_else(|| { couldntExtract!() })
                .and_then(|str| {
                    SocketAddr::from_str(str)
                        .map(|socket| socket.ip())
                        .or_else(|_| IpAddr::from_str(str))
                        .map_err(|_| { couldntExtract!() })
                }),
            Some(peer) => {
                if cfg!(not(debug_assertions)) {
                    if peer.to_string() != "127.0.0.1" {
                        error!("!!!FATAL!!! SERVER MISCONFIGURED, GOT REQUEST FROM REVERSE PROXY DIRECTLY");
                        panic!();
                    }
                }
                connection_info
                    .peer_addr()
                    .ok_or_else(|| { couldntExtract!() })
                    .and_then(|str| {
                        SocketAddr::from_str(str).map_err(|_| {couldntExtract!()})
                    })
                    .map(|socket| socket.ip())
            }
            _ => {
                if cfg!(not(debug_assertions)) {
                    error!("!!!FATAL!!! SERVER MISCONFIGURED, GOT OUTSIDE REQUEST NOT THROUGH PROXY");
                    panic!();
                }
                connection_info
                    .peer_addr()
                    .ok_or_else(|| { couldntExtract!() })
                    .and_then(|str| {
                        SocketAddr::from_str(str).map_err(|_| {couldntExtract!()})
                    })
                    .map(|socket| socket.ip())
            }
        }
    }

    /// Returns a list of whitelisted IP addresses
    /// 
    /// ## Returns
    /// 
    /// Returns a list of whitelisted IP addresses
    fn whitelisted_keys(&self) -> Vec<Self::Key> {
        let whitelisted = get_env_or("WHITELIST", "127.0.0.1").parse::<IpAddr>().unwrap();
        vec![whitelisted]
    }

    /// Returns the name of the key
    /// 
    /// ## Parameters
    /// 
    /// * `key` is the key
    /// 
    /// ## Returns
    /// 
    /// Returns the name of the key
    fn key_name(&self, key: &Self::Key) -> Option<String> {
        Some(key.to_string())
    }
}
