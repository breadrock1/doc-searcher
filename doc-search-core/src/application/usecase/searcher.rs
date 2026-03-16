use metrics::{counter, histogram};
use std::sync::Arc;
use tracing::{Instrument, instrument};

use crate::domain::searcher::SearchResult;
use crate::domain::searcher::models::{Pagination, PaginationParams, SearchingParams};
use crate::domain::searcher::{IPaginator, ISearcher};

#[derive(Clone)]
pub struct SearcherUseCase<Searcher>
where
    Searcher: ISearcher + IPaginator + Send + Sync,
{
    searcher: Arc<Searcher>,
}

impl<Searcher> SearcherUseCase<Searcher>
where
    Searcher: ISearcher + IPaginator + Send + Sync,
{
    pub fn new(searcher: Arc<Searcher>) -> Self {
        SearcherUseCase { searcher }
    }
}

impl<Searcher> SearcherUseCase<Searcher>
where
    Searcher: ISearcher + IPaginator + Send + Sync,
{
    #[instrument(level = "info", skip(self), ret(Debug))]
    pub async fn search_document_parts(
        &self,
        params: &SearchingParams,
    ) -> SearchResult<Pagination> {
        let instant = tokio::time::Instant::now();
        let result = self.searcher.search(params).await;

        let is_error = result.is_err();
        let searching_kind = params.get_kind().to_string();
        counter!(
            "searching_operations_total",
            "searching_kind" => searching_kind.clone(),
            "searching_status" => is_error.to_string(),
        )
        .increment(1);

        histogram!(
            "searching_duration_seconds",
            "searching_kind" => searching_kind,
            "searching_status" => is_error.to_string(),
        )
        .record(instant.elapsed().as_secs_f64());

        let pagination = result?;
        Ok(pagination)
    }

    #[instrument(level = "info", skip(self), ret(Debug))]
    pub async fn load_next_pagination(
        &self,
        params: &PaginationParams,
    ) -> SearchResult<Pagination> {
        let pagination = self.searcher.paginate(params).await?;

        Ok(pagination)
    }
}
