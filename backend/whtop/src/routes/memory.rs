use axum::{body::HttpBody, response::IntoResponse, routing::MethodRouter, Extension, Json};
use std::sync::Arc;
use sysinfo::{System, SystemExt};
use tokio::sync::RwLock;
use whtop_common::models::api::GetMemoryResponse;

pub fn memory<B>() -> MethodRouter<B>
where
    B: HttpBody + Send + 'static,
{
    MethodRouter::new().get(get_memory)
}

async fn get_memory(Extension(system): Extension<Arc<RwLock<System>>>) -> impl IntoResponse {
    let system = system.read().await;
    let response = GetMemoryResponse {
        total: system.total_memory(),
        used: system.used_memory(),
        free: system.free_memory(),
        available: system.available_memory(),
    };
    Json(response)
}
