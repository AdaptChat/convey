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

pub async fn delete(
    TypedHeader(Authorization(auth)): TypedHeader<Authorization<Bearer>>,
    Path(filename): Path<String>,
) -> Result<impl IntoResponse> {
    if auth.token() != *AUTH {
        return Err(Error::NotAuthorized);
    }

    let filename = format!("/attachments/{}", filename);
    storage::remove(filename).await?;

    Ok(())
}
