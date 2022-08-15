use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetProcessesResponse {
    pub processes: Vec<ProcessInfo>,
}

/// Information about a running process.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    /// The unique ID for the process.
    pub pid: String,
    /// The unique ID of the parent process, if any.
    pub parent_pid: Option<String>,
    /// The name of the process.
    pub name: String,
    /// The CPU usage of the process.
    pub cpu: f32,
    /// The amount of memory in use by the process.
    pub memory: u64,
    /// The amount of virtual memory allocated for the process.
    pub virtual_memory: u64,
    /// The number of seconds the process has been executing for.
    pub run_time: u64,
    /// The path to the process's executable, if available.
    pub path: Option<PathBuf>,
}
