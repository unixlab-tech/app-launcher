use shellexpand::tilde;
use std::env;

pub fn ensure_env_var(key: &str, default_value: &str) -> String {
    match env::var(key) {
        Ok(value) => tilde(&value).to_string(),
        Err(_) => {
            env::set_var(key, default_value);
            tilde(default_value).to_string()
        }
    }
}
