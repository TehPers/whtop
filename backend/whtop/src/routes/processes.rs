use axum::{
    body::HttpBody,
    response::IntoResponse,
    routing::{get, MethodRouter},
    Extension, Json,
};
use std::{cmp::Reverse, path::PathBuf, sync::Arc};
use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};
use tokio::sync::RwLock;
use whtop_common::models::api::{GetProcessesResponse, ProcessInfo};

pub fn processes<B>() -> MethodRouter<B>
where
    B: HttpBody + Send + 'static,
{
    get(get_processes)
}

async fn get_processes(Extension(system): Extension<Arc<RwLock<System>>>) -> impl IntoResponse {
    let system = system.read().await;
    let response: GetProcessesResponse = create_response(&system);
    Json(response)
}

fn create_response(system: &System) -> GetProcessesResponse {
    let mut processes: Vec<ProcessInfo> =
        system.processes().iter().map(create_process_info).collect();
    processes.sort_unstable_by_key(|process| Reverse(process.memory));
    GetProcessesResponse { processes }
}

fn create_process_info((pid, process): (&Pid, &Process)) -> ProcessInfo {
    ProcessInfo {
        pid: pid.to_string(),
        parent_pid: process.parent().map(|pid| pid.to_string()),
        name: process.name().into(),
        cpu: process.cpu_usage(),
        memory: process.memory(),
        virtual_memory: process.virtual_memory(),
        run_time: process.run_time(),
        path: {
            let path = process.exe();
            if path == PathBuf::default() {
                None
            } else {
                Some(path.to_owned())
            }
        },
    }
}
