use axum::{
    extract::Path,
    headers::{authorization::Bearer, Authorization},
    response::IntoResponse,
    TypedHeader,
};

use crate::{
    config::{AUTH, FILE_STORAGE_PATH},
    error::{Error, Result},
};

pub async fn delete(
    TypedHeader(Authorization(auth)): TypedHeader<Authorization<Bearer>>,
    Path((id, filename)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    if auth.token() != *AUTH {
        return Err(Error::NotAuthorized);
    }

    tokio::fs::remove_file(format!("{}/{id}-{filename}", *FILE_STORAGE_PATH)).await?;

    Ok(())
}
