use axum::{extract::Path, http::header, response::IntoResponse};

use crate::{error::Result, storage};

pub async fn download_emoji(Path((guild, name)): Path<(u64, String)>) -> Result<impl IntoResponse> {
    let content = storage::download(format!("/emoji/{guild}/{name}")).await?;

    Ok((
        [
            (header::CONTENT_TYPE, tree_magic_mini::from_u8(&content)),
            (
                header::CACHE_CONTROL,
                "public,max-age=604800,must-revalidate",
            ),
        ],
        content,
    )
        .into_response())
}
