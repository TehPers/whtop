use crate::{config::AppConfig, layers::RefreshSystemLayer};
use axum::{body::HttpBody, Extension, Router};
use axum_extra::routing::SpaRouter;
use std::{sync::Arc, time::Duration};
use sysinfo::{CpuRefreshKind, ProcessRefreshKind, RefreshKind, System, SystemExt};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

pub fn frontend<B>(config: Arc<AppConfig>) -> Router<B>
where
    B: HttpBody + Send + 'static,
{
    if config.serve_static {
        SpaRouter::new("/assets", &config.static_dir).into()
    } else {
        Router::default()
    }
}

pub fn system<B>(config: Arc<AppConfig>) -> Router<B>
where
    B: HttpBody + Send + 'static,
{
    // Create system info tracker
    let system = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::new().with_cpu_usage().with_frequency())
            .with_memory()
            .with_processes(ProcessRefreshKind::new().with_cpu()),
    );
    let system = Arc::new(RwLock::new(system));
    Router::new()
        .route("/cpu", crate::routes::api::system::cpu())
        .route("/memory", crate::routes::api::system::memory())
        .route("/processes", crate::routes::api::system::processes())
        .layer(
            ServiceBuilder::new()
                .layer(RefreshSystemLayer::new(
                    system.clone(),
                    Duration::from_secs_f32(config.refresh_rate_secs),
                ))
                .layer(CorsLayer::new().allow_origin(Any))
                .layer(Extension(system)),
        )
}
