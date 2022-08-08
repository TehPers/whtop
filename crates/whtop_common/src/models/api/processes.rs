use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize)]
pub struct GetProcessesResponse {
    pub processes: Vec<ProcessInfo>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ProcessInfo {
    pub pid: String,
    pub parent_pid: Option<String>,
    pub name: String,
    pub cpu: f32,
    pub memory: u64,
    pub virtual_memory: u64,
    pub run_time: u64,
    pub path: Option<PathBuf>,
}
