use metrics::{counter, histogram};
use std::sync::Arc;
use tracing::instrument;

use crate::application::services::embeddings::ITokenizer;
use crate::application::usecase::tokens::EnsuredTokens;
use crate::domain::searcher::models::{
    Pagination, PaginationParams, SearchKindParams, SearchingParams,
};
use crate::domain::searcher::{IPaginator, ISearcher, SearchError, SearchResult, validate_path};

#[derive(Clone)]
pub struct SearcherUseCase<Searcher>
where
    Searcher: ISearcher + IPaginator + Send + Sync,
{
    searcher: Arc<Searcher>,
    embeddings: Arc<dyn ITokenizer>,
}

impl<Searcher> SearcherUseCase<Searcher>
where
    Searcher: ISearcher + IPaginator + Send + Sync,
{
    pub fn new(searcher: Arc<Searcher>, embeddings: Arc<dyn ITokenizer>) -> Self {
        SearcherUseCase {
            searcher,
            embeddings,
        }
    }
}

impl<Searcher> SearcherUseCase<Searcher>
where
    Searcher: ISearcher + IPaginator + Send + Sync,
{
    #[instrument(level = "info", skip(self))]
    pub async fn search_document_parts(&self, params: SearchingParams) -> SearchResult<Pagination> {
        let instant = tokio::time::Instant::now();

        // The search path / folder prefix is used verbatim as a filter in the
        // OpenSearch query, so an invalid path (traversal, absolute, root)
        // must be rejected before it reaches the search layer. This applies to
        // every search kind: retrieve (path), fulltext, semantic and hybrid.
        if let Some(prefix) = params.get_prefix() {
            validate_path(prefix)
                .map_err(|err| SearchError::ValidationError(anyhow::anyhow!(err)))?;
        }

        let searching_kind = params.get_kind().to_string();

        let ensured = self.ensure_query_embeddings(params).await?;
        let result = self.searcher.search(ensured.params, ensured.tokens).await;

        let is_error = result.is_err();
        counter!(
            "docsearch_searching_operations_total",
            "searching_kind" => searching_kind.clone(),
            "searching_status" => is_error.to_string(),
        )
        .increment(1);

        histogram!(
            "docsearch_searching_duration_seconds",
            "searching_kind" => searching_kind,
            "searching_status" => is_error.to_string(),
        )
        .record(instant.elapsed().as_secs_f64());

        let pagination = result?;
        Ok(pagination)
    }

    #[instrument(level = "info", skip(self))]
    pub async fn paginate(&self, params: PaginationParams) -> SearchResult<Pagination> {
        let instant = tokio::time::Instant::now();
        let result = self.searcher.paginate(params).await;

        let is_error = result.is_err();
        counter!(
            "docsearch_paginating_operations_total",
            "paginating_status" => is_error.to_string(),
        )
        .increment(1);
        histogram!(
            "docsearch_paginating_duration_seconds",
            "paginating_status" => is_error.to_string(),
        )
        .record(instant.elapsed().as_secs_f64());

        let pagination = result?;
        Ok(pagination)
    }

    /// Computes the query embedding via the embeddings service for semantic/hybrid
    /// search kinds and returns it together with the (unchanged) params.
    ///
    /// An embeddings service error is propagated as [`SearchError::InternalError`],
    /// so the search backend is never touched on failure. For retrieve/fulltext
    /// search kinds no embedding is computed and `tokens` is `None`. Empty
    /// embeddings are still passed through — `build_knn_query` degrades them to
    /// `match_none`, so the full-text part of a hybrid search keeps working.
    async fn ensure_query_embeddings(
        &self,
        params: SearchingParams,
    ) -> SearchResult<EnsuredTokens> {
        match &params.kind {
            SearchKindParams::Semantic(par) => {
                let tokens = self
                    .embeddings
                    .compute(&par.query)
                    .await
                    .map_err(|err| SearchError::InternalError(err.into()))?;

                Ok(EnsuredTokens::new(params, Some(tokens)))
            }
            SearchKindParams::Hybrid(par) => {
                let tokens = self
                    .embeddings
                    .compute(&par.query)
                    .await
                    .map_err(|err| SearchError::InternalError(err.into()))?;

                Ok(EnsuredTokens::new(params, Some(tokens)))
            }
            SearchKindParams::Retrieve(_) | SearchKindParams::FullText(_) => {
                Ok(EnsuredTokens::new(params, None))
            }
        }
    }
}
