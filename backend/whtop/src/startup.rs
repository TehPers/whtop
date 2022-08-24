use crate::config::AppConfig;
use anyhow::Context;
use axum::{body::HttpBody, Extension, Router};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{trace::TraceLayer, compression::CompressionLayer};
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
    let app = build_app(config.clone()).await?;
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

async fn build_app<B>(config: Arc<AppConfig>) -> anyhow::Result<Router<B>>
where
    B: HttpBody + Send + 'static,
{
    // Backend API
    let api_router = Router::new().nest("/system", crate::modules::system(config.clone()));

    // Frontend
    let frontend_router = crate::modules::frontend(config.clone());

    // Global layers
    let router = Router::new()
        .nest("/api", api_router)
        .merge(frontend_router)
        .layer(
            ServiceBuilder::new()
                .layer(CompressionLayer::new())
                .layer(TraceLayer::new_for_http())
                .layer(Extension(config)),
        );

    Ok(router)
}
