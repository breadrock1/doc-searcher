use derive_builder::Builder;

#[derive(Debug, Builder)]
pub struct CreateIndexParams {
    pub id: String,
    pub knn: Option<KnnIndexParams>,
}

#[derive(Clone, Default, Debug, Builder)]
pub struct KnnIndexParams {
    pub knn_dimension: u32,
    pub token_limit: u32,
    pub overlap_rate: f32,
}
