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
    name: String,
    path: String,
}

pub async fn upload_emoji(
    TypedHeader(Authorization(auth)): TypedHeader<Authorization<Bearer>>,
    Path((guild, name)): Path<(u64, String)>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    if auth.token() != *AUTH {
        return Err(Error::NotAuthorized);
    }

    if let Ok(Some(mut field)) = multipart.next_field().await {
        let buffer = extract_field_custom_max(&mut field, 1024 * 512).await?;

        if !sanitize_filename::is_sanitized_with_options(
            &name,
            sanitize_filename::OptionsForCheck {
                windows: false,
                truncate: true,
            },
        ) {
            return Err(Error::IllegalFilename);
        }

        let file_name = format!("/emoji/{guild}/{name}");
        storage::upload(buffer, &file_name, false).await?;

        Ok(Json(EmojiUploadInfo {
            name,
            path: file_name,
        }))
    } else {
        Err(Error::MissingField)
    }
}
