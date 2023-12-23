mod delete;
mod delete_emoji;
mod download;
mod download_avatar;
mod download_default_avatar;
mod download_emoji;
mod upload;
mod upload_avatar;
mod upload_emoji;

use axum::extract::multipart::Field;
pub use delete::delete;
pub use delete_emoji::delete_emoji;
pub use download::download;
pub use download_avatar::download_avatar;
pub use download_default_avatar::download_default_avatar;
pub use download_emoji::download_emoji;
pub use upload::upload;
pub use upload_avatar::upload_avatar;
pub use upload_emoji::upload_emoji;

use crate::{
    config::MAX_SIZE,
    error::{Error, Result},
};

pub async fn extract_field(field: &mut Field<'_>) -> Result<Vec<u8>> {
    let mut current_size = 0_u64;
    let mut buffer = Vec::with_capacity(1024 * 1024 * 3);

    while let Some(chunk) = field.chunk().await? {
        current_size += chunk.len() as u64;

        if current_size > *MAX_SIZE {
            return Err(Error::TooLarge);
        }

        buffer.extend_from_slice(&chunk);
    }

    Ok(buffer)
}

pub async fn extract_field_custom_max(field: &mut Field<'_>, max: u64) -> Result<Vec<u8>> {
    let mut current_size = 0_u64;
    let mut buffer = Vec::with_capacity(max as usize);

    while let Some(chunk) = field.chunk().await? {
        current_size += chunk.len() as u64;

        if current_size > max {
            return Err(Error::TooLarge);
        }

        buffer.extend_from_slice(&chunk);
    }

    Ok(buffer)
}
