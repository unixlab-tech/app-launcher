use std::{
    fs::{create_dir_all, metadata, File},
    io::Write,
};

use super::utils::ensure_env_var;

const LAUNCHER_CONFIG_VAR: &str = "LAUNCHER_CONFIG_PATH";
const LAUNCHER_CONFIG_PATH: &str = "$HOME/.config/launcher";
const LAUNCHER_STYLE_VAR: &str = "LAUNCHER_STYLE_PATH";
const LAUNCHER_STYLE_PATH: &str = "$LAUNCHER_CONFIG_PATH/styles/default.css";
const DEFAULT_STYLES: &str = "* { margin: 0; padding: 0; }";

pub fn ensure_configs() -> String {
    let dir_path = ensure_env_var(LAUNCHER_CONFIG_VAR, LAUNCHER_CONFIG_PATH);

    if !metadata(&dir_path)
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
    {
        create_dir_all(&dir_path).expect("An error has been ocurred while trying creating folders");
    }

    dir_path
}

pub fn ensure_css_styles() -> String {
    let file_path = ensure_env_var(LAUNCHER_STYLE_VAR, LAUNCHER_STYLE_PATH);

    if !File::open(&file_path).is_ok() {
        let mut file =
            File::create(&file_path).expect("An error has been ocurred while trying create a file");

        file.write_all(DEFAULT_STYLES.as_bytes())
            .expect("An error has been ocurred while trying write in file");
    }

    file_path
}
