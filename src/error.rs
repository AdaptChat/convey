use axum::{extract::multipart::MultipartError, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorJson {
    code: u16,
    message: &'static str,
}

pub enum Error {
    NotAuthorized,
    MultipartError,
    TooLarge,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            Self::NotAuthorized => (
                StatusCode::UNAUTHORIZED,
                ErrorJson {
                    code: 401,
                    message: "Not authorized, missing `Authorization` header",
                },
            ),
            Self::MultipartError => (
                StatusCode::BAD_REQUEST,
                ErrorJson {
                    code: 400,
                    message: "Error parsing `multipart/form-data` request",
                },
            ),
            Self::TooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                ErrorJson {
                    code: 413,
                    message: "Attachment is too large",
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

pub type Result<T> = std::result::Result<T, Error>;
