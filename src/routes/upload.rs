use axum::{
    extract::Multipart,
    headers::{authorization::Bearer, Authorization},
    response::IntoResponse,
    Json, TypedHeader,
};
use uuid::Uuid;

use crate::{
    config::AUTH,
    error::{Error, Result},
    storage,
};

use super::{extract_field, UploadInfo};

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
        let zstd = buffer.len() >= 20;
        let id = Uuid::new_v4().to_string();
        let file_name = format!(
            "/attachments/{id}{}/{file_name}",
            if zstd { "/compr" } else { "" }
        );

        storage::upload(buffer, &file_name, zstd).await?;

        Ok(Json(UploadInfo { path: file_name }))
    } else {
        Err(Error::MissingField)
    }
}
