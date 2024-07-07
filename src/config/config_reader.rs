use std::io::Error;

use super::config_option::ConfigOption;

pub fn read_config_file() -> Result<Vec<ConfigOption>, Error> {
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "config file not found",
    ))
}
