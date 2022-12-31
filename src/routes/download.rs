use axum::{extract::Path, http::header, response::IntoResponse};

use crate::{config::FILE_STORAGE_PATH, error::Result};

pub async fn download(Path((id, filename)): Path<(String, String)>) -> Result<impl IntoResponse> {
    let content = tokio::task::spawn_blocking(move || -> Result<Vec<u8>> {
        let file = std::fs::File::open(format!("{}/{id}-{filename}", *FILE_STORAGE_PATH))?;

        Ok(zstd::stream::decode_all(file)?)
    })
    .await??;

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
