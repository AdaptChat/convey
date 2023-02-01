use axum::{
    extract::{Path, Query},
    http::header,
    response::IntoResponse,
};
use rdenticon::{ImageFormat, Rgba};
use serde::Deserialize;

use crate::error::Error;
use crate::error::Result;

#[derive(Copy, Clone, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DefaultAvatarTheme {
    #[default]
    Transparent,
    Light,
    Dark,
}

impl DefaultAvatarTheme {
    pub const fn as_rgba(&self) -> Rgba {
        match self {
            DefaultAvatarTheme::Transparent => Rgba::transparent(),
            DefaultAvatarTheme::Light => Rgba::white(),
            DefaultAvatarTheme::Dark => Rgba::new(24, 24, 24, 255),
        }
    }
}

#[inline(always)]
pub const fn default_size() -> u32 {
    512
}

#[derive(Deserialize)]
pub struct DefaultAvatarQuery {
    #[serde(default)]
    theme: DefaultAvatarTheme,
    #[serde(default = "default_size")]
    size: u32,
}

pub async fn download_default_avatar(
    Path(user_id): Path<String>,
    Query(query): Query<DefaultAvatarQuery>,
) -> Result<impl IntoResponse> {
    if !(64..=512).contains(&query.size) {
        return Err(Error::InvalidAvatarSize);
    }

    let content = tokio::task::spawn_blocking(move || -> Result<Vec<u8>> {
        let config = rdenticon::Config::builder()
            .size(query.size)
            .background_color(query.theme.as_rgba())
            .build()
            .expect("config should be valid");

        let identicon = rdenticon::generate_identicon(user_id, &config);
        let mut buffer = Vec::new();
        identicon
            .encode(ImageFormat::Png, &mut buffer)
            .map_err(|_| Error::EncodingFailed)?;

        Ok(zstd::stream::decode_all(&*buffer)?)
    })
    .await??;

    Ok((
        [
            (header::CONTENT_TYPE, tree_magic_mini::from_u8(&content)),
            (
                header::CACHE_CONTROL,
                "public,max-age=604800,must-revalidate",
            ),
        ],
        content,
    ))
}
