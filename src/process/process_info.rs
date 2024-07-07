#[derive(Debug)]
pub struct ProcessInfo {
    pub pid: u64,
    pub name: String,
    pub been_running_for_seconds: u64,
}
