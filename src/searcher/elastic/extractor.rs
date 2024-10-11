use crate::storage::models::{Document, DocumentPreview, DocumentVectors, HighlightEntity, InfoFolder};
use crate::errors::WebError;
use crate::searcher::models::SearchParams;
use crate::searcher::SearcherTrait;

use elquery::exclude_fields::ExcludeFields;
use elquery::filter_query::*;
use elquery::highlight_query::HighlightOrder;
use elquery::must_filter::CommonMustFilter;
use elquery::search_query::MultiMatchQuery;
use elquery::should_filter::CommonShouldFilter;
use elquery::sort_query::{SortItem, SortItemFormat, SortItemOrder, SortQuery};
use serde::Deserialize;
use serde_json::{json, Value};

#[async_trait::async_trait]
impl SearcherTrait<Document> for Document {
    async fn build_query(s_params: &SearchParams) -> Value {
        let (doc_size_from, doc_size_to) = s_params.get_doc_size();
        let (doc_cr_from, doc_cr_to) = s_params.get_doc_dates();
        let doc_ext = s_params.document_extension();
        let doc_type = s_params.document_type();
        let query = s_params.query();

        let common_filter = CommonMustFilter::default()
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
    async fn build_query(s_params: &SearchParams) -> Value {
        let (doc_size_from, doc_size_to) = s_params.get_doc_size();
        let (doc_cr_from, doc_cr_to) = s_params.get_doc_dates();
        let extensions = s_params.document_extension();

        let common_filter = CommonMustFilter::default()
            .with_date::<FilterRange, CreateDateQuery>("document_created", doc_cr_from, doc_cr_to)
            .with_range::<FilterRange>("document_size", doc_size_from, doc_size_to)
            .with_match::<FilterMatch>("document_extension", extensions)
            .build();

        let sort_item = SortItem::builder()
            .order(SortItemOrder::Desc)
            .format(SortItemFormat::StrictDateOptionalTimeNanos)
            .build()
            .unwrap();

        let sort_query = SortQuery::default()
            .with_field("document_created", sort_item)
            .build();

        let match_query = match s_params.query().is_empty() {
            true => {
                json!({
                    "must": {
                        "match_all": {}
                    }
                })
            }
            false => {
                let filter_multi_match = FilterMultiMatchItem::builder()
                    .query(s_params.query().clone())
                    .item_type(FilterMultiMatchType::PhrasePrefix)
                    .fields(vec!["document_name".to_string(), "document_path".to_string()])
                    .minimum_should_match("50%".to_string())
                    .build()
                    .unwrap();

                let filter_should = CommonShouldFilter::default()
                    .with_multi_match(filter_multi_match)
                    .build();

                serde_json::to_value(filter_should).unwrap()
            }
        };

        let search_query = json!({
            "query": {
                "bool": {
                    "filter": common_filter,
                    "match": match_query
                }
            },
            "sort": sort_query,
            "min_score": 1
        });

        search_query

        // TODO: Create body by elquery!
        // match s_params.query().is_empty() {
        //     true => {
        //         json!({
        //             "query": {
        //                 "bool": {
        //                     "filter": common_filter,
        //                     "must": {
        //                         "match_all": {}
        //                     }
        //                 }
        //             },
        //             "sort": [
        //                 {
        //                     "document_created": {
        //                         "order": "desc",
        //                         "format": "StrictDateOptionalTimeNanos"
        //                     }
        //                 }
        //             ],
        //         })
        //     }
        //     false => {
        //         json!({
        //             "query": {
        //                 "bool": {
        //                     "filter": common_filter,
        //                     "should": [
        //                         {
        //                             "multi_match": {
        //                                 "query": s_params.query(),
        //                                 "type": "phrase_prefix",
        //                                 "fields": [
        //                                     "document_name",
        //                                     "document_path"
        //                                 ],
        //                                 "minimum_should_match": "50%"
        //                             }
        //                         }
        //                     ]
        //                 }
        //             },
        //             "sort": [
        //                 {
        //                     "document_created": {
        //                         "order": "desc",
        //                         "format": "strict_date_optional_time_nanos"
        //                     }
        //                 }
        //             ],
        //             "min_score": 1
        //         })
        //     }
        // }
    }

    async fn extract_from_response(value: &Value) -> Result<DocumentPreview, WebError> {
        let source_value = &value[&"_source"];
        DocumentPreview::deserialize(source_value).map_err(WebError::from)
    }
}

#[async_trait::async_trait]
impl SearcherTrait<DocumentVectors> for DocumentVectors {
    async fn build_query(s_params: &SearchParams) -> Value {
        let query = s_params.query();
        let (size, _) = s_params.get_doc_size();
        let query_vector: Vec<f64> = Vec::default();
        let candidates = s_params.get_candidates();

        json!({
            "size": size,
            "knn": {
                "field": "embeddings.vector",
                "k": s_params.get_kkn_amount(),
                "num_candidates": candidates,
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
    async fn build_query(s_params: &SearchParams) -> Value {
        let is_system_value = match s_params.is_show_all() {
            true => "",
            false => "false",
        };

        let common_filter = CommonMustFilter::default()
            .with_exists::<FilterExists>("folder_type")
            .with_term::<FilterTerm>("is_system", is_system_value)
            .build();

        json!({
            "query": {
                "bool": {
                    "must": {
                        "match_all": {}
                    },
                    "filter": common_filter
                }
            }
        })
    }

    async fn extract_from_response(value: &Value) -> Result<InfoFolder, WebError> {
        let source_value = &value[&"_source"];
        InfoFolder::deserialize(source_value).map_err(WebError::from)
    }
}
