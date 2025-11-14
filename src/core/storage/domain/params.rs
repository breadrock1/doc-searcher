use derive_builder::Builder;
use gset::Getset;

const KNN_EF_SEARCHER: u32 = 100;
const KNN_DIMENSION: u32 = 768;
const TOKEN_LIMIT: u32 = 700;
const OVERLAP_RATE: f32 = 0.2;

#[derive(Clone, Debug, Builder, Getset)]
pub struct CreateIndexParams {
    #[getset(set, vis = "pub")]
    #[getset(get, vis = "pub")]
    id: String,
    #[getset(get, vis = "pub")]
    name: String,
    #[getset(get, vis = "pub")]
    path: String,
    #[getset(get, vis = "pub")]
    knn: Option<KnnIndexParams>,
}

#[derive(Clone, Debug, Builder, Getset)]
pub struct KnnIndexParams {
    #[getset(get_copy, vis = "pub")]
    knn_ef_searcher: u32,
    #[getset(get_copy, vis = "pub")]
    knn_dimension: u32,
    #[getset(get_copy, vis = "pub")]
    token_limit: u32,
    #[getset(get_copy, vis = "pub")]
    overlap_rate: f32,
}

impl Default for KnnIndexParams {
    fn default() -> Self {
        KnnIndexParams {
            knn_ef_searcher: KNN_EF_SEARCHER,
            knn_dimension: KNN_DIMENSION,
            token_limit: TOKEN_LIMIT,
            overlap_rate: OVERLAP_RATE,
        }
    }
}
