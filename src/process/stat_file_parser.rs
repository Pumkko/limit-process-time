use libc::{sysconf, _SC_CLK_TCK};

use super::process_info::ProcessInfo;

fn boot_time() -> u64 {
    unsafe {
        let mut up: libc::timespec = std::mem::zeroed();
        if libc::clock_gettime(libc::CLOCK_BOOTTIME, &mut up) == 0 {
            up.tv_sec as u64
        } else {
            panic!("failed to retrieve boottime");
        }
    }
}

fn remove_parentheses_from_comm(comm: &str) -> &str {
    &comm[1..(comm.len() - 1)]
}

pub fn parse_stat_file(content: &str) -> ProcessInfo {
    let parts: Vec<&str> = content.split(" ").collect();

    let comm_name_with_parentheses = parts[1];
    let clean_comm_name = remove_parentheses_from_comm(comm_name_with_parentheses);

    let clock_cycle: u64 = unsafe { sysconf(_SC_CLK_TCK) as _ };

    let start_time_clock_ticks = parts[21].parse::<u64>().unwrap();

    let start_time_in_seconds = start_time_clock_ticks / clock_cycle;

    let time_since_boot_in_seconds = boot_time();
    let running_for = time_since_boot_in_seconds - start_time_in_seconds;

    ProcessInfo {
        name: clean_comm_name.to_owned(),
        been_running_for_seconds: running_for,
    }
}
