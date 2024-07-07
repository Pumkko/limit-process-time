use crate::{config::config_option::ConfigOption, process::process_info::ProcessInfo};

pub fn get_process_ids_to_kill(
    config: Vec<ConfigOption>,
    running_process: Vec<ProcessInfo>,
) -> Vec<ProcessInfo> {
    let mut interesting_processes: Vec<ProcessInfo> = vec![];

    for process in running_process {
        let search_result = config.binary_search_by(|o| o.process_name.cmp(&process.name));

        if let Ok(process_rules_index) = search_result {
            let process_rules = &config[process_rules_index];

            if process.been_running_for_seconds > process_rules.allowed_life_duration_seconds {
                interesting_processes.push(process);
            }
        }
    }

    interesting_processes
}
