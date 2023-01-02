mod delete;
mod download;
mod download_avatar;
mod upload;
mod upload_avatar;

use axum::extract::multipart::Field;
pub use delete::delete;
pub use download::download;
pub use download_avatar::download_avatar;
pub use upload::upload;
pub use upload_avatar::upload_avatar;

use serde::Serialize;

use crate::{
    config::MAX_SIZE,
    error::{Error, Result},
};

#[derive(Serialize)]
pub struct UploadInfo {
    path: String,
}

pub async fn extract_field(field: &mut Field<'_>) -> Result<Vec<u8>> {
    let mut current_size = 0_u64;
    let mut buffer = Vec::with_capacity(1024);

    while let Some(chunk) = field.chunk().await? {
        current_size += chunk.len() as u64;

        if current_size > *MAX_SIZE {
            return Err(Error::TooLarge);
        }

        buffer.extend_from_slice(&chunk);
    }

    Ok(buffer)
}
