use serde_json::{json, Value};

use super::schema::HYBRID_SEARCH_PIPELINE_NAME;
use crate::application::dto::params::{FilterParams, FullTextSearchParams, HybridSearchParams, QueryBuilder, RetrieveDocumentParams, SemanticSearchParams, SemanticSearchWithTokensParams};

impl QueryBuilder for RetrieveDocumentParams {
    fn build_query(&self, _: Option<&str>) -> Value {
        let must = match self.path() {
            None => json!([{"match_all": {}}]),
            Some(path) => json!([{"match": {"file_path": path}}]),
        };

        json!({
            "_source": {
                "exclude": [
                    "chunked_text",
                    "embeddings",
                    "content",
               ]
            },
            "query": {
                "bool": {
                    "must": must,
                    "filter": build_filter_query(self.filter()),
                }
            },
            "highlight": build_highlight_query(),
            "sort": build_sort_query(self.result().order()),
        })
    }
}

impl QueryBuilder for FullTextSearchParams {
    fn build_query(&self, _: Option<&str>) -> Value {
        let must = match self.query() {
            None => json!([{"match_all": {}}]),
            Some(value) => json!({"match": {"content": value} }),
        };

        json!({
            "_source": {
                "exclude": [
                    "chunked_text",
                    "embeddings",
                    "content",
               ]
            },
            "query": {
                "bool": {
                    "must": must,
                    "filter": build_filter_query(self.filter()),
                }
            },
            "highlight": build_highlight_query(),
            "sort": build_sort_query(self.result().order()),
        })
    }
}

impl QueryBuilder for HybridSearchParams {
    fn build_query(&self, model_id: Option<&str>) -> Value {
        let query = self.query();
        let size = self.result().size();
        let knn_amount = self.knn_amount();

        json!({
            "_source": {
                "exclude": [
                    "chunked_text",
                    "embeddings",
                    "content",
               ]
            },
            "size": size,
            "query": {
                "hybrid": {
                    "queries": [
                        {
                            "match": {
                                "content": {
                                    "query": query,
                                }
                            }
                        },
                        {
                            "neural": {
                                "embeddings.knn": {
                                    "query_text": query,
                                    "model_id": model_id,
                                    "k": knn_amount,
                                }
                            }
                        },
                    ]
                }
            },
            "highlight": build_highlight_query(),
            "search_pipeline": HYBRID_SEARCH_PIPELINE_NAME,
        })
    }
}

impl QueryBuilder for SemanticSearchParams {
    fn build_query(&self, model_id: Option<&str>) -> Value {
        let query = self.query();
        let size = self.result().size();
        let knn_amount = self.knn_amount();

        json!({
            "_source": {
                "exclude": [
                    "embeddings",
                ]
            },
            "size": size,
            "query": {
                "nested": {
                    "score_mode": "max",
                    "path": "embeddings",
                    "query": {
                        "neural": {
                            "embeddings.knn": {
                                "query_text": query,
                                "model_id": model_id,
                                "k": knn_amount
                            }
                        }
                    }
                }
            },
            "highlight": build_highlight_query(),
        })
    }
}

impl QueryBuilder for SemanticSearchWithTokensParams {
    fn build_query(&self, _: Option<&str>) -> Value {
        let size = self.result().size();
        let knn_amount = self.knn_amount();
        let query_vector = self.tokens();

        json!({
            "_source": {
                "excludes": [
                    "embeddings",
                ]
            },
            "size": size,
            "query": {
                "nested": {
                    "score_mode": "max",
                    "path": "embeddings",
                    "query": {
                        "knn": {
                            "embeddings.knn": {
                                "vector": query_vector,
                                "k": knn_amount
                            }
                        }
                    }
                }
            },
            "highlight": build_highlight_query(),
        })
    }
}

fn build_filter_query(filter: &Option<FilterParams>) -> Value {
    match filter {
        None => json!([]),
        Some(params) => {
            json!([
                {
                    "range": {
                        "created_at": {
                            "gte": params.created_from(),
                            "lte": params.created_to()
                        }
                    }
                },
                {
                    "range": {
                        "file_size": {
                            "gte": params.size_from(),
                            "lte": params.size_to()
                        }
                    }
                }
            ])
        }
    }
}

fn build_highlight_query() -> Value {
    json!({
        "fields": {
            "content": {
                "type": "plain",
                "pre_tags": [""],
                "post_tags": [""]
            }
        }
    })
}

fn build_sort_query(order: &str) -> Value {
    json!([
        {
            "created_at": {
                "order": order
            }
        }
    ])
}
