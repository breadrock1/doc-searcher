use crate::errors::WebError;
use crate::forms::documents::document::Document;
use crate::forms::documents::metadata::HighlightEntity;
use crate::forms::documents::preview::DocumentPreview;
use crate::forms::documents::vector::DocumentVectors;
use crate::forms::documents::DocumentsTrait;
use crate::forms::folders::info::InfoFolder;
use crate::forms::searcher::s_params::SearchParams;
use crate::services::searcher::elastic::context::ContextOptions;
use crate::services::searcher::elastic::helper::send_llm_request;

use elquery::exclude_fields::ExcludeFields;
use elquery::filter_query::*;
use elquery::highlight_query::HighlightOrder;
use elquery::search_query::MultiMatchQuery;

use serde::Deserialize;
use serde_json::{json, Value};

#[async_trait::async_trait]
pub trait SearcherTrait<T: DocumentsTrait> {
    async fn build_query(s_params: &SearchParams, cxt_opts: &ContextOptions) -> Value;
    async fn extract_from_response(value: &Value) -> Result<T, WebError>;
}

#[async_trait::async_trait]
impl SearcherTrait<Document> for Document {
    async fn build_query(s_params: &SearchParams, _cxt_opts: &ContextOptions) -> Value {
        let (doc_size_from, doc_size_to) = s_params.get_doc_size();
        let (doc_cr_from, doc_cr_to) = s_params.get_doc_dates();
        let doc_ext = s_params.get_extension();
        let doc_type = s_params.get_type();
        let query = s_params.get_query();

        let common_filter = CommonFilter::new()
            .with_date::<FilterRange, CreateDateQuery>("document_created", doc_cr_from, doc_cr_to)
            .with_range::<FilterRange>("document_size", doc_size_from, doc_size_to)
            .with_term::<FilterTerm>("document_extension", doc_ext)
            .with_term::<FilterTerm>("document_type", doc_type)
            .build();

        let match_query = MultiMatchQuery::new(query);
        let highlight_order = HighlightOrder::default();

        let mut query_json_object = json!({
            "query": {
                "bool": {
                    "must": match_query,
                    "filter": common_filter
                }
            },
            "highlight": highlight_order
        });

        let cont_vector = Some(vec!["embeddings".to_string()]);
        let exclude_fields = ExcludeFields::new(cont_vector);
        let exclude_value = serde_json::to_value(exclude_fields).unwrap();
        query_json_object[&"_source"] = exclude_value;
        query_json_object
    }

    async fn extract_from_response(value: &Value) -> Result<Document, WebError> {
        let source_value = value[&"_source"].to_owned();
        let mut document = Document::deserialize(source_value)?;
        let highlight_value = value[&"highlight"].to_owned();
        let highlight_entity = HighlightEntity::deserialize(highlight_value).ok();
        document.append_highlight(highlight_entity);
        Ok(document)
    }
}

#[async_trait::async_trait]
impl SearcherTrait<DocumentPreview> for DocumentPreview {
    async fn build_query(s_params: &SearchParams, _cxt_opts: &ContextOptions) -> Value {
        let (doc_size_from, doc_size_to) = s_params.get_doc_size();
        let (doc_cr_from, doc_cr_to) = s_params.get_doc_dates();

        let common_filter = CommonFilter::new()
            .with_date::<FilterRange, CreateDateQuery>("document_created", doc_cr_from, doc_cr_to)
            .with_range::<FilterRange>("document_size", doc_size_from, doc_size_to)
            .with_match::<FilterMatch>("document_extension", s_params.get_extension())
            .build();

        // TODO: Create body by elquery!
        match s_params.get_query().is_empty() {
            true => {
                json!({
                    "query": {
                        "bool": {
                            "filter": common_filter,
                            "must": {
                                "match_all": {}
                            }
                        }
                    },
                    "sort": [
                        {
                            "document_created": {
                                "order": "desc",
                                "format": "strict_date_optional_time_nanos"
                            }
                        }
                    ],
                })
            }
            false => {
                json!({
                    "query": {
                        "bool": {
                            "filter": common_filter,
                            "should": [
                                {
                                    "multi_match": {
                                        "query": s_params.get_query(),
                                        "type": "phrase_prefix",
                                        "fields": [
                                            "document_name",
                                            "document_path"
                                        ],
                                        "minimum_should_match": "50%"
                                    }
                                }
                            ]
                        }
                    },
                    "sort": [
                        {
                            "document_created": {
                                "format": "strict_date_optional_time_nanos"
                            }
                        }
                    ],
                    "min_score": 1
                })
            }
        }
    }

    async fn extract_from_response(value: &Value) -> Result<DocumentPreview, WebError> {
        let source_value = &value[&"_source"];
        DocumentPreview::deserialize(source_value).map_err(WebError::from)
    }
}

#[async_trait::async_trait]
impl SearcherTrait<DocumentVectors> for DocumentVectors {
    async fn build_query(s_params: &SearchParams, cxt_opts: &ContextOptions) -> Value {
        let query = s_params.get_query();
        let (size, _) = s_params.get_doc_size();
        let query_vector = send_llm_request(cxt_opts, query).await;

        json!({
            "size": size,
            "knn": {
                "field": "embeddings.vector",
                "k": s_params.get_kkn_amount(),
                "num_candidates": s_params.get_candidates(),
                "query_vector": query_vector
            }
        })
    }

    async fn extract_from_response(value: &Value) -> Result<DocumentVectors, WebError> {
        let match_score = &value[&"_score"].as_f64().unwrap_or(1.0);
        let source_value = &value[&"_source"];
        let mut document = DocumentVectors::deserialize(source_value)?;
        document.set_match_score(*match_score);
        document.exclude_tokens();
        Ok(document)
    }
}

#[async_trait::async_trait]
impl SearcherTrait<InfoFolder> for InfoFolder {
    async fn build_query(s_params: &SearchParams, _: &ContextOptions) -> Value {
        // TODO: Create body by elquery!
        let filter_item = match s_params.get_show_all() {
            true => {
                let def_vec: Vec<String> = Vec::default();
                json!({"must": def_vec})
            }
            false => json!({
                "must": [
                    {
                        "term": {
                            "is_system": false
                        }
                    }
                ]
            }),
        };

        json!({
            "query": {
                "bool": {
                    "must": {
                        "match_all": {}
                    },
                    "filter": {
                        "bool": filter_item
                    }
                }
            }
        })
    }

    async fn extract_from_response(value: &Value) -> Result<InfoFolder, WebError> {
        let source_value = &value[&"_source"];
        InfoFolder::deserialize(source_value).map_err(WebError::from)
    }
}
