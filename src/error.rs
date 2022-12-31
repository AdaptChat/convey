use std::{borrow::Cow, io::ErrorKind};

use axum::{extract::multipart::MultipartError, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use tokio::task::JoinError;

#[derive(Serialize)]
pub struct ErrorJson {
    code: u16,
    message: Cow<'static, str>,
}

pub enum Error {
    NotAuthorized,
    NotFound,
    MissingField,
    MissingFilename,
    MultipartError,
    IOFailed(String),
    TooLarge,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            Self::NotAuthorized => (
                StatusCode::UNAUTHORIZED,
                ErrorJson {
                    code: 401,
                    message: Cow::Borrowed("Not authorized, missing `Authorization` header"),
                },
            ),
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                ErrorJson {
                    code: 404,
                    message: Cow::Borrowed("Not found"),
                },
            ),
            Self::MissingField => (
                StatusCode::BAD_REQUEST,
                ErrorJson {
                    code: 400,
                    message: Cow::Borrowed("No multipart field is attached"),
                },
            ),
            Self::MissingFilename => (
                StatusCode::BAD_REQUEST,
                ErrorJson {
                    code: 400,
                    message: Cow::Borrowed("Missing filename in form-data"),
                },
            ),
            Self::MultipartError => (
                StatusCode::BAD_REQUEST,
                ErrorJson {
                    code: 400,
                    message: Cow::Borrowed("Error parsing `multipart/form-data` request"),
                },
            ),
            Self::IOFailed(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorJson {
                    code: 500,
                    message: Cow::Owned(format!("Error saving attachment: {e}")),
                },
            ),
            Self::TooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                ErrorJson {
                    code: 413,
                    message: Cow::Borrowed("Attachment is too large"),
                },
            ),
        };

        (resp.0, Json(resp.1)).into_response()
    }
}

impl From<MultipartError> for Error {
    fn from(value: MultipartError) -> Self {
        Self::MultipartError
    }
}

impl From<JoinError> for Error {
    fn from(value: JoinError) -> Self {
        Self::IOFailed(value.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            ErrorKind::NotFound => Self::NotFound,
            _ => Self::IOFailed(value.to_string()),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
