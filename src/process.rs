use std::{fs, path::Path};

pub fn get_all_running_proces() -> Result<Vec<String>, std::io::Error> {
    let t = Path::new("/proc");
    let all_process_folder = t.read_dir().expect("failed to read /proc");

    let process_names: Result<Vec<String>, std::io::Error> = all_process_folder
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.unwrap();

            let comm_file_path = entry.path().join("comm");
            if entry.path().is_dir() && comm_file_path.exists() {
                Some(entry)
            } else {
                None
            }
        })
        .map(|entry| {
            let comm_file_path = entry.path().join("comm");

            let process_name = fs::read_to_string(comm_file_path);

            match process_name {
                Ok(name) => Ok(name.trim().to_owned()),
                Err(err) => Err(err),
            }
        })
        .collect();

    process_names
}
