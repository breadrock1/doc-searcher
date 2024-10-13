pub mod extractor;
pub mod helper;

use crate::elastic::ElasticClient;
use crate::errors::{Successful, WebError, WebErrorEntity, WebResult};
use crate::searcher::elastic::helper as s_helper;
use crate::searcher::forms::{DeletePaginationsForm, PaginateNextForm};
use crate::searcher::models::SearchParams;
use crate::searcher::{PaginatedResult, PaginatorService, SearcherService};
use crate::storage::forms::DocumentType;
use crate::storage::models::{Document, DocumentVectors};

use elasticsearch::{ClearScrollParts, ScrollParts};
use serde_json::Value;

#[async_trait::async_trait]
impl SearcherService for ElasticClient {
    async fn search_records(
        &self,
        s_params: &SearchParams,
        doc_type: &DocumentType,
    ) -> PaginatedResult<Value> {
        let es_client = self.es_client();
        let elastic = es_client.read().await;
        let folders = s_params.get_folders(true);
        let indexes = folders.split(',').collect::<Vec<&str>>();
        let paginated =
            s_helper::search_all::<Document>(&elastic, s_params, indexes.as_slice()).await?;
        Ok(helper::to_unified_pag(paginated, doc_type))
    }
    async fn search_fulltext(
        &self,
        s_params: &SearchParams,
        doc_type: &DocumentType,
    ) -> PaginatedResult<Value> {
        let es_client = self.es_client();
        let elastic = es_client.read().await;
        let folders = s_params.get_folders(true);
        let indexes = folders.split(',').collect::<Vec<&str>>();
        let paginated =
            s_helper::search::<Document>(&elastic, s_params, indexes.as_slice()).await?;
        Ok(helper::to_unified_pag(paginated, doc_type))
    }
    async fn search_semantic(
        &self,
        s_params: &SearchParams,
        doc_type: &DocumentType,
    ) -> PaginatedResult<Value> {
        let es_client = self.es_client();
        let elastic = es_client.read().await;
        let folders = s_params.get_folders(true);
        let indexes = folders.split(',').collect::<Vec<&str>>();
        let paginated =
            s_helper::search::<DocumentVectors>(&elastic, s_params, indexes.as_slice()).await?;

        match doc_type {
            DocumentType::GroupedVectors => Ok(helper::vec_to_grouped_value(paginated)),
            _ => Ok(helper::vec_to_value(paginated)),
        }
    }
}

#[async_trait::async_trait]
impl PaginatorService for ElasticClient {
    async fn delete_session(&self, form: &DeletePaginationsForm) -> WebResult<Successful> {
        let es_client = self.es_client();
        let elastic = es_client.read().await;
        let ids = form.get_sessions();
        let response = elastic
            .clear_scroll(ClearScrollParts::ScrollId(ids.as_slice()))
            .send()
            .await
            .map_err(WebError::from)?;

        helper::parse_elastic_response(response).await
    }

    async fn paginate(
        &self,
        scroll_form: &PaginateNextForm,
        doc_type: &DocumentType,
    ) -> PaginatedResult<Value> {
        if doc_type.is_vector_type() {
            let msg = "Can't paginate vectors search result";
            tracing::error!("Failed while paginate: {}", msg);
            let entity = WebErrorEntity::new(msg.to_string());
            return Err(WebError::PaginationError(entity));
        }

        let es_client = self.es_client();
        let elastic = es_client.read().await;
        let response_result = elastic
            .scroll(ScrollParts::ScrollId(scroll_form.get_scroll_id()))
            .pretty(true)
            .send()
            .await;

        if response_result.is_err() {
            let err = response_result.err().unwrap();
            tracing::error!("Failed to get next pagination: {}", err.to_string());
            let entity = WebErrorEntity::new(err.to_string());
            return Err(WebError::PaginationError(entity));
        }

        let response = response_result.unwrap();
        let paginated = helper::extract_elastic_response::<Document>(response).await;
        Ok(helper::to_unified_pag(paginated, doc_type))
    }
}
