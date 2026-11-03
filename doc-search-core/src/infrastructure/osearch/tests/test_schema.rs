use rstest::rstest;
use serde_json::Value;

use crate::domain::storage::models::KnnIndexParams;
use crate::infrastructure::osearch::config::OSearchConfig;
use crate::infrastructure::osearch::schema::{build_hybrid_search_schema, build_index_mappings};
use crate::infrastructure::osearch::tests::fixture::config::*;

#[rstest]
fn test_build_hybrid_search_schema() {
    let schema = build_hybrid_search_schema();

    assert!(schema["request_processors"].is_null());
    assert!(schema["phase_results_processors"][0]["normalization-processor"]["combination"]["technique"]
        .is_string());
}

#[rstest]
fn test_build_index_mappings_with_default_params(
    #[from(build_osearch_config)] config: OSearchConfig,
) {
    let schema = build_index_mappings(&config, None);

    let index_settings = &schema["settings"]["index"];
    assert_eq!(Value::Bool(true), index_settings["knn"]);
    assert_eq!(Value::from(100), index_settings["knn.algo_param.ef_search"]);
    assert_eq!(
        Value::from(NUMBER_OF_SHARDS),
        index_settings["number_of_shards"]
    );
    assert_eq!(
        Value::from(NUMBER_OF_REPLICAS),
        index_settings["number_of_replicas"]
    );

    assert!(schema["settings"]["index"]["default_pipeline"].is_null());

    let embeddings = &schema["mappings"]["properties"]["embeddings"];
    assert_eq!(Value::String("nested".into()), embeddings["type"]);
    assert_eq!(
        Value::String("knn_vector".into()),
        embeddings["properties"]["knn"]["type"]
    );
    assert_eq!(
        Value::from(KNN_DIMENSION),
        embeddings["properties"]["knn"]["dimension"]
    );
    assert_eq!(
        Value::String("hnsw".into()),
        embeddings["properties"]["knn"]["method"]["name"]
    );
    assert_eq!(
        Value::String("lucene".into()),
        embeddings["properties"]["knn"]["method"]["engine"]
    );
}

#[rstest]
fn test_build_index_mappings_with_custom_params(
    #[from(build_osearch_config)] config: OSearchConfig,
    #[from(build_knn_index_params)] params: KnnIndexParams,
) {
    let schema = build_index_mappings(&config, Some(&params));

    let knn_dimension =
        &schema["mappings"]["properties"]["embeddings"]["properties"]["knn"]["dimension"];
    assert_eq!(Value::from(KNN_DIMENSION), *knn_dimension);

    assert!(schema["settings"]["index"]["default_pipeline"].is_null());
}
