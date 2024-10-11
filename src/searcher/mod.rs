pub mod elastic;
pub mod endpoints;
pub mod forms;
pub mod models;

use crate::errors::{Successful, WebError, WebResult};
use crate::searcher::forms::{DeletePaginationsForm, PaginateNextForm};
use crate::searcher::models::Paginated;
use crate::searcher::models::SearchParams;
use crate::storage::forms::DocumentType;
use crate::storage::DocumentsTrait;

use serde_json::Value;

pub type PaginatedResult<T> = Result<Paginated<Vec<T>>, WebError>;

#[async_trait::async_trait]
pub trait SearcherService {
    async fn search_records(
        &self,
        s_params: &SearchParams,
        doc_type: &DocumentType,
    ) -> PaginatedResult<Value>;
    async fn search_fulltext(
        &self,
        s_params: &SearchParams,
        doc_type: &DocumentType,
    ) -> PaginatedResult<Value>;
    async fn search_semantic(
        &self,
        s_params: &SearchParams,
        doc_type: &DocumentType,
    ) -> PaginatedResult<Value>;
}

#[async_trait::async_trait]
pub trait PaginatorService {
    async fn delete_session(&self, scroll_ids: &DeletePaginationsForm) -> WebResult<Successful>;
    async fn paginate(
        &self,
        curr_scroll: &PaginateNextForm,
        doc_type: &DocumentType,
    ) -> PaginatedResult<Value>;
}

#[async_trait::async_trait]
pub trait SearcherTrait<T: DocumentsTrait> {
    async fn build_query(s_params: &SearchParams) -> Value;
    async fn extract_from_response(value: &Value) -> Result<T, WebError>;
}
