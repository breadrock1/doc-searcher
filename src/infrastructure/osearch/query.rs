use derive_builder::Builder;
use serde_json::{json, Value};

use super::schema::HYBRID_SEARCH_PIPELINE_NAME;
use crate::application::structures::params::{
    FilterParams, FullTextSearchParams, HybridSearchParams, RetrieveDocumentParams,
    SemanticSearchParams,
};

#[derive(Builder)]
pub struct QueryBuilderParams {
    pub model_id: Option<String>,
    pub include_extra_fields: Option<bool>,
}

impl QueryBuilderParams {
    pub fn set_model_id_if_none(&mut self, model_id: &str) {
        if self.model_id.is_none() {
            self.model_id = Some(model_id.to_string());
        }
    }
}

pub trait QueryBuilder {
    fn build_query(&self, params: QueryBuilderParams) -> Value;
}

impl From<&RetrieveDocumentParams> for QueryBuilderParams {
    fn from(params: &RetrieveDocumentParams) -> Self {
        QueryBuilderParamsBuilder::default()
            .model_id(None)
            .include_extra_fields(params.result().include_extra_fields())
            .build()
            .unwrap()
    }
}

impl From<&FullTextSearchParams> for QueryBuilderParams {
    fn from(params: &FullTextSearchParams) -> Self {
        QueryBuilderParamsBuilder::default()
            .model_id(None)
            .include_extra_fields(params.result().include_extra_fields())
            .build()
            .unwrap()
    }
}

impl From<&HybridSearchParams> for QueryBuilderParams {
    fn from(params: &HybridSearchParams) -> Self {
        QueryBuilderParamsBuilder::default()
            .model_id(params.model_id().clone())
            .include_extra_fields(params.result().include_extra_fields())
            .build()
            .unwrap()
    }
}

impl From<&SemanticSearchParams> for QueryBuilderParams {
    fn from(params: &SemanticSearchParams) -> Self {
        QueryBuilderParamsBuilder::default()
            .model_id(params.model_id().clone())
            .include_extra_fields(params.result().include_extra_fields())
            .build()
            .unwrap()
    }
}

impl QueryBuilder for RetrieveDocumentParams {
    fn build_query(&self, params: QueryBuilderParams) -> Value {
        let must = match self.path() {
            None => json!([{"match_all": {}}]),
            Some(path) => json!([{"match": {"file_path": path}}]),
        };

        let exclude = build_exclude_field(&params);

        json!({
            "_source": {
                "exclude": exclude,
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
    fn build_query(&self, params: QueryBuilderParams) -> Value {
        let must = match self.query() {
            None => json!([{"match_all": {}}]),
            Some(value) => json!({"match": {"content": value} }),
        };

        let exclude = build_exclude_field(&params);

        json!({
            "_source": {
                "exclude": exclude,
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
    fn build_query(&self, params: QueryBuilderParams) -> Value {
        let size = self.result().size();
        let query = build_semantic_query(self, params.model_id.as_ref());

        let exclude = params
            .include_extra_fields
            .map(|it| match it {
                false => Some(["content", "chunked_text", "embeddings"].as_slice()),
                true => Some(["content"].as_slice()),
            })
            .unwrap_or_default();

        json!({
            "_source": {
                "exclude": exclude,
            },
            "size": size,
            "query": {
                "bool": {
                    "must": [
                        {
                            "nested": {
                                "score_mode": "max",
                                "path": "embeddings",
                                "query": query,
                            }
                        }
                    ],
                    "filter": build_filter_query(self.filter()),
                }
            },
            "highlight": build_highlight_query(),
        })
    }
}

impl QueryBuilder for HybridSearchParams {
    fn build_query(&self, params: QueryBuilderParams) -> Value {
        let query = self.query();
        let size = self.result().size();
        let knn_amount = self.knn_amount();
        let model_id = params.model_id.as_ref();

        let exclude = params
            .include_extra_fields
            .map(|it| match it {
                false => Some(["content", "chunked_text", "embeddings"].as_slice()),
                true => Some(["content"].as_slice()),
            })
            .unwrap_or_default();

        json!({
            "_source": {
                "exclude": exclude
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

fn build_exclude_field(params: &QueryBuilderParams) -> Option<&[&str]> {
    params
        .include_extra_fields
        .map(|it| match it {
            false => Some(["chunked_text", "embeddings", "content"].as_slice()),
            true => Some(["chunked_text", "embeddings"].as_slice()),
        })
        .unwrap_or_default()
}

fn build_semantic_query(params: &SemanticSearchParams, model_id: Option<&String>) -> Value {
    let knn_amount = params.knn_amount();
    match params.tokens().as_ref() {
        None => {
            json!({
                "neural": {
                    "embeddings.knn": {
                        "query_text": params.query(),
                        "model_id": model_id,
                        "k": knn_amount
                    }
                }
            })
        }
        Some(tokens) => {
            json!({
                "knn": {
                    "embeddings.knn": {
                        "vector": tokens,
                        "k": knn_amount
                    }
                }
            })
        }
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
