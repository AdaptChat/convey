use crate::config::get_s3_bucket;
use crate::error::{Error, Result};

pub async fn upload(buffer: Vec<u8>, file_name: impl AsRef<str>) -> Result<()> {
    let resp = get_s3_bucket()
        .put_object(
            file_name.as_ref(),
            &tokio::task::spawn_blocking(move || super::compress(&buffer[..])).await??,
        )
        .await?
        .status_code();

    if resp != 200 {
        return Err(Error::S3Error(format!(
            "S3 returned non-200 status code ({resp})"
        )));
    }

    Ok(())
}

pub async fn download(file_name: impl AsRef<str>) -> Result<Vec<u8>> {
    let resp = get_s3_bucket().get_object(file_name).await?;

    match resp.status_code() {
        200 => Ok(resp.into()),
        404 => Err(Error::NotFound),
        code @ _ => Err(Error::S3Error(format!(
            "S3 returned non-200 status code ({code})"
        ))),
    }
}

pub async fn remove(file_name: impl AsRef<str>) -> Result<()> {
    let resp = get_s3_bucket()
        .delete_object(file_name)
        .await?
        .status_code();

    if resp != 200 {
        return Err(Error::S3Error(format!(
            "S3 returned non-200 status code ({resp})"
        )));
    }

    Ok(())
}
