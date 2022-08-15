use futures::{future::BoxFuture, FutureExt};
use hyper::Request;
use std::{
    sync::Arc,
    task::{Context, Poll},
    time::{Duration, Instant},
};
use sysinfo::{System, SystemExt};
use tokio::sync::{Mutex, RwLock};
use tower::{Layer, Service};
use tracing::debug;

#[derive(Clone, Debug)]
pub struct RefreshSystemLayer {
    system: Arc<RwLock<System>>,
    refresh_rate: Duration,
    last_refresh: Arc<Mutex<Option<Instant>>>,
}

impl RefreshSystemLayer {
    pub fn new(system: Arc<RwLock<System>>, refresh_rate: Duration) -> Self {
        RefreshSystemLayer {
            system,
            refresh_rate,
            last_refresh: Default::default(),
        }
    }
}

impl<S> Layer<S> for RefreshSystemLayer {
    type Service = RefreshSystemService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RefreshSystemService {
            inner,
            system: self.system.clone(),
            refresh_rate: self.refresh_rate,
            last_refresh: self.last_refresh.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RefreshSystemService<S> {
    inner: S,
    system: Arc<RwLock<System>>,
    refresh_rate: Duration,
    last_refresh: Arc<Mutex<Option<Instant>>>,
}

impl<ReqBody, S> Service<Request<ReqBody>> for RefreshSystemService<S>
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
            let mut last_refresh = last_refresh.lock().await;
            let should_refresh = match *last_refresh {
                None => true,
                Some(last_refresh) => last_refresh.elapsed() > refresh_rate,
            };
            if should_refresh {
                debug!(?last_refresh, "refreshing system");
                *last_refresh = Some(Instant::now());
                system.write().await.refresh_all();
            }

            // Execute next service
            inner.await
        }
        .boxed()
    }
}
