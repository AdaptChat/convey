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
