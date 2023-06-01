use axum::{
    extract::{Multipart, Path},
    headers::{authorization::Bearer, Authorization},
    response::IntoResponse,
    Json, TypedHeader,
};

use crate::{
    config::{AUTH, USE_ZSTD_AT},
    error::{Error, Result},
    storage::{self},
};

use super::extract_field;

#[derive(Serialize)]
pub struct AvatarUploadInfo {
    id: Uuid,
    path: String,
}

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

        let ext = field
            .file_name()
            .ok_or(Error::MissingFilename)?
            .rsplit_once('.')
            .ok_or(Error::IllegalFilename)?
            .1
            .to_string();

        let zstd = buffer.len() >= *USE_ZSTD_AT;
        let file_name = format!(
            "/avatars/{}{user_id}.{ext}",
            if zstd { "compr/" } else { "" }
        );

        storage::upload(buffer, &file_name, zstd).await?;

        Ok(Json(AvatarUploadInfo { path: file_name }))
    } else {
        Err(Error::MissingField)
    }
}
