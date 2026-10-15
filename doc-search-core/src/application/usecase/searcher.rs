use metrics::{counter, histogram};
use std::sync::Arc;
use tracing::instrument;

use crate::application::services::embeddings::ITokenizer;
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
        let params = match self.ensure_query_tokens(params).await? {
            Some(params) => params,
            None => return Ok(Pagination::new(None, vec![])),
        };
        let result = self.searcher.search(params).await;

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

    async fn ensure_query_tokens(
        &self,
        mut params: SearchingParams,
    ) -> SearchResult<Option<SearchingParams>> {
        let search_kind = &mut params.kind;

        let skip_search_by_tokens = match search_kind {
            SearchKindParams::Semantic(par) => {
                // Клиент уже передал готовые токены — провайдер не нужен.
                if par.tokens.is_some() {
                    false
                } else {
                    let tokens = self
                        .embeddings
                        .compute(&par.query)
                        .await
                        .unwrap_or_default();

                    // Без векторов семантический поиск бессмысленен — вернём пустой результат.
                    if tokens.is_empty() {
                        true
                    } else {
                        par.tokens = Some(tokens);
                        false
                    }
                }
            }
            SearchKindParams::Hybrid(par) => {
                if par.tokens.is_none() {
                    let tokens = self
                        .embeddings
                        .compute(&par.query)
                        .await
                        .unwrap_or_default();

                    par.tokens = Some(tokens);
                }
                false
            }
            _ => false,
        };

        if skip_search_by_tokens {
            Ok(None)
        } else {
            Ok(Some(params))
        }
    }
}
