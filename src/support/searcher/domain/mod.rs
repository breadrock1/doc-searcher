mod error;
pub use error::{SearchResult, SearchError};

mod model;
pub use model::Paginated;
pub use model::{FoundedDocument, FoundedDocumentBuilder, FoundedDocumentBuilderError};

mod params;
pub use params::{FilterParams, FilterParamsBuilder, FilterParamsBuilderError};
pub use params::{ResultParams, ResultParamsBuilder, ResultParamsBuilderError};
pub use params::{RetrieveParams, RetrieveParamsBuilder, RetrieveParamsBuilderError};
pub use params::{FullTextSearchParams, FullTextSearchParamsBuilder, FullTextSearchParamsBuilderError};
pub use params::{HybridSearchParams, HybridSearchParamsBuilder, HybridSearchParamsBuilderError};
pub use params::{SemanticSearchParams, SemanticSearchParamsBuilder, SemanticSearchParamsBuilderError};
pub use params::{PaginateParams, PaginateParamsBuilder, PaginateParamsBuilderError};

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
