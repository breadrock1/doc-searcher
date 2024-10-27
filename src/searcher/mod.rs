pub mod elastic;
pub mod endpoints;
pub mod errors;
pub mod forms;
pub mod models;

use crate::errors::Successful;
use crate::searcher::errors::{PaginatedResult, SearcherResult};
use crate::searcher::forms::DocumentType;
use crate::searcher::forms::{DeleteScrollsForm, ScrollNextForm};
use crate::searcher::forms::{FulltextParams, SemanticParams};

use serde_json::Value;

#[async_trait::async_trait]
pub trait SearcherService {
    async fn search_fulltext(
        &self,
        params: &FulltextParams,
        return_as: &DocumentType,
    ) -> PaginatedResult<Value>;

    async fn search_semantic(&self, params: &SemanticParams) -> PaginatedResult<Value>;
}

#[async_trait::async_trait]
pub trait PaginatorService {
    async fn delete_session(&self, form: &DeleteScrollsForm) -> SearcherResult<Successful>;

    async fn paginate(
        &self,
        form: &ScrollNextForm,
        doc_type: &DocumentType,
    ) -> PaginatedResult<Value>;
}
