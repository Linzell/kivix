use std::{collections::HashMap, env};

/// Returns HashMap of Enviroment variables
/// 
/// ## Returns
/// 
/// * `HashMap<String, String>` - The enviroment variables
/// 
/// ## Panics
/// Panics if the enviroment variables cannot be read
fn get_envv() -> HashMap<String, String> {
    env::vars().map(|(key, value)| (key, value)).collect()
}

/// Returns either the value of an enviroment variable or the default provided
/// 
/// ## Arguments
/// 
/// * `key` - The key of the enviroment variable
/// * `default` - The default value
/// 
/// ## Returns
/// 
/// * `String` - The value of the enviroment variable or the default
/// 
/// ## Panics
/// Panics if the variable does not exist
pub fn get_env_or(key: &str, default: &str) -> String {
    let envv = get_envv();
    if envv.contains_key(key) {
        // Safety: The HashMap is already checked for the key
        envv.get(key).unwrap().clone()
    } else {
        default.to_string()
    }
}

/// Returns the value of an enviroment variable
/// 
/// ## Arguments
/// 
/// * `key` - The key of the enviroment variable
/// 
/// ## Returns
/// 
/// * `String` - The value of the enviroment variable
///
/// ## Panics
/// Panics if the variable does not exist
pub fn get_env(key: &str) -> String {
    let envv = get_envv();
    if envv.contains_key(key) {
        envv.get(key).unwrap().clone()
    } else {
        panic!("Enviroment variable {0} is not set", key)
    }
}
