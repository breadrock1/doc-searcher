use elasticsearch::{ClearScrollParts, ScrollParts};
use serde_json::Value;

use crate::engine::elastic::helper::converter;
use crate::engine::elastic::helper::extractor::SearchQueryBuilder;
use crate::engine::elastic::ops::search;
use crate::engine::elastic::ops::search::Searcher;
use crate::engine::elastic::ElasticClient;
use crate::engine::error::{PaginatedResult, SearcherError, SearcherResult};
use crate::engine::form::{
    DeleteScrollsForm, DocumentType, FulltextParams, ScrollNextForm, SemanticParams,
};
use crate::engine::model::{Document, DocumentVectors};
use crate::engine::{PaginatorService, SearcherService};
use crate::errors::Successful;

#[async_trait::async_trait]
impl SearcherService for ElasticClient {
    async fn search_fulltext(
        &self,
        params: &FulltextParams,
        return_as: &DocumentType,
    ) -> PaginatedResult<Value> {
        let es = self.client.clone();
        let query = Document::build_search_query(params).await;
        let founded = Document::search(es, &query, params).await?;
        Ok(converter::to_unified_paginated(founded, return_as))
    }

    async fn search_semantic(&self, params: &SemanticParams) -> PaginatedResult<Value> {
        let es = self.client.clone();
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

        let es_client = self.client.clone();
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
            let msg = "can't paginate vectors search result";
            tracing::warn!(details = msg, "failed while paginating");
            return Err(SearcherError::RuntimeError(msg.to_string()));
        }

        let es_client = self.client.clone();
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
