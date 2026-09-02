use mockall::mock;

use crate::domain::searcher::models::{Pagination, PaginationParams, SearchingParams};
use crate::domain::searcher::{IPaginator, ISearcher, SearchError};

mock! {
    pub Searcher {}

    impl Clone for Searcher {
        fn clone(&self) -> Self;
    }

    #[async_trait::async_trait]
    impl ISearcher for Searcher {
        async fn search(&self, params: &SearchingParams) -> Result<Pagination, SearchError>;
    }

    #[async_trait::async_trait]
    impl IPaginator for Searcher {
        async fn paginate(&self, params: &PaginationParams) -> Result<Pagination, SearchError>;
    }
}
