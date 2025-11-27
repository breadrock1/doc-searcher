use crate::domain::searcher::SearchResult;
use crate::domain::searcher::models::PaginationParams;
use crate::domain::searcher::models::{Pagination, SearchingParams};

#[async_trait::async_trait]
pub trait ISearcher {
    async fn search(&self, params: &SearchingParams) -> SearchResult<Pagination>;
}

#[async_trait::async_trait]
pub trait IPaginator {
    async fn paginate(&self, params: &PaginationParams) -> SearchResult<Pagination>;
}
