use std::{error::Error, fmt};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProcessRules {
    pub processes: Vec<ConfigOption>,
}

#[derive(Deserialize)]
pub struct ConfigOption {
    pub process_name: String,
    pub allowed_life_duration_seconds: u64,
}

#[derive(Debug)]
pub struct ConfigFileError {}

impl fmt::Display for ConfigFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to read config file")
    }
}

impl Error for ConfigFileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl From<serde_json::Error> for ConfigFileError {
    fn from(e: serde_json::Error) -> Self {
        // I will do better later
        println!("{e:#?}");
        ConfigFileError {}
    }
}

impl From<std::io::Error> for ConfigFileError {
    fn from(e: std::io::Error) -> Self {
        println!("{e:#?}");
        ConfigFileError {}
    }
}
