use serde_json::{json, Value};

use crate::application::structures::params::KnnIndexParams;
use crate::infrastructure::osearch::config::{OSearchClusterConfig, OSearchKnnConfig};

pub const INGEST_PIPELINE_NAME: &str = "embeddings-ingest-pipeline";
pub const HYBRID_SEARCH_PIPELINE_NAME: &str = "hybrid-search-pipeline";
const NORMALIZATION_TECHNIQUE: &str = "min_max";
const COMBINATION_TECHNIQUE: &str = "arithmetic_mean";
const KNN_SPACE_TYPE: &str = "l2";
const TOKENIZER_KIND: &str = "standard";

pub fn create_ingest_schema(config: &OSearchKnnConfig, params: Option<&KnnIndexParams>) -> Value {
    let knn_default_params = KnnIndexParams::default();
    let knn_params = params.unwrap_or(&knn_default_params);

    let schema_query = json!({
        "description": "A text chunking and embedding ingest pipeline",
        "processors": [
            {
                "text_chunking": {
                "algorithm": {
                    "fixed_token_length": {
                        "token_limit": knn_params.token_limit(),
                        "overlap_rate": knn_params.overlap_rate(),
                        "tokenizer": TOKENIZER_KIND,
                    }
                },
                    "field_map": {
                        "content": "chunked_text"
                    }
                }
            },
            {
                "text_embedding": {
                    "model_id": config.model_id(),
                    "field_map": {
                        "chunked_text": "embeddings"
                    }
                }
            }
        ]
    });

    schema_query
}

pub fn create_hybrid_search_schema(config: &OSearchKnnConfig) -> Value {
    let schema_query = json!({
        "description": "Post processor for hybrid searching",
        "request_processors": [
            {
                "neural_query_enricher" : {
                    "default_model_id": config.model_id(),
                }
            }
        ],

        "phase_results_processors": [
            {
                "normalization-processor": {
                    "normalization": {
                        "technique": NORMALIZATION_TECHNIQUE,
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

pub fn create_document_schema(
    config: &OSearchClusterConfig,
    params: Option<&KnnIndexParams>,
) -> Value {
    let knn_default_params = KnnIndexParams::default();
    let knn_params = params.unwrap_or(&knn_default_params);

    let schema_query = json!({
        "settings": {
            "index": {
                "knn": true,
                "knn.algo_param.ef_search": knn_params.knn_ef_searcher(),
                "number_of_shards": config.number_of_shards(),
                "number_of_replicas": config.number_of_replicas(),
                "search.default_pipeline": HYBRID_SEARCH_PIPELINE_NAME,
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
                            "dimension": knn_params.knn_dimension(),
                            "method": {
                                "engine": "lucene",
                                "space_type": "l2",
                                "name": "hnsw",
                            }
                        }
                    }
                }
            }
        }
    });

    schema_query
}
