use anyhow::Context;
use serde_json::{Value, json};

use super::schema::HYBRID_SEARCH_PIPELINE_NAME;
use crate::domain::searcher::models::{
    FilterParams, ResultOrder, ResultParams, SearchKindParams, SearchingParams,
};
use crate::infrastructure::osearch::dto::{
    FullTextQueryParams, FullTextQueryParamsBuilder, HybridQueryParams, HybridQueryParamsBuilder,
    RetrieveAllDocPartsQueryParams, RetrieveIndexDocsQueryParams,
    RetrieveIndexDocsQueryParamsBuilder, SemanticQueryParams, SemanticQueryParamsBuilder,
};
use crate::infrastructure::osearch::error::{OSearchError, OSearchResult};
use crate::shared::kernel::Tokens;

const HYBRID_PAGINATION_DEPTH: usize = 3;

pub fn build_search_query(params: SearchingParams, tokens: Option<Tokens>) -> OSearchResult<Value> {
    let result = params.get_result().clone();
    let filter = params.get_filter().cloned();

    match params.kind {
        SearchKindParams::Retrieve(params) => {
            let query_params = RetrieveIndexDocsQueryParamsBuilder::default()
                .path(params.path)
                .result(result)
                .filter(filter)
                .build()
                .context("failed to build retrieve query params")
                .map_err(OSearchError::BuildQueryError)?;

            Ok(query_params.build_query(None))
        }
        SearchKindParams::FullText(params) => {
            let query_params = FullTextQueryParamsBuilder::default()
                .query(params.query)
                .prefix(params.prefix)
                .result(result)
                .filter(filter)
                .build()
                .context("failed to build fulltext query params")
                .map_err(OSearchError::BuildQueryError)?;

            Ok(query_params.build_query(None))
        }
        SearchKindParams::Semantic(params) => {
            let query_params = SemanticQueryParamsBuilder::default()
                .query(params.query)
                .knn_amount(params.knn_amount)
                .min_score(params.min_score)
                .prefix(params.prefix)
                .result(result)
                .filter(filter)
                .build()
                .context("failed to build semantic query params")
                .map_err(OSearchError::BuildQueryError)?;

            Ok(query_params.build_query(tokens.as_deref()))
        }
        SearchKindParams::Hybrid(params) => {
            let query_params = HybridQueryParamsBuilder::default()
                .query(params.query)
                .knn_amount(params.knn_amount)
                .min_score(params.min_score)
                .prefix(params.prefix)
                .result(result)
                .filter(filter)
                .build()
                .context("failed to build hybrid query params")
                .map_err(OSearchError::BuildQueryError)?;

            Ok(query_params.build_query(tokens.as_deref()))
        }
    }
}

pub trait QueryBuildHelper {
    fn build_query(&self, query_embedding: Option<&Vec<f64>>) -> Value;
}

impl QueryBuildHelper for RetrieveAllDocPartsQueryParams {
    fn build_query(&self, _query_embedding: Option<&Vec<f64>>) -> Value {
        let large_doc_id = self.large_doc_id();
        let must_queries = match self.only_first_part() {
            false => json!([
                {
                    "match": {
                        "large_doc_id": large_doc_id,
                    }
                }
            ]),
            true => json!([
                {
                    "match": {
                        "large_doc_id": large_doc_id,
                    }
                },
                {
                    "match": {
                        "doc_part_id": 1,
                    }
                }
            ]),
        };

        let mut query = json!({
            "query": {
                "bool": {
                    "must": must_queries,
                }
            }
        });

        if self.with_sorting() {
            query[&"sort"] = json!({
                "doc_part_id": {
                    "order": "ASC"
                }
            })
        }

        query
    }
}

impl QueryBuildHelper for RetrieveIndexDocsQueryParams {
    fn build_query(&self, _query_embedding: Option<&Vec<f64>>) -> Value {
        let must = match self.path() {
            None => json!([
                {
                    "match": {
                        "doc_part_id": 1,
                    }
                }
            ]),
            Some(path) => json!([
                {
                    "match": {
                        "file_path": path,
                    }
                },
                {
                    "match": {
                        "doc_part_id": 1,
                    }
                }
            ]),
        };

        let result = self.result();
        let sort = build_sort_query(&result.order);
        let exclude = self.get_excluded_params();
        let filter = build_filter_query(self.filter());

        json!({
            "_source": {
                "exclude": exclude,
            },
            "query": {
                "bool": {
                    "must": must,
                    "filter": filter,
                }
            },
            "sort": sort,
        })
    }
}

impl QueryBuildHelper for FullTextQueryParams {
    fn build_query(&self, _query_embedding: Option<&Vec<f64>>) -> Value {
        let mut must = match self.query() {
            None => vec![json!({"match_all": {}})],
            Some(value) => vec![json!({"match": {"content": value} })],
        };
        if let Some(prefix) = self.prefix() {
            must.push(json!({"match": {"file_path": prefix} }));
        }
        let must = json!(must);

        let result = self.result();
        let highlight = build_highlight_query(result);
        let exclude = self.get_excluded_params();
        let sort = build_sort_query(&result.order);
        let filter = build_filter_query(self.filter());

        json!({
            "_source": {
                "exclude": exclude,
            },
            "highlight": highlight,
            "sort": sort,
            "query": {
                "bool": {
                    "must": must,
                    "filter": filter,
                }
            }
        })
    }
}

impl QueryBuildHelper for SemanticQueryParams {
    fn build_query(&self, query_embedding: Option<&Vec<f64>>) -> Value {
        let knn_amount = self.knn_amount();
        let knn_query = build_knn_query(query_embedding, knn_amount);

        let size = self.result().size;
        let exclude = self.get_excluded_params();
        let filter = build_filter_with_prefix_query(self.filter(), self.prefix().as_deref());
        let highlight = build_highlight_query(self.result());

        let mut base_value = json!({
            "_source": {
                "exclude": exclude,
            },
            "size": size,
            "highlight": highlight,
            "query": {
                "bool": {
                    "filter": filter,
                    "must": [
                        {
                            "nested": {
                                "path": "embeddings",
                                "score_mode": "max",
                                "query": knn_query
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

impl QueryBuildHelper for HybridQueryParams {
    fn build_query(&self, query_embedding: Option<&Vec<f64>>) -> Value {
        let query = self.query();
        let knn_amount = self.knn_amount();
        let semantic_query = build_knn_query(query_embedding, knn_amount);

        let size = self.result().size;
        let exclude = self.get_excluded_params();
        let filter = build_filter_with_prefix_query(self.filter(), self.prefix().as_deref());
        let highlight = build_highlight_query(self.result());

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
                "boost": 2.0
            }
        });

        let mut base_value = json!({
            "_source": {
                "exclude": exclude
            },
            "size": size,
            "search_pipeline": HYBRID_SEARCH_PIPELINE_NAME,
            "highlight": highlight,
            "query": {
                "hybrid": {
                    "pagination_depth": HYBRID_PAGINATION_DEPTH,
                    "queries": [
                        semantic_query,
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

fn build_knn_query(tokens: Option<&Vec<f64>>, knn_amount: u16) -> Value {
    match tokens.filter(|tokens| !tokens.is_empty()) {
        Some(tokens) => json!({
            "knn": {
                "embeddings.knn": {
                    "vector": tokens,
                    "k": knn_amount,
                }
            }
        }),
        None => json!({ "match_none": {} }),
    }
}

fn build_filter_with_prefix_query(filter: &Option<FilterParams>, prefix: Option<&str>) -> Value {
    let mut terms = match build_filter_query(filter) {
        Value::Array(terms) => terms,
        _ => unreachable!("build_filter_query always returns an array"),
    };
    if let Some(prefix) = prefix {
        terms.push(json!({"match": {"file_path": prefix} }));
    }
    Value::Array(terms)
}

fn build_filter_query(filter: &Option<FilterParams>) -> Value {
    match filter {
        None => json!([]),
        Some(params) => {
            let array_json_value = json!([]);
            if let Value::Array(mut filter_params) = array_json_value {
                if let Some(source) = params.source.as_ref() {
                    filter_params.push(json!({
                        "match": {
                            "metadata.source": source,
                        }
                    }))
                }

                if let Some(semantic_source) = params.semantic_source.as_ref() {
                    filter_params.push(json!({
                        "match": {
                            "metadata.semantic_source": semantic_source,
                        }
                    }))
                }

                if let Some(location_coords) = params.location_coords.as_ref() {
                    let distance = params.distance.as_deref().unwrap_or("5km");

                    filter_params.push(json!({
                        "nested": {
                            "path": "metadata.locations",
                            "query": {
                                "geo_distance": {
                                    "distance": distance,
                                    "metadata.locations.coords": location_coords,
                                }
                            }
                        }
                    }));
                }

                if let Some(created_from) = params.created_from {
                    filter_params.push(json!({
                        "range": {
                            "created_at": {
                                "gte": created_from,
                            }
                        }
                    }));
                }

                if let Some(created_to) = params.created_to {
                    filter_params.push(json!({
                        "range": {
                            "created_at": {
                                "lte": created_to,
                            }
                        }
                    }));
                }

                if let Some(file_size_from) = params.size_from {
                    filter_params.push(json!({
                        "range": {
                            "file_size": {
                                "gte": file_size_from,
                            }
                        }
                    }));
                }

                if let Some(file_size_to) = params.size_to {
                    filter_params.push(json!({
                        "range": {
                            "file_size": {
                                "lte": file_size_to,
                            }
                        }
                    }));
                }

                return Value::from(filter_params);
            }

            array_json_value
        }
    }
}

fn build_highlight_query(params: &ResultParams) -> Value {
    if !params.highlight_enable {
        return json!({});
    }

    let mut base_value = json!({
        "fields": {
            "content": {
                "pre_tags": [""],
                "post_tags": [""],
            }
        }
    });

    if let Some(fragment_size) = params.highlight_item_size {
        base_value["fields"]["content"]["fragment_size"] = json!(fragment_size);
    }

    if let Some(fragment_count) = params.highlight_items {
        base_value["fields"]["content"]["number_of_fragments"] = json!(fragment_count);
    }

    base_value
}

fn build_sort_query(order: &ResultOrder) -> Value {
    let order = match order {
        ResultOrder::ASC => "asc",
        ResultOrder::DESC => "desc",
    };

    json!([
        {
            "created_at": {
                "order": order
            }
        }
    ])
}
