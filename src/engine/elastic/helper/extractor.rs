use elquery::exclude::ExcludeFields;
use elquery::filter::must_filter::BoolMustFilter;
use elquery::highlight::HighlightQuery;
use elquery::r#match::{BoolQuery, BoolQueryType};
use elquery::search::must_query::BoolMustQuery;
use elquery::CommonQuery;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::engine::error::SearcherResult;
use crate::engine::form::{FulltextParams, SemanticParams};
use crate::engine::model::{Document, DocumentVectors, DocumentsTrait, HighlightEntity};

#[async_trait::async_trait]
pub trait SearchQueryBuilder<T: DocumentsTrait> {
    type Params;

    async fn build_search_query(s_params: &Self::Params) -> Value;
    async fn extract_from_response(value: &Value) -> SearcherResult<T>;
}

#[async_trait::async_trait]
impl SearchQueryBuilder<Document> for Document {
    type Params = FulltextParams;

    async fn build_search_query(params: &Self::Params) -> Value {
        let (doc_size_from, doc_size_to) = params.document_size();
        let (doc_cr_from, doc_cr_to) = params.document_dates();
        let doc_ext = params.document_extension().clone();
        let doc_type = params.document_type().clone();

        let must_filter = BoolMustFilter::default()
            .with_range("document_created", doc_cr_from, doc_cr_to)
            .with_range("document_size", doc_size_from, doc_size_to)
            .with_term("document_extension", doc_ext)
            .with_term("document_type", doc_type)
            .build();

        let match_fields = vec!["content".to_string(), "document_path".to_string()];
        let must_query = BoolMustQuery::default()
            .with_query(params.query())
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

    async fn extract_from_response(value: &Value) -> SearcherResult<Document> {
        let source_value = value[&"_source"].to_owned();
        let mut document = Document::deserialize(source_value)?;
        let highlight_value = value[&"highlight"].to_owned();
        let highlight_entity = HighlightEntity::deserialize(highlight_value).ok();
        document.set_highlight(highlight_entity);
        Ok(document)
    }
}

#[async_trait::async_trait]
impl SearchQueryBuilder<DocumentVectors> for DocumentVectors {
    type Params = SemanticParams;

    async fn build_search_query(s_params: &Self::Params) -> Value {
        let size = s_params.result_size();
        let candidates = s_params.candidates();
        let knn_amount = s_params.knn_amount();
        let query_vector = s_params.query_tokens();

        json!({
            "size": size,
            "knn": {
                "field": "embeddings.vector",
                "k": knn_amount,
                "num_candidates": candidates,
                "query_vector": query_vector
            }
        })
    }

    async fn extract_from_response(value: &Value) -> SearcherResult<DocumentVectors> {
        let match_score = &value[&"_score"].as_f64().unwrap_or(1.0);
        let source_value = &value[&"_source"];
        let mut document = DocumentVectors::deserialize(source_value)?;
        document.set_match_score(Some(*match_score));
        document.exclude_tokens();
        Ok(document)
    }
}
