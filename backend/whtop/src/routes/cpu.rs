use axum::{body::HttpBody, response::IntoResponse, routing::MethodRouter, Extension, Json};
use std::sync::Arc;
use sysinfo::{Cpu, CpuExt, System, SystemExt};
use tokio::sync::RwLock;
use whtop_common::models::api::{CpuInfo, GetCpuResponse, GlobalCpuInfo};

pub fn cpu<B>() -> MethodRouter<B>
where
    B: HttpBody + Send + 'static,
{
    MethodRouter::new().get(get_cpu)
}

async fn get_cpu(Extension(system): Extension<Arc<RwLock<System>>>) -> impl IntoResponse {
    let system = system.read().await;
    let global = create_global_cpu_info(system.global_cpu_info());
    let cpus = system.cpus().iter().map(create_cpu_info).collect();
    let response = GetCpuResponse { global, cpus };
    Json(response)
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
