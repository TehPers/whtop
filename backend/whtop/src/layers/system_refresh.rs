use axum::http::Request;
use chrono::{DateTime, Duration, Local};
use futures::{future::BoxFuture, FutureExt};
use std::{
    sync::Arc,
    task::{Context, Poll},
};
use sysinfo::{System, SystemExt};
use tokio::sync::RwLock;
use tower::{Layer, Service};
use tracing::debug;

#[derive(Clone, Debug)]
pub struct RefreshSystemLayer {
    system: Arc<RwLock<System>>,
    refresh_rate: Duration,
    last_refresh: Arc<RwLock<Option<DateTime<Local>>>>,
}

impl RefreshSystemLayer {
    pub fn new(system: Arc<RwLock<System>>, refresh_rate: Duration) -> Self {
        RefreshSystemLayer {
            system,
            refresh_rate,
            last_refresh: Default::default(),
        }
    }

    pub fn last_refresh(&self) -> Arc<RwLock<Option<DateTime<Local>>>> {
        self.last_refresh.clone()
    }
}

impl<S> Layer<S> for RefreshSystemLayer {
    type Service = RefreshSystem<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RefreshSystem {
            inner,
            system: self.system.clone(),
            refresh_rate: self.refresh_rate,
            last_refresh: self.last_refresh.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RefreshSystem<S> {
    inner: S,
    system: Arc<RwLock<System>>,
    refresh_rate: Duration,
    last_refresh: Arc<RwLock<Option<DateTime<Local>>>>,
}

impl<ReqBody, S> Service<Request<ReqBody>> for RefreshSystem<S>
where
    S: Service<Request<ReqBody>>,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let system = self.system.clone();
        let refresh_rate = self.refresh_rate;
        let last_refresh = self.last_refresh.clone();
        let inner = self.inner.call(req);
        async move {
            // Update system if needed
            let needs_refresh = {
                let guard = last_refresh.read().await;
                should_refresh(*guard, refresh_rate).is_some()
            };
            if needs_refresh {
                let mut guard = last_refresh.write().await;

                // Check again because the lock was re-acquired
                if let Some(now) = should_refresh(*guard, refresh_rate) {
                    debug!(?last_refresh, "refreshing system");
                    *guard = Some(now);
                    system.write().await.refresh_all();
                }
            }

            // Execute next service
            inner.await
        }
        .boxed()
    }
}

fn should_refresh(
    last_refresh: Option<DateTime<Local>>,
    refresh_rate: Duration,
) -> Option<DateTime<Local>> {
    let now = Local::now();
    match last_refresh {
        None => Some(now),
        Some(last_refresh) if now - last_refresh >= refresh_rate => Some(now),
        _ => None,
    }
}
