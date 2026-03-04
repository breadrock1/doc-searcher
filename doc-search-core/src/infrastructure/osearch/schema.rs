use serde_json::{Value, json};

use crate::domain::storage::models::{KnnIndexParams, KnnIndexParamsBuilder};
use crate::infrastructure::osearch::OSearchConfig;
use crate::infrastructure::osearch::config::OSearchKnnConfig;

pub const INGEST_PIPELINE_NAME: &str = "embeddings-ingest-pipeline";
pub const HYBRID_SEARCH_PIPELINE_NAME: &str = "hybrid-search-pipeline";
const NORMALIZATION_TECHNIQUE: &str = "min_max";
const COMBINATION_TECHNIQUE: &str = "arithmetic_mean";
const TOKENIZER_KIND: &str = "standard";
const ALGO_PARAM_EF_SEARCH: u32 = 100;

pub fn build_hybrid_search_schema(config: &OSearchKnnConfig) -> Value {
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

pub fn builder_ingest_schema(config: &OSearchConfig, params: Option<&KnnIndexParams>) -> Value {
    let semantic_config = config.semantic();
    let knn_default_params = KnnIndexParamsBuilder::default()
        .knn_dimension(semantic_config.knn_dimension())
        .token_limit(semantic_config.token_limit())
        .overlap_rate(semantic_config.overlap_rate())
        .build()
        .expect("knn index params build failed");

    let knn_params = params.unwrap_or(&knn_default_params);
    let schema_query = json!({
        "description": "A text chunking and embedding ingest pipeline",
        "processors": [
            {
                "text_chunking": {
                "algorithm": {
                    "fixed_token_length": {
                        "token_limit": knn_params.token_limit,
                        "overlap_rate": knn_params.overlap_rate,
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
                    "model_id": semantic_config.model_id(),
                    "field_map": {
                        "chunked_text": "embeddings"
                    }
                }
            }
        ]
    });

    schema_query
}

pub fn build_index_mappings(config: &OSearchConfig, params: Option<&KnnIndexParams>) -> Value {
    let semantic_config = config.semantic();
    let knn_default_params = KnnIndexParamsBuilder::default()
        .knn_dimension(semantic_config.knn_dimension())
        .token_limit(semantic_config.token_limit())
        .overlap_rate(semantic_config.overlap_rate())
        .build()
        .expect("knn index params build failed");

    let knn_params = params.unwrap_or(&knn_default_params);
    let cluster_config = config.cluster();
    let schema_query = json!({
        "settings": {
            "index": {
                "knn": true,
                "knn.algo_param.ef_search": ALGO_PARAM_EF_SEARCH,
                "number_of_shards": cluster_config.number_of_shards(),
                "number_of_replicas": cluster_config.number_of_replicas(),
            },
            "default_pipeline": INGEST_PIPELINE_NAME,
        },
        "mappings": {
            "properties": {
                "large_doc_id": {
                    "type": "keyword"
                },
                "doc_part_id": {
                    "type": "keyword"
                },
                "file_name": {
                    "type": "text",
                    "fields": {
                      "keyword": { "type": "keyword", "ignore_above": 256 }
                    }
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
                            "dimension": knn_params.knn_dimension,
                            "method": {
                              "name": "hnsw",
                              "engine": "lucene",
                            }
                        }
                    }
                },
                "metadata": {
                    "type": "object",
                    "properties": {
                        "photo": {
                            "type": "keyword"
                        },
                        "pipelines": {
                            "type": "keyword"
                        },
                        "references": {
                            "type": "keyword"
                        },
                        "semantic_source": {
                            "type": "keyword"
                        },
                        "pipeline_id": {
                            "type": "long"
                        },
                        "source": {
                            "type": "keyword"
                        },
                        "summary": {
                            "type": "text"
                        },
                        "groups": {
                            "type": "nested",
                            "properties": {
                                "name": {
                                    "fields": {
                                        "keyword": {
                                            "type": "keyword"
                                        }
                                    },
                                    "type": "text"
                                }
                            }
                        },
                        "classes": {
                            "type": "nested",
                            "properties": {
                                "name": {
                                    "type": "keyword"
                                },
                                "probability": {
                                    "type": "float"
                                }
                            }
                        },
                        "icons": {
                            "type": "nested",
                            "properties": {
                                "name": {
                                    "type": "keyword"
                                }
                            }
                        },
                        "locations": {
                            "type": "nested",
                            "properties": {
                                "coords": {
                                    "type": "geo_point"
                                },
                                "name": {
                                    "type": "text"
                                }
                            }
                        },
                        "subjects": {
                            "type": "nested",
                            "properties": {
                                "name": {
                                    "fields": {
                                        "keyword": {
                                            "type": "keyword"
                                        }
                                    },
                                    "type": "text"
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    schema_query
}
