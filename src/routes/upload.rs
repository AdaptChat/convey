use std::fs::File;

use axum::{
    extract::Multipart,
    headers::{authorization::Bearer, Authorization},
    response::IntoResponse,
    Json, TypedHeader,
};
use futures_util::stream::TryStreamExt;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    config::{AUTH, FILE_STORAGE_PATH, MAX_SIZE},
    error::{Error, Result},
};

#[derive(Serialize)]
struct UploadInfo {
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
        let mut current_size = 0_u64;
        let mut buffer = Vec::with_capacity(1024);

        while let Some(chunk) = field.try_next().await? {
            current_size += chunk.len() as u64;

            if current_size > *MAX_SIZE {
                return Err(Error::TooLarge);
            }

            buffer.append(&mut chunk.to_vec());
        }

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

            zstd::stream::copy_encode(&buffer[..], file, 10)?;

            Ok(format!("/attachments/{id}/{file_name}"))
        })
        .await??;

        Ok(Json(UploadInfo { path }))
    } else {
        Err(Error::MissingField)
    }
}
