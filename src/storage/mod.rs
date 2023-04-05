pub mod fs;
pub mod s3;

use std::io::{Read, Write};

use crate::{config::S3, error::Result};

pub fn compress(from: impl Read) -> Result<Vec<u8>> {
    zstd::stream::encode_all(from, 15).map_err(Into::into)
}

pub fn compress_to(from: impl Read, to: impl Write) -> Result<()> {
    zstd::stream::copy_encode(from, to, 15).map_err(Into::into)
}

pub async fn upload(buffer: Vec<u8>, file_name: impl AsRef<str>) -> Result<()> {
    if *S3 {
        s3::upload(buffer, file_name).await
    } else {
        fs::upload(buffer, file_name.as_ref()).await
    }
}

pub async fn download(file_name: impl AsRef<str>) -> Result<Vec<u8>> {
    if *S3 {
        s3::download(file_name).await
    } else {
        fs::download(file_name.as_ref()).await
    }
}

pub async fn remove(file_name: impl AsRef<str>) -> Result<()> {
    if *S3 {
        s3::remove(file_name).await
    } else {
        fs::remove(file_name.as_ref()).await
    }
}
