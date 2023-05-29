use std::fs::File;

use std::io::{Read, Write};
use std::path::Path;

use crate::config::FILE_STORAGE_PATH;
use crate::error::Result;

pub async fn upload(buffer: Vec<u8>, file_name: impl ToString, zstd: bool) -> Result<()> {
    let file_name = file_name.to_string();

    tokio::task::spawn_blocking(move || -> Result<()> {
        let path = format!("{}{}", *FILE_STORAGE_PATH, file_name);
        let mut file = File::create(&path)?;

        if zstd {
            super::compress_to(&buffer[..], file)?;
        } else {
            file.write_all(&buffer)?;
        }

        Ok(())
    })
    .await?
}

pub async fn download(file_name: impl ToString) -> Result<Vec<u8>> {
    let file_name = file_name.to_string();

    tokio::task::spawn_blocking(move || -> Result<Vec<u8>> {
        let mut file =
            std::fs::File::open(format!("{}/{}", *FILE_STORAGE_PATH, file_name.to_string()))?;

        Ok(if file_name.contains("/compr") {
            zstd::stream::decode_all(file)?
        } else {
            let mut buf = vec![];
            file.read_to_end(&mut buf)?;

            buf
        })
    })
    .await?
}

pub async fn remove(file_name: impl AsRef<Path>) -> Result<()> {
    tokio::fs::remove_file(file_name.as_ref())
        .await
        .map_err(Into::into)
}
