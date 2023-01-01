use std::fs::{self, File};

use axum::{
    extract::{Multipart, Path},
    headers::{authorization::Bearer, Authorization},
    response::IntoResponse,
    Json, TypedHeader,
};
use uuid::Uuid;

use crate::{
    config::{AUTH, FILE_STORAGE_PATH},
    error::{Error, Result},
};

use super::{UploadInfo, extract_field};

pub async fn upload_avatar(
    TypedHeader(Authorization(auth)): TypedHeader<Authorization<Bearer>>,
    Path(user_id): Path<String>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    if auth.token() != *AUTH {
        return Err(Error::NotAuthorized);
    }

    if let Ok(Some(mut field)) = multipart.next_field().await {
        let buffer = extract_field(&mut field).await?;

        let ext = sanitize_filename::sanitize_with_options(
            field.file_name().ok_or(Error::MissingFilename)?,
            sanitize_filename::Options {
                windows: false,
                truncate: true,
                replacement: "_",
            },
        )
        .split_once('.')
        .ok_or(Error::IllegalFilename)?
        .1
        .to_string();

        let id = Uuid::new_v4().to_string();

        let path = tokio::task::spawn_blocking(move || -> Result<String> {
            let path = format!("{}/{user_id}", *FILE_STORAGE_PATH);
            drop(fs::remove_dir(&path));
            fs::create_dir(&path)?;

            let path = format!("{}/{user_id}/{id}.{ext}", *FILE_STORAGE_PATH);
            let file = File::create(&path)?;

            zstd::stream::copy_encode(&buffer[..], file, 15)?;

            Ok(format!("/avatars/{user_id}/{id}.{ext}"))
        })
        .await??;

        Ok(Json(UploadInfo { path }))
    } else {
        Err(Error::MissingField)
    }
}
