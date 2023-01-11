use axum::{body::HttpBody, extract::State, response::IntoResponse, routing::MethodRouter, Json};
use sysinfo::SystemExt;
use whtop_common::models::api::GetMemoryResponse;

use crate::routes::RouteResult;

use super::SystemState;

pub fn memory<B>() -> MethodRouter<SystemState, B>
where
    B: HttpBody + Send + 'static,
{
    MethodRouter::new().get(get_memory)
}

async fn get_memory(State(state): State<SystemState>) -> RouteResult<impl IntoResponse> {
    let system = state.system.read().await;
    let response = GetMemoryResponse {
        total: system.total_memory(),
        used: system.used_memory(),
        free: system.free_memory(),
        available: system.available_memory(),
    };
    Ok(Json(response))
}
