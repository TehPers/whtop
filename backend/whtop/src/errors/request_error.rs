use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct RequestError {
    status_code: StatusCode,
    formatted: FormattedError,
}

impl IntoResponse for RequestError {
    fn into_response(self) -> Response {
        (self.status_code, Json(self.formatted)).into_response()
    }
}

impl<E> From<E> for RequestError
where
    E: Into<FormattedError>,
{
    fn from(error: E) -> Self {
        RequestError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            formatted: error.into(),
        }
    }
}

pub trait IntoRequestError {
    fn with_status_code(self, code: StatusCode) -> RequestError;
}

impl IntoRequestError for RequestError {
    fn with_status_code(self, code: StatusCode) -> RequestError {
        RequestError {
            status_code: code,
            ..self
        }
    }
}

impl<E> IntoRequestError for E
where
    E: Into<FormattedError>,
{
    fn with_status_code(self, code: StatusCode) -> RequestError {
        RequestError {
            status_code: code,
            formatted: self.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct FormattedError {
    message: String,
    caused_by: Vec<String>,
}

impl From<anyhow::Error> for FormattedError {
    fn from(error: anyhow::Error) -> Self {
        let message = error.to_string();
        let caused_by = std::iter::successors(error.source(), |&source| source.source())
            .map(|err| err.to_string())
            .collect();
        FormattedError { message, caused_by }
    }
}
