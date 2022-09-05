use axum::http::{Request, Response};
use chrono::{DateTime, TimeZone};
use chrono_tz::GMT;
use futures::{future::MaybeDone, ready};
use pin_project_lite::pin_project;
use std::{
    fmt::Display,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct LastModifiedLayer<F> {
    get_last_modified: F,
}

impl<F> LastModifiedLayer<F> {
    pub fn new(get_last_modified: F) -> Self {
        Self { get_last_modified }
    }
}

impl<S, F> Layer<S> for LastModifiedLayer<F>
where
    F: Clone,
{
    type Service = LastModified<S, F>;

    fn layer(&self, inner: S) -> Self::Service {
        LastModified {
            inner,
            get_last_modified: self.get_last_modified.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LastModified<S, F> {
    inner: S,
    get_last_modified: F,
}

impl<S, M, Tz, F, ReqBody, ResBody> Service<Request<ReqBody>> for LastModified<S, F>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
    F: FnMut() -> M,
    M: Future<Output = DateTime<Tz>>,
    Tz: TimeZone,
    Tz::Offset: Display,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = LastModifiedFuture<S::Future, M>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        LastModifiedFuture {
            inner: futures::future::maybe_done(self.inner.call(req)),
            last_modified: (self.get_last_modified)(),
        }
    }
}

pin_project! {
    pub struct LastModifiedFuture<S, M>
    where
        S: Future
    {
        #[pin]
        inner: MaybeDone<S>,
        #[pin]
        last_modified: M,
    }
}

impl<S, M, ResBody, E, Tz> Future for LastModifiedFuture<S, M>
where
    S: Future<Output = Result<Response<ResBody>, E>>,
    M: Future<Output = DateTime<Tz>>,
    Tz: TimeZone,
    Tz::Offset: Display,
{
    type Output = Result<Response<ResBody>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut projected = self.project();

        // Wait for response
        ready!(projected.inner.as_mut().poll(cx));

        // Wait for last modified time
        let last_modified = ready!(projected.last_modified.poll(cx));
        let mut response = projected
            .inner
            .take_output()
            .expect("LastModifiedFuture polled after ready")?;

        // Insert the header value
        response
            .headers_mut()
            .entry(axum::http::header::LAST_MODIFIED)
            .or_insert_with(|| {
                let last_modified = last_modified.with_timezone(&GMT);
                last_modified
                    .to_rfc2822()
                    .parse()
                    .expect("invalid header value for last modified time")
            });

        Poll::Ready(Ok(response))
    }
}
