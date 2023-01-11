use axum::{body::HttpBody, extract::State, response::IntoResponse, routing::MethodRouter, Json};
use sysinfo::{Cpu, CpuExt, SystemExt};
use whtop_common::models::api::{CpuInfo, GetCpuResponse, GlobalCpuInfo};

use crate::routes::RouteResult;

use super::SystemState;

pub fn cpu<B>() -> MethodRouter<SystemState, B>
where
    B: HttpBody + Send + 'static,
{
    MethodRouter::new().get(get_cpu)
}

async fn get_cpu(State(state): State<SystemState>) -> RouteResult<impl IntoResponse> {
    let system = state.system.read().await;
    let global = create_global_cpu_info(system.global_cpu_info());
    let cpus = system.cpus().iter().map(create_cpu_info).collect();
    let response = GetCpuResponse { global, cpus };
    Ok(Json(response))
}

fn create_cpu_info(cpu: &Cpu) -> CpuInfo {
    CpuInfo {
        name: cpu.name().into(),
        inner: create_global_cpu_info(cpu),
    }
}

fn create_global_cpu_info(cpu: &Cpu) -> GlobalCpuInfo {
    GlobalCpuInfo {
        usage: cpu.cpu_usage(),
        frequency: cpu.frequency(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Bytes, http::StatusCode, response::IntoResponse};
    use std::sync::Arc;
    use sysinfo::System;
    use tokio::sync::RwLock;
    use whtop_common::models::api::GetCpuResponse;

    async fn body_to_bytes<B>(mut body: B) -> Vec<u8>
    where
        B: HttpBody<Data = Bytes> + Unpin,
    {
        let mut result = Vec::with_capacity(body.size_hint().lower().max(2048) as usize);
        while let Some(chunk) = body.data().await {
            let Ok(chunk) = chunk else {
                unreachable!("error reading chunk from body")
            };

            result.extend_from_slice(&chunk);
        }

        result
    }

    #[tokio::test]
    async fn test_get_cpu() {
        // Setup
        let mut system = System::new_all();
        system.refresh_all();
        let system = Arc::new(RwLock::new(system));

        // Execute
        let response = get_cpu(State(SystemState { system })).await.unwrap();

        // Assert
        let (parts, body) = response.into_response().into_parts();
        let body = body_to_bytes(body).await;
        assert_eq!(parts.status, StatusCode::OK);
        let body: GetCpuResponse = serde_json::from_slice(&body).unwrap();
        assert!(body.cpus.len() > 0);
    }
}
