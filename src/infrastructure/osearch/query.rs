use serde_json::{json, Value};

use crate::application::dto::{FilterParams, FullTextSearchParams, QueryBuilder, RetrieveDocumentParams, SemanticSearchParams, SemanticSearchWithTokensParams};

impl QueryBuilder for RetrieveDocumentParams {
    fn build_query(&self) -> Value {
        let must = match self.path() {
            None => json!([{"match_all": {}}]),
            Some(path) => json!([{"term": {"file_path.keyword": path}}]),
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
    fn build_query(&self) -> Value {
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

impl QueryBuilder for SemanticSearchParams {
    fn build_query(&self) -> Value {
        let query = self.query();
        let size = self.result().size();
        let knn_amount = self.knn_amount();
        let model_id = "qRhky5cBW8Qg3Gf4qJgp";

        json!({
            "_source": {
                "exclude": [
                    "embeddings",
                ]
            },
            "size": size,
            "query": {
                "neural": {
                    "embeddings.knn": {
                        "query_text": query,
                        "model_id": model_id,
                        "k": knn_amount,
                    }
                }
            },
            "highlight": build_highlight_query(),
        })
    }
}

impl QueryBuilder for SemanticSearchWithTokensParams {
    fn build_query(&self) -> Value {
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
                "knn": {
                    "embeddings.knn": {
                        "vector": query_vector,
                        "k": knn_amount,
                    }
                }
            },
            "highlight": build_highlight_query(),
        })
    }
}

fn build_filter_query(filter: &Option<FilterParams>) -> Value {
    let filter = match filter {
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
    };

    filter
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
