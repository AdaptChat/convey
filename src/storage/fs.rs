use std::fs::File;

use std::path::Path;

use crate::config::FILE_STORAGE_PATH;
use crate::error::Result;

pub async fn upload(buffer: Vec<u8>, file_name: impl ToString) -> Result<()> {
    let file_name = file_name.to_string();

    tokio::task::spawn_blocking(move || -> Result<()> {
        let path = format!("{}{}", *FILE_STORAGE_PATH, file_name);
        let file = File::create(&path)?;

        super::compress_to(&buffer[..], file)?;

        Ok(())
    })
    .await?
}

pub async fn download(file_name: impl ToString) -> Result<Vec<u8>> {
    let file_name = file_name.to_string();

    tokio::task::spawn_blocking(move || -> Result<Vec<u8>> {
        let file =
            std::fs::File::open(format!("{}/{}", *FILE_STORAGE_PATH, file_name.to_string()))?;

        Ok(zstd::stream::decode_all(file)?)
    })
    .await?
}

pub async fn remove(file_name: impl AsRef<Path>) -> Result<()> {
    tokio::fs::remove_file(file_name.as_ref())
        .await
        .map_err(Into::into)
}
