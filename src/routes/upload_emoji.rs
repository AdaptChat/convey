use axum::{
    extract::{Multipart, Path},
    headers::{authorization::Bearer, Authorization},
    response::IntoResponse,
    Json, TypedHeader,
};
use serde::Serialize;

use crate::{
    config::AUTH,
    error::{Error, Result},
    storage,
};

use super::extract_field_custom_max;

#[derive(Serialize)]
struct EmojiUploadInfo {
    id: u64,
    path: String,
}

pub async fn upload_emoji(
    TypedHeader(Authorization(auth)): TypedHeader<Authorization<Bearer>>,
    Path(id): Path<u64>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    if auth.token() != *AUTH {
        return Err(Error::NotAuthorized);
    }

    if let Ok(Some(mut field)) = multipart.next_field().await {
        let buffer = extract_field_custom_max(&mut field, 1024 * 512).await?;

        let file_name = format!("/emoji/{id}");
        storage::upload(buffer, &file_name, false).await?;

        Ok(Json(EmojiUploadInfo {
            id,
            path: file_name,
        }))
    } else {
        Err(Error::MissingField)
    }
}
