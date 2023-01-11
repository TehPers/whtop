use axum::http::{Request, Response};
use pin_project_lite::pin_project;
use std::{
    fmt::{Display, Formatter},
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};
use tower::{Layer, Service};

/// Options for the `cache-control` header.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CacheOptions {
    /// The response cannot be stored (`no-store`).
    pub no_store: bool,
    /// The response must be validated with the origin server before using it (`no-cache`).
    pub no_cache: bool,
    /// The response can be stored in shared caches (`public`).
    pub public: bool,
    /// The response cannot be stored in shared caches (`private`).
    pub private: bool,
    /// The max age in caches, in seconds (`max-age`).
    pub max_age: Option<u64>,
    /// The max age in shared caches, in seconds (`s-maxage`).
    pub shared_max_age: Option<u64>,
    /// Whether the response must be validated when stale (`must-revalidate`).
    pub must_revalidate: bool,
    /// Whether the response must be validated when stale in shared caches (`proxy-revalidate`).
    pub proxy_revalidate: bool,
    /// Whether the response's caching requirements must be understood to be cached
    /// (`must-understand`). This should be coupled with `no_store` to prevent any caching
    /// of responses that cannot be understood.
    pub must_understand: bool,
    /// Whether the response should not be transformed by intermediaries (`no-transform`).
    pub no_transform: bool,
    /// Whether the resource is immutable (`immutable`).
    pub immutable: bool,
    /// How many seconds a stale response can be reused while being revalidated
    /// (`stale-while-revalidate`).
    pub stale_while_revalidate: Option<u64>,
    /// How many seconds a stale error response can be reused (`stale-if-error`)
    pub stale_if_error: Option<u64>,
}

impl Default for CacheOptions {
    fn default() -> Self {
        Self {
            no_store: false,
            no_cache: false,
            public: false,
            private: false,
            max_age: None,
            shared_max_age: None,
            must_revalidate: false,
            proxy_revalidate: false,
            must_understand: false,
            no_transform: false,
            immutable: false,
            stale_while_revalidate: None,
            stale_if_error: None,
        }
    }
}

impl Display for CacheOptions {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        fn delimit(is_first: &mut bool, f: &mut Formatter) -> std::fmt::Result {
            if *is_first {
                *is_first = false;
                Ok(())
            } else {
                f.write_str(", ")
            }
        }

        let mut is_first = true;
        if self.no_store {
            delimit(&mut is_first, f)?;
            f.write_str("no-store")?;
        }
        if self.no_cache {
            delimit(&mut is_first, f)?;
            f.write_str("no-cache")?;
        }
        if self.public {
            delimit(&mut is_first, f)?;
            f.write_str("public")?;
        }
        if self.private {
            delimit(&mut is_first, f)?;
            f.write_str("private")?;
        }
        if let Some(max_age) = self.max_age {
            delimit(&mut is_first, f)?;
            write!(f, "max-age={max_age}")?;
        }
        if let Some(shared_max_age) = self.shared_max_age {
            delimit(&mut is_first, f)?;
            write!(f, "s-maxage={shared_max_age}")?;
        }
        if self.must_revalidate {
            delimit(&mut is_first, f)?;
            f.write_str("must-revalidate")?;
        }
        if self.proxy_revalidate {
            delimit(&mut is_first, f)?;
            f.write_str("proxy-revalidate")?;
        }
        if self.must_understand {
            delimit(&mut is_first, f)?;
            f.write_str("must-understand")?;
        }
        if self.no_transform {
            delimit(&mut is_first, f)?;
            f.write_str("no-transform")?;
        }
        if self.immutable {
            delimit(&mut is_first, f)?;
            f.write_str("immutable")?;
        }
        if let Some(stale_while_revalidate) = self.stale_while_revalidate {
            delimit(&mut is_first, f)?;
            write!(f, "stale-while-revalidate={stale_while_revalidate}")?;
        }
        if let Some(stale_if_error) = self.stale_if_error {
            delimit(&mut is_first, f)?;
            write!(f, "stale-if-error={stale_if_error}")?;
        }

        Ok(())
    }
}

/// Applies the `cache-control` header to outgoing responses.
#[derive(Clone, Debug)]
pub struct CacheControlLayer {
    options: CacheOptions,
}

impl CacheControlLayer {
    pub fn new(options: CacheOptions) -> Self {
        Self { options }
    }
}

impl<S> Layer<S> for CacheControlLayer {
    type Service = CacheControl<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CacheControl {
            inner,
            options: self.options,
        }
    }
}

/// Applies the `cache-control` header to the outgoing response.
#[derive(Clone, Debug)]
pub struct CacheControl<S> {
    inner: S,
    options: CacheOptions,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for CacheControl<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let inner = self.inner.call(req);
        ResponseFuture {
            inner,
            options: self.options,
        }
    }
}

pin_project! {
    /// Response future for [`CacheControl`].
    #[derive(Clone, Debug)]
    pub struct ResponseFuture<F> {
        #[pin]
        inner: F,
        options: CacheOptions
    }
}

impl<F, B, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response<B>, E>>,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let projected = self.project();
        let mut response = ready!(projected.inner.poll(cx))?;
        response
            .headers_mut()
            .entry(axum::http::header::CACHE_CONTROL)
            .or_insert_with(|| {
                projected
                    .options
                    .to_string()
                    .parse()
                    .expect("invalid cache-control value")
            });

        Poll::Ready(Ok(response))
    }
}
