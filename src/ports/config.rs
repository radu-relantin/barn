use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub cursor_keymaps: CursorKeyMapsConfig,
}
#[derive(Deserialize, Debug)]
pub struct CursorKeyMapsConfig {
    pub down: char,
    pub up: char,
    pub left: char,
    pub right: char,
}

pub fn read_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    toml::from_str(&fs::read_to_string(file_path)?).map_err(Into::into)
}
