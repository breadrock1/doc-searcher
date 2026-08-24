#![allow(dead_code)]

use rstest::fixture;
use serde_json::json;

use crate::domain::storage::models::{KnnIndexParams, KnnIndexParamsBuilder};
use crate::infrastructure::osearch::config::OSearchConfig;

pub const MODEL_ID: &str = "p30o65gBnrvKdVIONWdC";
pub const KNN_DIMENSION: u32 = 768;
pub const TOKEN_LIMIT: u32 = 700;
pub const OVERLAP_RATE: f32 = 0.2;
pub const NUMBER_OF_SHARDS: usize = 8;
pub const NUMBER_OF_REPLICAS: usize = 1;

#[fixture]
pub fn build_osearch_config() -> OSearchConfig {
    let config: OSearchConfig = serde_json::from_value(json!({
        "address": "http://localhost:9200",
        "username": "admin",
        "password": "admin",
        "semantic": {
            "model_id": MODEL_ID,
            "knn_dimension": KNN_DIMENSION,
            "token_limit": TOKEN_LIMIT,
            "overlap_rate": OVERLAP_RATE,
            "knn_ef_searcher": 100,
        },
        "cluster": {
            "number_of_shards": NUMBER_OF_SHARDS,
            "number_of_replicas": NUMBER_OF_REPLICAS,
        },
    }))
    .expect("failed to deserialize test osearch config");

    config
}

#[fixture]
pub fn build_knn_index_params() -> KnnIndexParams {
    KnnIndexParamsBuilder::default()
        .knn_dimension(KNN_DIMENSION)
        .token_limit(TOKEN_LIMIT)
        .overlap_rate(OVERLAP_RATE)
        .build()
        .expect("failed to build knn index params")
}
