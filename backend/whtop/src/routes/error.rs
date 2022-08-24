use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

pub type RouteResult<T> = Result<T, RouteError>;

#[derive(Debug)]
pub enum RouteError {
    InternalError(anyhow::Error),
}

impl Display for RouteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RouteError::InternalError(_) => write!(f, "internal error"),
        }
    }
}

impl Error for RouteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RouteError::InternalError(err) => Some(&**err),
        }
    }
}

impl From<anyhow::Error> for RouteError {
    fn from(error: anyhow::Error) -> Self {
        RouteError::InternalError(error)
    }
}

impl IntoResponse for RouteError {
    fn into_response(self) -> Response {
        match self {
            RouteError::InternalError(inner) => {
                let body = RouteErrorResponseBody::InternalError {
                    message: format!("{inner:?}"),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum RouteErrorResponseBody {
    InternalError { message: String },
}
