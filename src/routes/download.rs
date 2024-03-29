use axum::{extract::Path, http::header, response::IntoResponse};

use crate::{error::Result, storage};

pub async fn download(Path(filename): Path<String>) -> Result<impl IntoResponse> {
    let filename = format!("/attachments/{filename}");

    let content = storage::download(filename).await?;

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
