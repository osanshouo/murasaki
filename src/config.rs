use std::fs;
use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub site_name: String,
    pub language: String,
    pub root: String,
    pub author: String,
    pub year: String,
}

impl Config {
    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        let summary = fs::read_to_string("./Summary.toml")?;
        Ok(toml::from_str(&summary)?)
    }
}
