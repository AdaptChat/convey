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
    IllegalFilename,
    MultipartError(String),
    IOFailed(String),
    S3Error(String),
    EncodingFailed,
    TooLarge,
    InvalidAvatarSize,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            Self::NotAuthorized => (
                StatusCode::UNAUTHORIZED,
                ErrorJson {
                    code: 401,
                    message: Cow::Borrowed(
                        "Not authorized, missing / mismatched `Authorization` header",
                    ),
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
            Self::IllegalFilename => (
                StatusCode::BAD_REQUEST,
                ErrorJson {
                    code: 400,
                    message: Cow::Borrowed("Filename provided does not contain extension"),
                },
            ),
            Self::MultipartError(e) => (
                StatusCode::BAD_REQUEST,
                ErrorJson {
                    code: 400,
                    message: Cow::Owned(format!(
                        "Error parsing `multipart/form-data` request: {e}"
                    )),
                },
            ),
            Self::IOFailed(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorJson {
                    code: 500,
                    message: Cow::Owned(format!("Error saving attachment: {e}")),
                },
            ),
            Self::S3Error(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorJson {
                    code: 500,
                    message: Cow::Owned(format!("s3 error: {e}")),
                },
            ),
            Self::TooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                ErrorJson {
                    code: 413,
                    message: Cow::Borrowed("Attachment is too large"),
                },
            ),
            Self::EncodingFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorJson {
                    code: 500,
                    message: Cow::Borrowed("Failed to encode image"),
                },
            ),
            Self::InvalidAvatarSize => (
                StatusCode::BAD_REQUEST,
                ErrorJson {
                    code: 400,
                    message: Cow::Borrowed("Avatar size should be between 64 and 512"),
                },
            ),
        };

        (resp.0, Json(resp.1)).into_response()
    }
}

impl From<MultipartError> for Error {
    fn from(value: MultipartError) -> Self {
        use std::error::Error;

        if let Some(source) = value.source() {
            Self::MultipartError(format!("{source:?}"))
        } else {
            Self::MultipartError("Unknown Error".to_string())
        }
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

impl From<s3::error::S3Error> for Error {
    fn from(value: s3::error::S3Error) -> Self {
        Self::S3Error(value.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
