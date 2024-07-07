use libc::{sysconf, _SC_CLK_TCK};
use std::fs::File;
use std::io::Read;

use super::process_info::ProcessInfo;

fn to_u64(v: &[u8]) -> u64 {
    let mut x = 0;

    for c in v {
        x *= 10;
        x += u64::from(c - b'0');
    }
    x
}

fn boot_time() -> u64 {
    if let Ok(buf) = File::open("/proc/stat").and_then(|mut f| {
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        Ok(buf)
    }) {
        let line = buf.split(|c| *c == b'\n').find(|l| l.starts_with(b"btime"));

        if let Some(line) = line {
            return line
                .split(|x| *x == b' ')
                .filter(|s| !s.is_empty())
                .nth(1)
                .map(to_u64)
                .unwrap_or(0);
        }
    }
    // Either we didn't find "btime" or "/proc/stat" wasn't available for some reason...
    unsafe {
        let mut up: libc::timespec = std::mem::zeroed();
        if libc::clock_gettime(libc::CLOCK_BOOTTIME, &mut up) == 0 {
            up.tv_sec as u64
        } else {
            println!("clock_gettime failed: boot time cannot be retrieve...");
            0
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

    ProcessInfo {
        name: clean_comm_name.to_owned(),
        been_running_for_seconds: 0,
    }
}
