use anyhow::Context;
use axum::{body::HttpBody, Router, Server};
use tower::ServiceBuilder;
use tower_http::{
    compression::{predicate::SizeAbove, CompressionLayer},
    trace::TraceLayer,
};
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::config::AppConfig;

const DEFAULT_ENV_FILTER: &str = "info";

pub async fn start() -> anyhow::Result<()> {
    // Setup tracing
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| DEFAULT_ENV_FILTER.into()))
        .with(tracing_subscriber::fmt::layer().compact())
        .try_init()?;

    // Load config
    let config = load_config()?;
    debug!(?config, "config loaded");

    // Create app
    let app = build_app(&config).await?;
    info!("listening on {}", config.address);
    Server::try_bind(&config.address)
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

async fn build_app<B>(config: &AppConfig) -> anyhow::Result<Router<(), B>>
where
    B: HttpBody + Send + 'static,
{
    // Backend API
    let api_router = Router::new().nest("/system", crate::modules::system(config));

    // Frontend
    let frontend_router = crate::modules::frontend(config);

    // Global layers
    let router = Router::new()
        .nest("/api", api_router)
        .merge(frontend_router)
        .layer(
            ServiceBuilder::new()
                .layer(CompressionLayer::new().compress_when(SizeAbove::new(1000)))
                .layer(TraceLayer::new_for_http()),
        );

    Ok(router)
}
