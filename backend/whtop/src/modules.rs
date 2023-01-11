use crate::{
    config::AppConfig,
    layers::{CacheControlLayer, CacheOptions, LastModifiedLayer, RefreshSystemLayer},
    routes::api::system::SystemState,
};
use axum::{body::HttpBody, Router};
use axum_extra::routing::SpaRouter;
use chrono::{Duration, Local};
use std::sync::Arc;
use sysinfo::{CpuRefreshKind, ProcessRefreshKind, RefreshKind, System, SystemExt};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

pub fn frontend<B>(config: &AppConfig) -> Router<(), B>
where
    B: HttpBody + Send + 'static,
{
    if config.serve_static {
        SpaRouter::new("/assets", &config.static_dir).into()
    } else {
        Router::default()
    }
}

pub fn system<B>(config: &AppConfig) -> Router<(), B>
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

    // Layers
    let refresh_layer = RefreshSystemLayer::new(
        system.clone(),
        Duration::seconds(config.refresh_rate_secs.floor() as i64)
            + Duration::nanoseconds((config.refresh_rate_secs.fract() * 1e9) as i64),
    );
    let last_refresh = refresh_layer.last_refresh();
    let cors_layer = CorsLayer::new().allow_origin(Any);
    let cache_control_layer = CacheControlLayer::new(CacheOptions {
        max_age: Some(config.refresh_rate_secs.floor() as u64),
        public: true,
        ..Default::default()
    });
    let last_modified_layer = LastModifiedLayer::new(move || {
        let last_refresh = last_refresh.clone();
        async move {
            let guard = last_refresh.read().await;
            guard.unwrap_or_else(Local::now)
        }
    });

    // Build router
    let state = SystemState { system };
    Router::new()
        .route(
            "/cpu",
            crate::routes::api::system::cpu().with_state(state.clone()),
        )
        .route(
            "/memory",
            crate::routes::api::system::memory().with_state(state.clone()),
        )
        .route(
            "/processes",
            crate::routes::api::system::processes().with_state(state),
        )
        .layer(
            ServiceBuilder::new()
                .layer(refresh_layer)
                .layer(cors_layer)
                .layer(cache_control_layer)
                .layer(last_modified_layer),
        )
}
