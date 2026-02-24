use rstest::fixture;

use crate::domain::storage::models::{CreateIndexParams, KnnIndexParams};

pub const TEST_INDEX: &str = "test-index";
pub const KNN_DIMENSION: u32 = 768;
pub const TOKEN_LIMIT: u32 = 700;
pub const OVERLAP_RATE: f32 = 0.2;

#[fixture]
pub fn create_index_params() -> CreateIndexParams {
    CreateIndexParams {
        id: TEST_INDEX.to_string(),
        knn: None,
    }
}

#[fixture]
pub fn create_index_params_with_knn() -> CreateIndexParams {
    let mut index_params = create_index_params();
    index_params.knn = Some(KnnIndexParams {
        knn_dimension: KNN_DIMENSION,
        token_limit: TOKEN_LIMIT,
        overlap_rate: OVERLAP_RATE,
    });

    index_params
}
