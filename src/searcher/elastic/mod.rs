mod converter;
pub mod extractor;
pub(crate) mod search;

use crate::elastic::ElasticClient;
use crate::errors::Successful;
use crate::searcher::elastic::extractor::SearchQueryBuilder;
use crate::searcher::elastic::search::Searcher;
use crate::searcher::errors::{PaginatedResult, SearcherError, SearcherResult};
use crate::searcher::forms::DocumentType;
use crate::searcher::forms::{DeleteScrollsForm, ScrollNextForm};
use crate::searcher::forms::{FulltextParams, SemanticParams};
use crate::searcher::{PaginatorService, SearcherService};
use crate::storage::models::{Document, DocumentVectors};

use elasticsearch::{ClearScrollParts, ScrollParts};
use serde_json::Value;

#[async_trait::async_trait]
impl SearcherService for ElasticClient {
    async fn search_fulltext(
        &self,
        params: &FulltextParams,
        return_as: &DocumentType,
    ) -> PaginatedResult<Value> {
        let es = self.es_client();
        let query = Document::build_search_query(params).await;
        let founded = Document::search(es, &query, params).await?;
        Ok(converter::to_unified_paginated(founded, return_as))
    }

    async fn search_semantic(&self, params: &SemanticParams) -> PaginatedResult<Value> {
        let es = self.es_client();
        let query = DocumentVectors::build_search_query(params).await;
        let founded = DocumentVectors::search(es, &query, params).await?;

        if params.is_grouped().unwrap_or_default() {
            tracing::info!("grouping semantic searching results");
            return Ok(converter::vec_to_grouped_value(founded));
        }

        Ok(converter::vec_to_value(founded))
    }
}

#[async_trait::async_trait]
impl PaginatorService for ElasticClient {
    async fn delete_session(&self, form: &DeleteScrollsForm) -> SearcherResult<Successful> {
        let ids = form
            .sessions()
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>();

        let es_client = self.es_client();
        let elastic = es_client.read().await;
        let response = elastic
            .clear_scroll(ClearScrollParts::ScrollId(ids.as_slice()))
            .send()
            .await?;

        let response = ElasticClient::extract_response_msg(response).await?;
        Ok(response)
    }

    async fn paginate(
        &self,
        scroll_form: &ScrollNextForm,
        doc_type: &DocumentType,
    ) -> PaginatedResult<Value> {
        if let DocumentType::Vectors = doc_type {
            let msg = "Can't paginate vectors search result";
            tracing::warn!("Failed while paginate: {msg}");
            return Err(SearcherError::ServiceError(msg.to_string()));
        }

        let es_client = self.es_client();
        let elastic = es_client.read().await;
        let response = elastic
            .scroll(ScrollParts::ScrollId(scroll_form.scroll_id()))
            .pretty(true)
            .send()
            .await?;

        let paginated = search::extract_searcher_result::<Document>(response).await?;
        Ok(converter::to_unified_paginated(paginated, doc_type))
    }
}
