use std::cmp::Reverse;

use axum::{body::HttpBody, extract::State, response::IntoResponse, routing::MethodRouter, Json};
use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};
use whtop_common::models::api::{GetProcessesResponse, ProcessInfo};

use crate::routes::RouteResult;

use super::SystemState;

pub fn processes<B>() -> MethodRouter<SystemState, B>
where
    B: HttpBody + Send + 'static,
{
    MethodRouter::new().get(get_processes)
}

async fn get_processes(State(state): State<SystemState>) -> RouteResult<impl IntoResponse> {
    let system = state.system.read().await;
    let response: GetProcessesResponse = create_response(&system);
    Ok(Json(response))
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
    }
}
