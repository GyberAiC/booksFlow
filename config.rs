use toml;
use std::fs;

pub struct Config {
    pub api_key: String,
    pub authors: Vec<String>,
}

impl Config {
    pub fn from_file(filename: &str) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(filename)?;
        let config: Config = toml::from_str(&content).unwrap();
        Ok(config)
    }
}
