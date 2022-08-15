use futures::Future;
use pin_project::pin_project;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tower::{Layer, Service};

type JsTimeoutFuture = gloo::timers::future::TimeoutFuture;

#[derive(Clone, Debug)]
pub struct TimeoutLayer {
    timeout: Duration,
}

impl TimeoutLayer {
    pub fn new(timeout: Duration) -> Self {
        TimeoutLayer { timeout }
    }
}

impl<S> Layer<S> for TimeoutLayer {
    type Service = Timeout<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Timeout {
            timeout: self.timeout,
            inner,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Timeout<S> {
    timeout: Duration,
    inner: S,
}

impl<Req, S> Service<Req> for Timeout<S>
where
    S: Service<Req>,
    S::Error: From<TimedOut>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = TimeoutFuture<S::Response, S::Error, S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        let future = self.inner.call(req);
        let timeout = gloo::timers::future::sleep(self.timeout);
        return TimeoutFuture {
            timeout,
            future,
            _marker: Default::default(),
        };
    }
}

#[derive(Debug)]
#[pin_project]
pub struct TimeoutFuture<R, E, F> {
    #[pin]
    future: F,
    #[pin]
    timeout: JsTimeoutFuture,
    _marker: PhantomData<fn(F) -> Result<R, E>>,
}

impl<R, E, F> Future for TimeoutFuture<R, E, F>
where
    F: Future<Output = Result<R, E>>,
    E: From<TimedOut>,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let projected = self.project();

        if let Poll::Ready(result) = projected.future.poll(cx) {
            return Poll::Ready(result);
        }

        if let Poll::Ready(_) = projected.timeout.poll(cx) {
            return Poll::Ready(Err(TimedOut::default().into()));
        }

        Poll::Pending
    }
}

#[derive(Debug, Default)]
pub struct TimedOut;

impl Display for TimedOut {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "request timed out")
    }
}

impl Error for TimedOut {}
