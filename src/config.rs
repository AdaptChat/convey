use lazy_static::lazy_static;

lazy_static! {
    pub static ref AUTH: String = std::env::var("AUTH").expect("`AUTH env var is required");
    pub static ref MAX_SIZE: u64 = std::env::var("MAX_SIZE").map_or(1024 * 1024 * 10, |v| v
        .parse()
        .expect("Invalid input for MAX_SIZE, expected integer"));
    pub static ref FILE_STORAGE_PATH: String =
        std::env::var("FILE_STORAGE_PATH").expect("`FILE_STORAGE_PATH` is required");
}
