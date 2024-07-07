#![warn(clippy::all, clippy::precedence)]

mod config;
mod process;
mod process_killer;

use libc::{kill, SIGTERM};

use crate::config::config_option::ConfigOption;
use crate::config::config_reader::read_config_file;
use crate::process::process_probe::get_all_running_processes;

fn main() {
    let config = match read_config_file() {
        Ok(conf) => conf,
        _ => vec![ConfigOption {
            allowed_life_duration_seconds: 1,
            process_name: "hoi4".to_owned(),
        }],
    };

    let processes = get_all_running_processes().expect("Failed to list running processes");
    let processes_to_kill = process_killer::get_process_ids_to_kill(config, processes);

    println!("{processes_to_kill:#?}");
    for p in processes_to_kill {
        unsafe {
            kill(p.pid, SIGTERM);
        }
    }
}
