use derive_builder::Builder;
use gset::Getset;

#[derive(Debug, Builder, Getset)]
pub struct Resource {
    #[getset(get, vis = "pub")]
    id: String,
    #[getset(get, vis = "pub")]
    name: String,
    #[getset(get, vis = "pub")]
    created_at: chrono::NaiveDateTime,
    #[getset(get_copy, vis = "pub")]
    is_public: bool,
}

#[derive(Debug, Builder, Getset)]
pub struct UserInfo {
    #[getset(get, vis = "pub")]
    user_id: String,
}

