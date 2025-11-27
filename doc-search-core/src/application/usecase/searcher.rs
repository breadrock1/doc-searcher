use std::sync::Arc;
use tracing::Instrument;

use crate::domain::searcher::SearchResult;
use crate::domain::searcher::models::{Pagination, PaginationParams, SearchingParams};
use crate::domain::searcher::{IPaginator, ISearcher};

#[derive(Clone)]
pub struct SearcherUseCase<Searcher>
where
    Searcher: ISearcher + IPaginator + Send + Sync + Clone,
{
    searcher: Arc<Searcher>,
}

impl<Searcher> SearcherUseCase<Searcher>
where
    Searcher: ISearcher + IPaginator + Send + Sync + Clone,
{
    pub fn new(searcher: Arc<Searcher>) -> Self {
        SearcherUseCase { searcher }
    }
}

impl<Searcher> SearcherUseCase<Searcher>
where
    Searcher: ISearcher + IPaginator + Send + Sync + Clone,
{
    pub async fn search_document_parts(
        &self,
        params: &SearchingParams,
    ) -> SearchResult<Pagination> {
        let pagination = self
            .searcher
            .search(params)
            .instrument(tracing::info_span!("search-document-parts"))
            .await?;

        Ok(pagination)
    }

    pub async fn load_next_pagination(
        &self,
        params: &PaginationParams,
    ) -> SearchResult<Pagination> {
        let pagination = self
            .searcher
            .paginate(params)
            .instrument(tracing::info_span!("load-next-pagination"))
            .await?;

        Ok(pagination)
    }
}
