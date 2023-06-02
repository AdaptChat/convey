use axum::{
    extract::{Path, Query},
    http::header,
    response::IntoResponse,
};

use crate::{
    error::{Error, Result},
    storage,
};

use super::download_default_avatar::DefaultAvatarQuery;

pub async fn download_avatar(
    Path(mut filename): Path<String>,
    Query(query): Query<DefaultAvatarQuery>,
) -> Result<impl IntoResponse> {
    if filename.ends_with("default.png") {
        let user_id = filename.rsplit('/').nth(1).ok_or(Error::NotFound)?;
        return Ok(super::download_default_avatar(user_id.to_string(), query)
            .await?
            .into_response());
    }

    super::remove_compr(&mut filename);
    let content = storage::download(format!("/avatars/{filename}")).await?;

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
