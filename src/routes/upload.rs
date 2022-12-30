use axum::{
    extract::Multipart,
    headers::{authorization::Bearer, Authorization},
    response::IntoResponse,
    TypedHeader,
};
use highway::{HighwayHasher, Key};
use lazy_static::lazy_static;

use crate::{
    config::{AUTH, MAX_SIZE},
    error::{Error, Result},
};

lazy_static! {
    static ref hasher: HighwayHasher = HighwayHasher::new(Key([
        16129575160643678914,
        8006219525329119735,
        2098523345898263339,
        540360354731526120
    ]));
}

pub async fn upload(
    TypedHeader(Authorization(auth)): TypedHeader<Authorization<Bearer>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    if auth.token() != *AUTH {
        return Err(Error::NotAuthorized);
    }

    if let Ok(Some(mut field)) = multipart.next_field().await {
        let mut current_size = 0_u64;
        let mut buffer = Vec::<u8>::with_capacity(1024);

        while let Some(chunk) = field.chunk().await? {
            let count = chunk.len();
            current_size += count as u64;

            if current_size > *MAX_SIZE {
                return Err(Error::TooLarge);
            }
            buffer.reserve(count);

            let len = buffer.len();

            unsafe {
                std::ptr::copy_nonoverlapping(chunk.as_ptr(), buffer.as_mut_ptr().add(len), count);

                buffer.set_len(len + count)
            }
        }
    }

    Ok(())
}
