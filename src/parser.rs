use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default, flatten)]
    pub globals: HashMap<String, toml::Value>,
    #[serde(default)]
    pub sections: HashMap<String, HashMap<String, toml::Value>>,
}

impl TryFrom<&'static str> for Config {
    type Error = toml::de::Error;

    fn try_from(raw: &'static str) -> Result<Self, Self::Error> {
        toml::from_str(raw)
    }
}

impl Config {
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> crate::error::Result<Self> {
        let mut buffer = Vec::new();

        let mut file = std::fs::File::open(path)?;
        file.read_to_end(&mut buffer)?;

        Ok(toml::from_slice(&buffer)?)
    }
}
