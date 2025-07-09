use serde_json::{json, Value};

pub const INGEST_PIPELINE_NAME: &str = "embeddings-ingest-pipeline";
pub const HYBRID_SEARCH_PIPELINE_NAME: &str = "hybrid-search-pipeline";
const NORMALIZATION_TECHNIQUE: &str = "min_max";
const COMBINATION_TECHNIQUE: &str = "arithmetic_mean";
const TOKENIZER_KIND: &str = "standard";
const KNN_SPACE_TYPE: &str = "cosinesimil";
const KNN_EF_SEARCHER: u32 = 100;
const KNN_DIMENSION: u32 = 384;
const TOKEN_LIMIT: u32 = 50;
const OVERLAP_RATE: f32 = 0.2;
const NUMBER_OF_SHARDS: u16 = 1;
const NUMBER_OF_REPLICAS: u16 = 1;

pub fn create_ingest_schema(model_id: &str) -> Value {
    let schema_query = json!({
        "description": "A text chunking and embedding ingest pipeline",
        "processors": [
            {
                "text_chunking": {
                "algorithm": {
                    "fixed_token_length": {
                        "token_limit": TOKEN_LIMIT,
                        "overlap_rate": OVERLAP_RATE,
                        "tokenizer": TOKENIZER_KIND
                    }
                },
                    "field_map": {
                        "content": "chunked_text"
                    }
                }
            },
            {
                "text_embedding": {
                    "model_id": model_id,
                    "field_map": {
                        "chunked_text": "embeddings"
                    }
                }
            }
        ]
    });

    schema_query
}

pub fn create_hybrid_search_schema(model_id: &str) -> Value {
    let schema_query = json!({
        "description": "Post processor for hybrid searching",
        "request_processors": [
            {
                "neural_query_enricher" : {
                    "default_model_id": model_id,
                }
            }
        ],

        "phase_results_processors": [
            {
                "normalization-processor": {
                    "normalization": {
                        "technique": NORMALIZATION_TECHNIQUE
                    },
                    "combination": {
                        "technique": COMBINATION_TECHNIQUE,
                        "parameters": {
                            "weights": [
                                0.3,
                                0.7
                            ]
                        }
                    }
                }
            }
        ]
    });

    schema_query
}

pub fn create_document_schema() -> Value {
    let schema_query = json!({
        "settings": {
            "index": {
                "knn": true,
                "knn.space_type": KNN_SPACE_TYPE,
                "knn.algo_param.ef_search": KNN_EF_SEARCHER,
                "number_of_shards": NUMBER_OF_SHARDS,
                "number_of_replicas": NUMBER_OF_REPLICAS,
                "search.default_pipeline": INGEST_PIPELINE_NAME,
            }
        },
        "mappings": {
            "properties": {
                "id": {
                    "type": "keyword"
                },
                "file_name": {
                    "type": "text"
                },
                "file_path": {
                    "type": "text"
                },
                "file_size": {
                    "type": "long"
                },
                "ssdeep": {
                    "type": "keyword"
                },
                "content": {
                    "type": "text"
                },
                "created_at": {
                    "type": "date",
                    "format": "epoch_second"
                },
                "modified_at": {
                    "type": "date",
                    "format": "epoch_second"
                },
                "embeddings": {
                    "type": "nested",
                    "properties": {
                        "knn": {
                            "type": "knn_vector",
                            "dimension": KNN_DIMENSION
                        }
                    }
                }
            }
        }
    });

    schema_query
}
