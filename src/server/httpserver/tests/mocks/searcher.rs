use mockall::mock;

use doc_search_core::domain::searcher::models::{Pagination, PaginationParams, SearchingParams};
use doc_search_core::domain::searcher::SearchError;
use doc_search_core::domain::searcher::{IPaginator, ISearcher};

mock! {
    pub SearcherService {}

    #[async_trait::async_trait]
    impl ISearcher for SearcherService {
        async fn search(&self, params: &SearchingParams) -> Result<Pagination, SearchError>;
    }

    #[async_trait::async_trait]
    impl IPaginator for SearcherService {
        async fn paginate(&self, params: &PaginationParams) -> Result<Pagination, SearchError>;
    }
}
