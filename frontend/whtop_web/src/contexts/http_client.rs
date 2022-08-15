use crate::layers::TimeoutLayer;
use anyhow::Context as _;
use futures::{future::LocalBoxFuture, lock::Mutex, FutureExt};
use gloo::net::http::Request;
use std::{
    rc::Rc,
    task::{Context, Poll},
    time::Duration,
};
use tower::{util::UnsyncBoxService, Service, ServiceBuilder, ServiceExt};
use web_sys::AbortController;

#[derive(Clone)]
pub struct HttpClient {
    service: Rc<Mutex<UnsyncBoxService<Request, Response, anyhow::Error>>>,
}

impl HttpClient {
    pub fn new(options: HttpClientOptions) -> Self {
        let service = ServiceBuilder::new()
            .check_service::<RequestService, Request, Response, anyhow::Error>()
            .layer(TimeoutLayer::new(options.timeout))
            .concurrency_limit(options.concurrency_limit)
            .check_service::<RequestService, Request, Response, anyhow::Error>()
            .service(RequestService::default());

        HttpClient {
            service: Rc::new(Mutex::new(UnsyncBoxService::new(service))),
        }
    }

    pub async fn send(&self, request: Request) -> anyhow::Result<Response> {
        let mut service = self.service.lock().await;
        service
            .ready()
            .await
            .context("an error occurred while waiting to send the request")?;
        let fut = service.call(request);
        drop(service);
        let response = fut
            .await
            .context("an error occurred while sending the request")?;
        Ok(response)
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        HttpClient::new(Default::default())
    }
}

impl PartialEq for HttpClient {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.service, &other.service)
    }
}

#[derive(Clone, Debug)]
pub struct HttpClientOptions {
    timeout: Duration,
    concurrency_limit: usize,
}

impl Default for HttpClientOptions {
    fn default() -> Self {
        HttpClientOptions {
            timeout: Duration::from_secs(3),
            concurrency_limit: 5,
        }
    }
}

#[derive(Debug, Default)]
struct RequestService;

impl Service<Request> for RequestService {
    type Response = Response;
    type Error = anyhow::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        // Cancel request on drop, if able
        let abort_controller = AbortController::new().ok();
        let abort_signal = abort_controller
            .as_ref()
            .map(|controller| controller.signal());
        let req = req.abort_signal(abort_signal.as_ref());
        let abort = abort_controller.map(AutoAbort);

        // Send request
        async move {
            let response = req.send().await.context("error sending request")?;
            Ok(Response {
                response,
                _abort: abort,
            })
        }
        .boxed_local()
    }
}

#[derive(Debug)]
struct AutoAbort(AbortController);

impl Drop for AutoAbort {
    fn drop(&mut self) {
        self.0.abort();
    }
}

#[derive(Debug)]
pub struct Response {
    response: gloo::net::http::Response,
    _abort: Option<AutoAbort>,
}

impl Response {
    pub fn inner(&self) -> &gloo::net::http::Response {
        &self.response
    }
}
