#![warn(clippy::all, clippy::precedence)]

mod config;
mod process;
mod process_killer;

use config::config_option::ProcessRules;
use libc::{kill, SIGTERM};

use crate::config::config_option::ConfigOption;
use crate::config::config_reader::read_config_file;
use crate::process::process_probe::get_all_running_processes;

fn main() {
    let config = match read_config_file() {
        Ok(conf) => conf,
        _ => ProcessRules {
            processes: vec![ConfigOption {
                allowed_life_duration_seconds: 3600,
                process_name: "hoi4".to_owned(),
            }],
        },
    };

    let processes = get_all_running_processes().expect("Failed to list running processes");
    let processes_to_kill = process_killer::get_process_ids_to_kill(config.processes, processes);

    println!("Killing process: {processes_to_kill:#?}");
    for p in processes_to_kill {
        unsafe {
            kill(p.pid, SIGTERM);
        }
    }
}
