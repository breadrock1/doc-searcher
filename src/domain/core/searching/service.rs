use crate::domain::core::searching::model::FoundedDocument;
use crate::domain::core::searching::params::{FullTextSearchParams, HybridSearchParams, PaginateParams, RetrieveParams, SemanticSearchParams};
use crate::domain::core::searching::SearchResult;

#[async_trait::async_trait]
pub trait ISearcher {
    async fn retrieve(&self, ids: &str, params: &RetrieveParams) -> SearchResult<FoundedDocument>;
    async fn fulltext(&self, params: &FullTextSearchParams) -> SearchResult<FoundedDocument>;
    async fn hybrid(&self, params: &HybridSearchParams) -> SearchResult<FoundedDocument>;
    async fn semantic(&self, params: &SemanticSearchParams) -> SearchResult<FoundedDocument>;
}

#[async_trait::async_trait]
pub trait IPaginate {
    async fn delete_session(&self, session_id: &str) -> SearchResult<()>;
    async fn paginate(&self, params: &PaginateParams) -> SearchResult<FoundedDocument>;
}
