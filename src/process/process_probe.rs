use std::{fs, path::Path};

use libc::{sysconf, _SC_CLK_TCK};

use super::{process_info::ProcessInfo, stat_file_parser::parse_stat_file};

fn seconds_since_boot() -> u64 {
    unsafe {
        let mut up: libc::timespec = std::mem::zeroed();
        if libc::clock_gettime(libc::CLOCK_BOOTTIME, &mut up) == 0 {
            up.tv_sec as u64
        } else {
            panic!("failed to retrieve boottime");
        }
    }
}

pub fn get_all_running_processes() -> Result<Vec<ProcessInfo>, std::io::Error> {
    let t = Path::new("/proc");
    let all_process_folder = t.read_dir().expect("failed to read /proc");

    let seconds_since_boot = seconds_since_boot();
    let clock_cycle: u64 = unsafe { sysconf(_SC_CLK_TCK) as _ };

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
                Ok(name) => Ok(parse_stat_file(
                    name.trim(),
                    seconds_since_boot,
                    clock_cycle,
                )),
                Err(err) => Err(err),
            }
        })
        .collect();

    process_names
}
