use derive_builder::Builder;
use serde_json::{json, Value};

use super::schema::HYBRID_SEARCH_PIPELINE_NAME;
use crate::application::structures::params::{
    FilterParams, FullTextSearchParams, HybridSearchParams, ResultParams, RetrieveDocumentParams,
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
            "highlight": build_highlight_query(self.result()),
            "sort": build_sort_query(self.result().order()),
            "query": {
                "bool": {
                    "must": must,
                    "filter": build_filter_query(self.filter()),
                }
            }
        })
    }
}

impl QueryBuilder for SemanticSearchParams {
    fn build_query(&self, params: QueryBuilderParams) -> Value {
        let query = self.query();
        let tokens = self.tokens().as_ref();
        let knn_amount = self.knn_amount();
        let model_id = params.model_id.as_ref();
        let neural_query = build_semantic_query(query, tokens, knn_amount, model_id);

        let size = self.result().size();
        let filter = build_filter_query(self.filter());

        let exclude = params
            .include_extra_fields
            .map(|it| match it {
                false => Some(["content", "chunked_text", "embeddings"].as_slice()),
                true => Some(["content"].as_slice()),
            })
            .unwrap_or_default();

        let mut base_value = json!({
            "_source": {
                "exclude": exclude,
            },
            "size": size,
            "highlight": build_highlight_query(self.result()),
            "query": {
                "bool": {
                    "filter": filter,
                    "must": [
                        {
                            "nested": {
                                "path": "embeddings",
                                "score_mode": "max",
                                "query": neural_query
                            }
                        }
                    ]
                }
            }
        });

        if let Some(min_score) = self.min_score() {
            base_value["min_score"] = json!(min_score);
        }

        base_value
    }
}

impl QueryBuilder for HybridSearchParams {
    fn build_query(&self, params: QueryBuilderParams) -> Value {
        let query = self.query();
        let size = self.result().size();
        let knn_amount = self.knn_amount();
        let model_id = params.model_id.as_ref();
        let filter = build_filter_query(self.filter());

        let exclude = params
            .include_extra_fields
            .map(|it| match it {
                false => Some(["content"].as_slice()),
                true => Some(["chunked_text", "embeddings"].as_slice()),
            })
            .unwrap_or_default();

        let multi_match_query = json!({
            "query": query,
            "fields": ["content", "chunked_text"],
            "type": "cross_fields",
            "operator": "or",
        });

        let match_phrase_query = json!({
            "content": {
                "query": query,
                "slop": 2,
                "boost": 3.0
            }
        });

        let mut base_value = json!({
            "_source": {
                "exclude": exclude
            },
            "size": size,
            "search_pipeline": HYBRID_SEARCH_PIPELINE_NAME,
            "highlight": build_highlight_query(self.result()),
            "query": {
                "hybrid": {
                    "queries": [
                        {
                            "neural": {
                                "embeddings.knn": {
                                    "query_text": query,
                                    "model_id": model_id,
                                    "k": knn_amount
                                }
                            }
                        },
                        {
                            "bool": {
                                "should": [
                                    {
                                        "multi_match": multi_match_query
                                    },
                                    {
                                        "match_phrase": match_phrase_query
                                    }
                                ],
                                "filter": filter,
                            }
                        }
                    ]
                }
            },
        });

        if let Some(min_score) = self.min_score() {
            base_value["min_score"] = json!(min_score);
        }

        base_value
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

fn build_semantic_query(
    query: &str,
    tokens: Option<&Vec<f64>>,
    knn_amount: Option<u16>,
    model_id: Option<&String>,
) -> Value {
    match tokens {
        None => {
            json!({
                "neural": {
                    "embeddings.knn": {
                        "query_text": query,
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
            let array_json_value = json!([]);
            if let Value::Array(mut filter_params) = array_json_value {
                if let Some(doc_part_id) = params.doc_part_id() {
                    filter_params.push(json!({
                        "term": {
                            "doc_part_id": doc_part_id,
                        }
                    }));
                }

                if let Some(created_from) = params.created_from() {
                    if let Some(created_to) = params.created_to() {
                        filter_params.push(json!({
                            "range": {
                                "created_at": {
                                    "gte": created_from,
                                    "lte": created_to,
                                }
                            }
                        }))
                    } else {
                        filter_params.push(json!({
                            "range": {
                                "created_at": {
                                    "gte": created_from,
                                }
                            }
                        }))
                    }
                }

                if let Some(file_size_from) = params.size_from() {
                    if let Some(file_size_to) = params.size_to() {
                        filter_params.push(json!({
                            "range": {
                                "file_size": {
                                    "gte": file_size_from,
                                    "lte": file_size_to,
                                }
                            }
                        }))
                    } else {
                        filter_params.push(json!({
                            "range": {
                                "file_size": {
                                    "gte": params.size_from(),
                                }
                            }
                        }))
                    }
                }

                return Value::from(filter_params);
            }

            array_json_value
        }
    }
}

fn build_highlight_query(params: &ResultParams) -> Value {
    let mut base_value = json!({
        "fields": {
            "content": {
                "pre_tags": [""],
                "post_tags": [""],
            }
        }
    });

    if let Some(fragment_size) = params.highlight_item_size() {
        base_value["fields"]["content"]["fragment_size"] = json!(fragment_size);
    }

    if let Some(fragment_count) = params.highlight_items() {
        base_value["fields"]["content"]["number_of_fragments"] = json!(fragment_count);
    }

    base_value
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
