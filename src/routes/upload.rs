use std::fs::File;

use axum::{
    extract::Multipart,
    headers::{authorization::Bearer, Authorization},
    response::IntoResponse,
    Json, TypedHeader,
};
use uuid::Uuid;

use crate::{
    config::{AUTH, FILE_STORAGE_PATH},
    error::{Error, Result},
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

        let id = Uuid::new_v4().to_string();

        let path = tokio::task::spawn_blocking(move || -> Result<String> {
            let path = format!("{}/{id}-{file_name}", *FILE_STORAGE_PATH);
            let file = File::create(&path)?;

            zstd::stream::copy_encode(&buffer[..], file, 15)?;

            Ok(format!("/attachments/{id}/{file_name}"))
        })
        .await??;

        Ok(Json(UploadInfo { path }))
    } else {
        Err(Error::MissingField)
    }
}
