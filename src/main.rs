// main.rs
#![allow(clippy::enum_variant_names)]
mod error;
#[cfg(feature = "proxy")]
mod governor;
mod prelude;
mod repository;
mod models;
mod utils;

use std::{
    collections::HashMap,
    env, fs,
    io::{self, BufReader},
};

use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_identity::{config::LogoutBehaviour, IdentityMiddleware};
use actix_session::{config::PersistentSession, SessionMiddleware};
use actix_session_surrealdb::SurrealSessionStore;
use actix_web::{
    cookie::{time::Duration, Key},
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use dotenv::dotenv;
use log::info;

use repository::surrealdb_repo::SurrealDBRepo;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

use crate::utils::env::get_env_or;

#[cfg(feature = "proxy")]
use crate::governor::NginxIpKeyExctrator;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let envv: HashMap<String, String> = env::vars().map(|(key, value)| (key, value)).collect();
    if cfg!(debug_assertions) {
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    } else {
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    }

    // let config = load_rustls_config();

    let surreal = match SurrealDBRepo::init().await {
        Ok(surreal) => {
            info!("✅ Connection to the database is successful!");
            surreal
        }
        Err(e) => {
            info!("🔥 Failed to connect to the database: {:?}", e);
            std::process::exit(1);
        }
    };

    let cookie_key = if envv.contains_key("COOKIE_KEY") {
        Key::from(envv.get("COOKIE_KEY").unwrap().as_bytes())
    } else {
        Key::generate()
    };

    let port = get_env_or("PORT", "8080");

    info!("🚀 Starting server on port {}", port);

    #[cfg(feature = "proxy")]
    let reverse_proxy = get_env("REVERSE_PROXY").parse::<IpAddr>().unwrap();

    HttpServer::new(move || {
        let logger = Logger::default();
        let json_config = web::JsonConfig::default()
            .limit(65536) // 64 KiB
            .error_handler(|err, _req| {
                actix_web::error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().finish(),
                )
                .into()
            });

        let cors = Cors::default()
            // .allowed_origin(if cfg!(debug_assertions) {
            //     "http://localhost:3000"
            // } else {
            //     "https://test.com"
            // })
            .allowed_origin("http://localhost:3000")
            .supports_credentials()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        #[cfg(feature = "proxy")]
        let governor_config = GovernorConfigBuilder::default()
            .key_extractor(NginxIpKeyExctrator)
            .per_second(10)
            .burst_size(2)
            .use_headers()
            .finish()
            .unwrap();

        #[cfg(not(feature = "proxy"))]
        let governor_config = GovernorConfigBuilder::default()
            .per_second(10)
            .burst_size(2)
            .use_headers()
            .finish()
            .unwrap();

        #[allow(clippy::let_and_return)]
        let app = App::new()
            .wrap(Governor::new(&governor_config))
            .wrap(
                IdentityMiddleware::builder()
                    .logout_behaviour(LogoutBehaviour::PurgeSession)
                    .build(),
            )
            .wrap(logger)
            .wrap(
                SessionMiddleware::builder(
                    SurrealSessionStore::from_connection(surreal.session_db.clone(), "sessions"),
                    cookie_key.clone(),
                )
                .cookie_same_site(actix_web::cookie::SameSite::None)
                .cookie_secure(true)
                .cookie_http_only(true)
                .session_lifecycle(
                    PersistentSession::default()
                        .session_ttl_extension_policy(
                            actix_session::config::TtlExtensionPolicy::OnStateChanges,
                        )
                        .session_ttl(Duration::days(7)),
                )
                .build(),
            )
            .wrap(cors)
            .app_data(json_config)
            .app_data(Data::new(surreal.db.clone()));
        #[cfg(feature = "proxy")]
        let app = app.app_data(Data::new(reverse_proxy));

        app
    })
    .bind(format!("0.0.0.0:{port}"))?
    // .bind_rustls(format!("0.0.0.0:{port}"), config)?
    .run()
    .await
}

fn load_rustls_config() -> rustls::ServerConfig {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let cert_file = &mut BufReader::new(fs::File::open("cert.pem").expect("cert.pem to load"));
    let key_file = &mut BufReader::new(fs::File::open("key.pem").expect("key.pem to load"));

    let cert_chain = certs(cert_file)
        .expect("certificate to load")
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .expect("key to load")
        .into_iter()
        .map(PrivateKey)
        .collect();

    if keys.is_empty() {
        panic!("Could not locate private keys");
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
