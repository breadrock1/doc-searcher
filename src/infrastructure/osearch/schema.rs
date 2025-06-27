use serde_json::{json, Value};

pub(super) fn create_document_schema() -> Value {
    let settings = json!({
        "index": {
            "knn": true,
            "knn.algo_param.ef_search": 100,
            "number_of_shards": 1,
            "number_of_replicas": 1,
        }
    });

    let chunks = json!({
        "type": "nested",
        "properties": {
            "chunk_id": {
                "type": "integer"
            },
            "chunked_text": {
                "type": "text",
                "analyzer": "standard"
            }
        }
    });

    let tokens = json!({
        "type": "nested",
        "properties": {
            "vector": {
                "type": "knn_vector",
                "dimension": 3,
                "method": {
                    "name": "hnsw",
                    "space_type": "cosinesimil",
                    "engine": "nmslib",
                    "parameters": {
                        "ef_construction": 128,
                        "m": 16
                    }
                }
            }
        }
    });

    json!({
        "settings": settings,
        "mappings": {
            "properties": {
                "id": {
                    "type": "keyword"
                },
                "file_name": {
                    "type": "keyword",
                    "fields": {
                        "text": {
                            "type": "text"
                        }
                    }
                },
                "file_path": {
                    "type": "keyword"
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
                        "chunks": chunks,
                        "tokens": tokens,
                    }
                }
           }
       }
    })
}
