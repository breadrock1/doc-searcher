use derive_builder::Builder;
use getset::{CopyGetters, Getters};

#[derive(Builder, Getters, CopyGetters)]
pub struct Index {
    #[getset(get = "pub")]
    id: String,
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    path: String,
    #[getset(get_copy = "pub")]
    created_at: i64,
}
