use axum::{extract::Path, http::header, response::IntoResponse};

use crate::{error::Result, storage};

pub async fn download(Path((id, file_name)): Path<(String, String)>) -> Result<impl IntoResponse> {
    let file_name = format!("/attachments/{id}/{file_name}");
    let content = storage::download(file_name).await?;

    Ok((
        [
            (header::CONTENT_TYPE, tree_magic_mini::from_u8(&content)),
            (
                header::CACHE_CONTROL,
                "public,max-age=604800,must-revalidate",
            ),
        ],
        content,
    ))
}
