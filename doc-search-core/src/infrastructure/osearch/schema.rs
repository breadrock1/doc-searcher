use serde_json::{Value, json};

use crate::domain::storage::models::{KnnIndexParams, KnnIndexParamsBuilder};
use crate::infrastructure::osearch::OSearchConfig;

pub const HYBRID_SEARCH_PIPELINE_NAME: &str = "hybrid-search-pipeline";
const NORMALIZATION_TECHNIQUE: &str = "rrf";
const COMBINATION_TECHNIQUE: &str = "arithmetic_mean";
const ALGO_PARAM_EF_SEARCH: u32 = 100;

pub fn build_hybrid_search_schema() -> Value {
    json!({
        "description": "Post processor for hybrid searching",
        "phase_results_processors": [
            {
                "normalization-processor": {
                    "normalization": {
                        "technique": NORMALIZATION_TECHNIQUE,
                    },
                    "combination": {
                        "technique": COMBINATION_TECHNIQUE,
                        "parameters": {
                            "weights": [0.55, 0.45]
                        }
                    }
                }
            }
        ]
    })
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
