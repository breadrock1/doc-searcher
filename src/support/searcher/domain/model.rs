use derive_builder::Builder;
use gset::Getset;

use crate::core::storage::domain::DocumentPart;

#[derive(Builder, Getset)]
pub struct Paginated<D> {
    #[getset(get, vis = "pub")]
    founded: Vec<D>,
    #[getset(get, vis = "pub")]
    scroll_id: Option<String>,
}

#[derive(Clone, Builder, Getset)]
pub struct FoundedDocument {
    #[getset(get, vis = "pub")]
    id: String,
    #[getset(get, vis = "pub")]
    folder_id: String,
    #[getset(get_copy, vis = "pub")]
    score: Option<f64>,
    #[getset(get, vis = "pub")]
    document: DocumentPart,
    #[getset(get, vis = "pub")]
    highlight: Vec<String>,
}

