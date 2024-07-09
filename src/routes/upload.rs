use axum::{
    extract::Multipart,
    response::IntoResponse,
    Json,
};
use axum_extra::{TypedHeader, headers::{authorization::Bearer, Authorization}};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    config::{AUTH, USE_ZSTD_AT},
    error::{Error, Result},
    storage,
};

use super::extract_field;

#[derive(Serialize)]
pub struct AttachmentUploadInfo {
    id: Uuid,
    path: String,
}

pub async fn upload(
    TypedHeader(Authorization(auth)): TypedHeader<Authorization<Bearer>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    if auth.token() != *AUTH {
        return Err(Error::NotAuthorized);
    }

    if let Ok(Some(mut field)) = multipart.next_field().await {
        let buffer = extract_field(&mut field).await?;

        let file_name = sanitize_filename::sanitize_with_options(
            field.file_name().ok_or(Error::MissingFilename)?,
            sanitize_filename::Options {
                windows: false,
                truncate: true,
                replacement: "_",
            },
        );
        let zstd = buffer.len() >= *USE_ZSTD_AT;
        let id = Uuid::new_v4();
        let file_name = format!(
            "/attachments/{}{id}/{file_name}",
            if zstd { "compr/" } else { "" }
        );

        storage::upload(buffer, &file_name, zstd).await?;

        Ok(Json(AttachmentUploadInfo {
            id,
            path: file_name,
        }))
    } else {
        Err(Error::MissingField)
    }
}
