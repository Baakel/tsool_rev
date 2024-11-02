use std::sync::LazyLock;
use std::{fs, path::PathBuf, str::FromStr};

use serde::Deserialize;

pub static CONFIG: LazyLock<Config> = LazyLock::new(init_configs);

pub fn init_configs() -> Config {
    let config_file_str = format!("{}/tsool/config.toml", get_config_dir());
    let config_path = PathBuf::from_str(&config_file_str).unwrap();
    if !config_path.exists() {
        if !config_path.parent().unwrap().exists() {
            fs::create_dir(config_path.parent().unwrap()).unwrap();
        }
        let db_url = std::env::var("DATABASE_URL").unwrap();
        return Config {
            database_url: db_url,
        };
    }
    let config_file = fs::read_to_string(config_path).unwrap();
    toml::from_str::<Config>(&config_file).unwrap()
}

fn get_config_dir() -> String {
    match std::env::var("XDG_CONFIG_HOME") {
        Err(_) => format!("{}/.config", std::env::var("HOME").unwrap()),
        Ok(dir) => dir,
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
}
