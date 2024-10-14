use crate::errors::WebError;
use crate::searcher::models::SearchParams;
use crate::searcher::SearcherTrait;
use crate::storage::models::{
    Document, DocumentPreview, DocumentVectors, HighlightEntity, InfoFolder,
};

use elquery::exclude::ExcludeFields;
use elquery::filter::must_filter::BoolMustFilter;
use elquery::highlight::HighlightQuery;
use elquery::r#match::{BoolQuery, BoolQueryType};
use elquery::search::multi_match_query::BoolMultiMatchQuery;
use elquery::search::must_query::BoolMustQuery;
use elquery::search::should_query::{BoolShouldQuery, MatchItemQuery, MatchItemType};
use elquery::sort::{SortItem, SortItemFormat, SortItemOrder, SortQuery};
use elquery::CommonQuery;
use serde::Deserialize;
use serde_json::{json, Value};

#[async_trait::async_trait]
impl SearcherTrait<Document> for Document {
    async fn build_query(s_params: &SearchParams) -> Value {
        let (doc_size_from, doc_size_to) = s_params.get_doc_size();
        let (doc_cr_from, doc_cr_to) = s_params.get_doc_dates();
        let doc_ext = s_params.document_extension();
        let doc_type = s_params.document_type();

        let must_filter = BoolMustFilter::default()
            .with_range("document_created", doc_cr_from, Some(doc_cr_to))
            .with_range("document_size", doc_size_from, Some(doc_size_to))
            .with_term("document_extension", doc_ext)
            .with_term("document_type", doc_type)
            .build();

        let match_fields = vec!["content".to_string(), "document_path".to_string()];
        let must_query = BoolMustQuery::default()
            .with_query(s_params.query())
            .with_operator(None)
            .with_fields(match_fields)
            .build();

        let bool_query = BoolQuery::default()
            .with_query(must_query, BoolQueryType::Must)
            .with_filter(must_filter)
            .build();

        let highlight_query = HighlightQuery::default().build();
        let exclude_query = ExcludeFields::default().with_fields(vec!["embeddings".to_string()]);

        let query = CommonQuery::builder()
            .query(bool_query)
            .sort(None)
            .min_score(None)
            .highlight(Some(highlight_query))
            ._source(Some(exclude_query))
            .build()
            .unwrap();

        serde_json::to_value(query).unwrap()
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
        let doc_ext = s_params.document_extension();
        let query = s_params.query();

        let must_filter = BoolMustFilter::default()
            .with_range("document_created", doc_cr_from, Some(doc_cr_to))
            .with_range("document_size", doc_size_from, Some(doc_size_to))
            .with_term("document_extension", doc_ext)
            .build();

        let sort_item = SortItem::default()
            .with_order(SortItemOrder::Desc)
            .with_format(SortItemFormat::StrictDateOptionalTimeNanos)
            .build();

        let sort_query = SortQuery::default()
            .with_must_field("document_created", sort_item)
            .build();

        let sort_queries = vec![serde_json::to_value(sort_query).unwrap()];

        let exclude_query = ExcludeFields::default().with_fields(vec!["embeddings".to_string()]);

        let bool_query = match query.is_empty() {
            true => BoolQuery::default()
                .with_match_all(BoolQueryType::Should)
                .with_filter(must_filter)
                .build(),
            false => {
                let fields = vec!["document_name".to_string(), "document_path".to_string()];

                let item_query = MatchItemQuery::builder()
                    .query(query.to_owned())
                    .item_type(Some(MatchItemType::PhrasePrefix))
                    .minimum_should_match("50%".to_string())
                    .fields(fields)
                    .build()
                    .unwrap();

                let multi_match_query = BoolMultiMatchQuery::default().set_item(item_query).build();

                let should_query = BoolShouldQuery::default()
                    .append_item(multi_match_query)
                    .build();

                BoolQuery::default()
                    .with_query(should_query, BoolQueryType::Should)
                    .with_filter(must_filter)
                    .build()
            }
        };

        let query = CommonQuery::builder()
            .query(bool_query)
            .min_score(None)
            .highlight(None)
            .sort(Some(sort_queries))
            ._source(Some(exclude_query))
            .build()
            .unwrap();

        serde_json::to_value(query).unwrap()
    }

    async fn extract_from_response(value: &Value) -> Result<DocumentPreview, WebError> {
        let source_value = &value[&"_source"];
        DocumentPreview::deserialize(source_value).map_err(WebError::from)
    }
}

#[async_trait::async_trait]
impl SearcherTrait<DocumentVectors> for DocumentVectors {
    async fn build_query(s_params: &SearchParams) -> Value {
        let (size, _) = s_params.get_doc_size();
        let candidates = s_params.get_candidates();
        let query_vector = s_params.query_tokens();

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
        let must_filter = BoolMustFilter::default();
        let must_filter = match s_params.is_show_all() {
            true => must_filter,
            false => must_filter.with_term("is_system", "false"),
        }
        .with_exists("folder_type")
        .build();

        let match_all_query = BoolQuery::default()
            .with_match_all(BoolQueryType::Must)
            .with_filter(must_filter)
            .build();

        let query = CommonQuery::builder()
            .query(match_all_query)
            .sort(None)
            .min_score(None)
            ._source(None)
            .highlight(None)
            .build()
            .unwrap();

        serde_json::to_value(query).unwrap()
    }

    async fn extract_from_response(value: &Value) -> Result<InfoFolder, WebError> {
        let source_value = &value[&"_source"];
        InfoFolder::deserialize(source_value).map_err(WebError::from)
    }
}
