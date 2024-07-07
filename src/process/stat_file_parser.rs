use super::process_info::ProcessInfo;

fn remove_parentheses_from_comm(comm: &str) -> &str {
    &comm[1..(comm.len() - 1)]
}

pub fn parse_stat_file(
    content: &str,
    seconds_since_boot: u64,
    ticks_per_second: u64,
) -> ProcessInfo {
    let parts: Vec<&str> = content.split(" ").collect();

    let pid = parts[0].parse::<u64>().unwrap();
    let comm_name_with_parentheses = parts[1];
    let clean_comm_name = remove_parentheses_from_comm(comm_name_with_parentheses);

    let process_start_clock_ticks_after_boot = parts[21].parse::<u64>().unwrap();
    let process_start_seconds_after_boot = process_start_clock_ticks_after_boot / ticks_per_second;
    let running_for = seconds_since_boot - process_start_seconds_after_boot;

    ProcessInfo {
        pid,
        name: clean_comm_name.to_owned(),
        been_running_for_seconds: running_for,
    }
}
