use rstest::rstest;
use serde_json::Value;

use crate::domain::storage::models::KnnIndexParams;
use crate::infrastructure::osearch::config::OSearchConfig;
use crate::infrastructure::osearch::schema::{
    INGEST_PIPELINE_NAME, build_hybrid_search_schema, build_index_mappings, builder_ingest_schema,
};
use crate::infrastructure::osearch::tests::fixture::config::*;

#[rstest]
fn test_build_hybrid_search_schema(#[from(build_osearch_config)] config: OSearchConfig) {
    let schema = build_hybrid_search_schema(config.semantic());

    assert_eq!(
        Value::String("Post processor for hybrid searching".into()),
        schema["description"]
    );

    let default_model_id =
        &schema["request_processors"][0]["neural_query_enricher"]["default_model_id"];
    assert_eq!(Value::String(MODEL_ID.to_string()), *default_model_id);

    let normalization =
        &schema["phase_results_processors"][0]["normalization-processor"]["normalization"];
    assert_eq!(Value::String("min_max".into()), normalization["technique"]);

    let combination =
        &schema["phase_results_processors"][0]["normalization-processor"]["combination"];
    assert_eq!(
        Value::String("arithmetic_mean".into()),
        combination["technique"]
    );
    assert_eq!(
        Value::from(vec![0.3, 0.7]),
        combination["parameters"]["weights"]
    );
}

#[rstest]
fn test_builder_ingest_schema_with_default_params(
    #[from(build_osearch_config)] config: OSearchConfig,
) {
    let schema = builder_ingest_schema(&config, None);

    assert_eq!(
        Value::String("A text chunking and embedding ingest pipeline".into()),
        schema["description"]
    );

    let chunking = &schema["processors"][0]["text_chunking"]["algorithm"]["fixed_token_length"];
    assert_eq!(Value::from(TOKEN_LIMIT), chunking["token_limit"]);
    assert_eq!(Value::from(OVERLAP_RATE), chunking["overlap_rate"]);
    assert_eq!(Value::String("standard".into()), chunking["tokenizer"]);

    let embedding = &schema["processors"][1]["text_embedding"];
    assert_eq!(Value::String(MODEL_ID.to_string()), embedding["model_id"]);
}

#[rstest]
fn test_builder_ingest_schema_with_custom_params(
    #[from(build_osearch_config)] config: OSearchConfig,
    #[from(build_knn_index_params)] params: KnnIndexParams,
) {
    let schema = builder_ingest_schema(&config, Some(&params));

    let chunking = &schema["processors"][0]["text_chunking"]["algorithm"]["fixed_token_length"];
    assert_eq!(Value::from(TOKEN_LIMIT), chunking["token_limit"]);
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

    assert_eq!(
        Value::String(INGEST_PIPELINE_NAME.to_string()),
        schema["settings"]["default_pipeline"]
    );

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
}
