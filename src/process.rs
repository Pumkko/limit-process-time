mod process_info;
mod stat_file_parser;
mod stat_file_parser_test;

use std::{fs, path::Path};

use process_info::ProcessInfo;
use stat_file_parser::parse_stat_file;

pub fn get_all_running_proces() -> Result<Vec<ProcessInfo>, std::io::Error> {
    let t = Path::new("/proc");
    let all_process_folder = t.read_dir().expect("failed to read /proc");

    let process_names: Result<Vec<ProcessInfo>, std::io::Error> = all_process_folder
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.unwrap();

            let stat_file_path = entry.path().join("stat");
            if entry.path().is_dir() && stat_file_path.exists() && stat_file_path.is_file() {
                Some(entry)
            } else {
                None
            }
        })
        .map(|entry| {
            let stat_file_path = entry.path().join("stat");

            let process_name = fs::read_to_string(stat_file_path);

            match process_name {
                Ok(name) => Ok(parse_stat_file(name.trim())),
                Err(err) => Err(err),
            }
        })
        .collect();

    process_names
}
