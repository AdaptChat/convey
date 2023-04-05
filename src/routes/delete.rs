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
    Path((id, file_name)): Path<(String, String)>,
) -> Result<impl IntoResponse> {
    if auth.token() != *AUTH {
        return Err(Error::NotAuthorized);
    }

    let file_name = format!("/attachments/{id}/{file_name}");
    storage::remove(file_name).await?;

    Ok(())
}
