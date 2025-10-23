use derive_builder::Builder;
use gset::Getset;

#[derive(Debug, Builder, Getset)]
pub struct UserInfo {
    #[getset(get, vis = "pub")]
    user_id: String
}
