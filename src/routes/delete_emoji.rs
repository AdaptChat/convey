use axum::{
    extract::Path,
    headers::{authorization::Bearer, Authorization},
    response::IntoResponse,
    TypedHeader,
};

use crate::{
    config::AUTH,
    error::{Error, Result},
    storage,
};

pub async fn delete_emoji(
    TypedHeader(Authorization(auth)): TypedHeader<Authorization<Bearer>>,
    Path((guild, name)): Path<(u64, String)>,
) -> Result<impl IntoResponse> {
    if auth.token() != *AUTH {
        return Err(Error::NotAuthorized);
    }

    let filename = format!("/emoji/{guild}/{name}");
    storage::remove(filename).await?;

    Ok(())
}
