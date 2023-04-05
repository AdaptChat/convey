use axum::{extract::Path, http::header, response::IntoResponse};

use crate::{error::Result, storage};

pub async fn download_avatar(Path(user_id): Path<String>) -> Result<impl IntoResponse> {
    let content = storage::download(format!("/avatars/{user_id}")).await?;

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
