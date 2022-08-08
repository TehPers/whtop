use crate::{config::AppConfig, layers::RefreshSystemLayer};
use anyhow::Context;
use axum::{body::HttpBody, http::HeaderValue, Extension, Router};
use std::{sync::Arc, time::Duration};
use sysinfo::{System, SystemExt};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{set_header::SetResponseHeaderLayer, trace::TraceLayer};
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

const DEFAULT_ENV_FILTER: &str = "info";

pub async fn start() -> anyhow::Result<()> {
    // Setup tracing
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| DEFAULT_ENV_FILTER.into()))
        .with(tracing_subscriber::fmt::layer().compact())
        .try_init()?;

    // Load config
    let config = Arc::new(load_config()?);
    debug!(?config, "config loaded");

    // Create app;
    let app = build_app(config.clone());
    info!("listening on {}", config.address);
    axum::Server::try_bind(&config.address)
        .context("error binding to address")?
        .serve(app.into_make_service())
        .await
        .context("error running server")
}

fn load_config() -> anyhow::Result<AppConfig> {
    envy::prefixed("WHTOP_")
        .from_env()
        .context("error reading config")
}

fn build_app<B>(config: Arc<AppConfig>) -> Router<B>
where
    B: HttpBody + Send + 'static,
{
    // Create system info tracker
    let system = Arc::new(RwLock::new(System::new_all()));
    Router::new()
        .route("/cpu", crate::routes::cpu())
        .route("/memory", crate::routes::memory())
        .route("/processes", crate::routes::processes())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(RefreshSystemLayer::new(
                    system.clone(),
                    Duration::from_secs_f32(config.refresh_rate_secs),
                ))
                .layer(SetResponseHeaderLayer::if_not_present(
                    hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    HeaderValue::from_static("*"),
                ))
                .layer(Extension(system))
                .layer(Extension(config)),
        )
}
