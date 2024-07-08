use std::{env, fs::File, io::BufReader, path::Path};

use super::config_option::{ConfigFileError, ProcessRules};

pub fn read_config_file() -> Result<ProcessRules, ConfigFileError> {
    let key = "HOME";
    let home_path = match env::var_os(key) {
        Some(val) => Ok(val),
        None => Err(ConfigFileError {}),
    }?;

    let config_file_path =
        Path::new(&home_path).join(".config/limit-process-time-cron/config.json");

    let file = File::open(config_file_path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let rules: ProcessRules = serde_json::from_reader(reader)?;

    Ok(rules)
}
