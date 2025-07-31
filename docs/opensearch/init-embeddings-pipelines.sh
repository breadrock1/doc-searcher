# Upload all-MiniLM-L6-v2 mode

POST /_plugins/_ml/models/_upload
{
  "name": "huggingface/sentence-transformers/all-MiniLM-L6-v2",
  "version": "1.0.1",
  "model_format": "TORCH_SCRIPT"
}

# returned task_id value
GET /_plugins/_ml/tasks/ORh30JcBW8Qg3Gf4IKKG

# returned new task_id value
POST /_plugins/_ml/models/$task_id/_load
{
  "parameters": {
    "wait_for_completion": true
  }
}

# returned model_id value
GET /_plugins/_ml/tasks/$task_id


# Setup Semantic and Hybrid Searching

PUT /_ingest/pipeline/embeddings-ingest-pipeline
{
  "description": "Pipeline for generating embeddings",
  "processors": [
    {
      "text_chunking": {
        "algorithm": {
          "fixed_token_length": {
            "token_limit": 50,
            "overlap_rate": 0.2,
            "tokenizer": "standard"
          }
        },
        "field_map": {
          "content": "chunked_text"
        }
      }
    },
    {
      "text_embedding": {
        "model_id": "$model_id",
        "field_map": {
          "chunked_text": "embeddings"
        }
      }
    }
  ]
}

PUT /_search/pipeline/embeddings-post-pipeline
{
  "description": "Post processor for hybrid search",
  "request_processors": [
    {
      "neural_query_enricher" : {
        "default_model_id": "$model_id"
      }
    }
  ],
  "phase_results_processors": [
    {
      "normalization-processor": {
        "normalization": {
          "technique": "min_max"
        },
        "combination": {
          "technique": "arithmetic_mean",
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
}

# Create test index with pipelines
PUT /my-own-index
{
  "settings": {
    "index": {
      "knn": true,
      "knn.algo_param.ef_search": 100
    },
    "default_pipeline": "embeddings-ingest-pipeline"
  },
  "mappings": {
    "properties": {
      "content": {
        "type": "text"
      },
      "embeddings": {
        "type": "nested",
        "properties": {
          "knn": {
            "type": "knn_vector",
            "dimension": 384,
            "method": {
              "engine": "lucene",
              "space_type": "l2",
              "name": "hnsw",
              "parameters": {}
            }
          }
        }
      }
    }
  }
}

POST /my-own-index/_doc?pipeline=embeddings-ingest-pipeline
{
  "content": "The quick brown fox jumps over the lazy dog"
}
POST /my-own-index/_doc?pipeline=embeddings-ingest-pipeline
{
  "content": "Fast animals leap over sleeping canines"
}
POST /my-own-index/_doc?pipeline=embeddings-ingest-pipeline
{
  "content": "Artificial intelligence is transforming industries"
}

POST /my-own-index/_search
{"query": {"match_all": {}}}

POST /my-own-index/_search
{
  "query": {
    "nested": {
      "score_mode": "max",
			"path":       "embeddings",
      "query": {
        "neural": {
          "embeddings.knn": {
            "query_text": "animals jumping faster",
            "model_id": "$model_id",
            "k": 100
          }
        }
      }

    }
  }
}

POST /my-own-index/_search?search_pipeline=embeddings-post-pipeline
{
  "query": {
    "hybrid": {
      "queries": [
        {
          "match": {
            "content": {
              "query": "fox jumps over the lazy dog"
            }
          }
        },
        {
          "neural": {
            "embeddings.knn": {
              "query_text": "fox jumps over the lazy dog",
              "model_id": "$model_id",
              "k": 5
            }
          }
        }
      ]
    }
  }
}
