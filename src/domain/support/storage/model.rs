use derive_builder::Builder;
use gset::Getset;

#[derive(Clone, Debug, Builder, Getset)]
pub struct Index {
    #[getset(get, vis = "pub")]
    id: String,
    #[getset(get, vis = "pub")]
    name: String,
    #[getset(get, vis = "pub")]
    path: String,
}

#[derive(Clone, Debug, Builder, Getset)]
pub struct DocumentPart {
    #[getset(set, vis = "pub")]
    #[getset(get_copy, vis = "pub")]
    doc_part_id: usize,
    #[getset(get, vis = "pub")]
    file_name: String,
    #[getset(get, vis = "pub")]
    file_path: String,
    #[getset(get_copy, vis = "pub")]
    file_size: u32,
    #[getset(get_copy, vis = "pub")]
    created_at: i64,
    #[getset(get_copy, vis = "pub")]
    modified_at: i64,
    #[getset(set, vis = "pub")]
    #[getset(get, vis = "pub")]
    content: Option<String>,
    #[getset(get, vis = "pub")]
    chunked_text: Option<Vec<String>>,
    #[getset(get, vis = "pub")]
    embeddings: Option<Vec<Embeddings>>,
}

#[derive(Clone, Debug)]
pub struct Embeddings {
    pub knn: Vec<f64>,
}

impl Embeddings {
    pub fn new(knn: Vec<f64>) -> Self {
        Self { knn }
    }
}
