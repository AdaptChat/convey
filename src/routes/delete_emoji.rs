use axum::{
    extract::Path,
    response::IntoResponse,
};
use axum_extra::{TypedHeader, headers::{authorization::Bearer, Authorization}};

use crate::{
    config::AUTH,
    error::{Error, Result},
    storage,
};

pub async fn delete_emoji(
    TypedHeader(Authorization(auth)): TypedHeader<Authorization<Bearer>>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse> {
    if auth.token() != *AUTH {
        return Err(Error::NotAuthorized);
    }

    let filename = format!("/emoji/{id}");
    storage::remove(filename).await?;

    Ok(())
}
