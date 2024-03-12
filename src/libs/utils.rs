use std::env::set_var;

use shellexpand::{env, tilde};

pub fn expand_env_var(key: &str, default_value: &str) -> String {
    if env(key).is_err() {
        set_var(key, default_value)
    }

    let expanded_path = env(key).unwrap().to_string();
    tilde(&expanded_path).to_string()
}
