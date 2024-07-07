#![warn(clippy::all, clippy::precedence)]

mod config;
mod process;

use crate::config::config_option::ConfigOption;
use crate::config::config_reader::read_config_file;
use crate::process::process_probe::get_all_running_processes;

fn main() {
    let config = match read_config_file() {
        Ok(conf) => conf,
        _ => vec![ConfigOption {
            allowed_life_duration_seconds: 3600,
            process_name: "hoi4".to_owned(),
        }],
    };

    let processes = get_all_running_processes().expect("Failed to list running processes");

    let interesting_processes: Vec<_> = processes
        .iter()
        .filter(|p| {
            let search_result = config.binary_search_by(|o| o.process_name.cmp(&p.name));

            match search_result {
                Ok(_) => true,
                _ => false,
            }
        })
        .collect();

    println!("{interesting_processes:#?}");
}
