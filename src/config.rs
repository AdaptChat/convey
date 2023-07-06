use std::{sync::OnceLock, path::PathBuf};

use lazy_static::lazy_static;
use s3::{creds::Credentials, Bucket, Region};

pub static S3_BUCKET: OnceLock<Bucket> = OnceLock::new();

#[inline]
pub fn get_s3_bucket() -> &'static Bucket {
    S3_BUCKET.get_or_init(|| {
        Bucket::new(&*S3_BUCKET_NAME, S3_REGION.clone(), S3_CREDENTIALS.clone())
            .expect("Failed to init S3 bucket")
            .with_path_style()
    })
}

lazy_static! {
    pub static ref AUTH: String = std::env::var("AUTH").expect("`AUTH env var is required");
    pub static ref MAX_SIZE: u64 = std::env::var("MAX_SIZE").map_or(1024 * 1024 * 10, |v| v
        .parse()
        .expect("Invalid input for MAX_SIZE, expected integer"));
    pub static ref USE_ZSTD_AT: usize = std::env::var("USE_ZSTD_AT").map_or(20, |v| v
        .parse()
        .expect("Invalid input for USE_ZSTD_AT, expected integer"));
    pub static ref FILE_STORAGE_PATH: String =
        std::env::var("FILE_STORAGE_PATH").unwrap_or_else(|_| "./files".to_string());
    pub static ref ASSETS_PATH: PathBuf = std::env::var("ASSETS_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("assets"));
    pub static ref S3_BUCKET_NAME: String = std::env::var("S3_BUCKET_NAME").unwrap_or_default();
    pub static ref S3_REGION: Region = Region::Custom {
        region: std::env::var("S3_REGION").unwrap_or_default(),
        endpoint: std::env::var("S3_ENDPOINT").unwrap_or_default()
    };
    pub static ref S3_CREDENTIALS: Credentials = Credentials {
        access_key: std::env::var("S3_ACCESS_KEY").ok(),
        secret_key: std::env::var("S3_SECRET_KEY").ok(),
        security_token: None,
        session_token: None,
        expiration: None
    };
    pub static ref S3: bool = std::env::var("S3_BUCKET_NAME").is_ok()
        && std::env::var("S3_REGION").is_ok()
        && std::env::var("S3_ENDPOINT").is_ok();
}
