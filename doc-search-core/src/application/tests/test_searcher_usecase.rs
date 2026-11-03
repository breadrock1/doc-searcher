use anyhow::{Context, anyhow};
use rstest::rstest;
use std::sync::Arc;

use crate::application::services::embeddings::error::TokenizerError;
use crate::application::tests::mock::searcher::MockSearcher;
use crate::application::tests::mock::tokenizer::MockEmbeddingsProvider;
use crate::application::usecase::searcher::SearcherUseCase;
use crate::domain::searcher::SearchError;
use crate::domain::searcher::models::{
    FullTextSearchingParamsBuilder, HybridSearchingParamsBuilder, Pagination,
    PaginationParamsBuilder, RetrieveIndexDocumentsParamsBuilder, SearchKindParams,
    SearchingParams, SemanticSearchingParamsBuilder,
};
use crate::domain::searcher::tests::fixture::params::{
    build_full_text_searching_params, build_hybrid_searching_params, build_result_searching_params,
    build_semantic_searching_params,
};
use crate::shared::kernel::Tokens;

#[rstest]
#[tokio::test]
async fn test_search_document_parts_success(
    #[from(build_full_text_searching_params)] params: SearchingParams,
) -> anyhow::Result<()> {
    let mut mock = MockSearcher::new();
    mock.expect_search()
        .times(1)
        .returning(|_, _| Ok(Pagination::new(None, vec![])));

    let usecase = SearcherUseCase::new(Arc::new(mock), Arc::new(MockEmbeddingsProvider::new()));
    let result = usecase.search_document_parts(params).await;

    assert!(result.is_ok());
    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_search_document_parts_error(
    #[from(build_full_text_searching_params)] params: SearchingParams,
) -> anyhow::Result<()> {
    let mut mock = MockSearcher::new();
    mock.expect_search()
        .times(1)
        .returning(|_, _| Err(SearchError::InternalError(anyhow::anyhow!("boom"))));

    let usecase = SearcherUseCase::new(Arc::new(mock), Arc::new(MockEmbeddingsProvider::new()));
    let result = usecase.search_document_parts(params).await;

    assert!(result.is_err());
    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_load_next_pagination_success() -> anyhow::Result<()> {
    let mut mock = MockSearcher::new();
    mock.expect_paginate()
        .times(1)
        .returning(|_| Ok(Pagination::new(Some("next-scroll".to_string()), vec![])));

    let usecase = SearcherUseCase::new(Arc::new(mock), Arc::new(MockEmbeddingsProvider::new()));
    let params = PaginationParamsBuilder::default()
        .scroll_id("scroll-value".to_string())
        .build()
        .context("failed to build pagination params")?;

    let result = usecase.paginate(params).await;

    let pagination = result.expect("expected pagination");
    assert_eq!(Some("next-scroll".to_string()), pagination.scroll_id);
    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_load_next_pagination_error() -> anyhow::Result<()> {
    let mut mock = MockSearcher::new();
    mock.expect_paginate()
        .times(1)
        .returning(|_| Err(SearchError::ServiceError(anyhow::anyhow!("boom"))));

    let usecase = SearcherUseCase::new(Arc::new(mock), Arc::new(MockEmbeddingsProvider::new()));
    let params = PaginationParamsBuilder::default()
        .scroll_id("scroll-value".to_string())
        .build()
        .context("failed to build pagination params")?;

    let result = usecase.paginate(params).await;

    assert!(result.is_err());
    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_semantic_computes_embeddings_and_searches(
    #[from(build_semantic_searching_params)] params: SearchingParams,
) -> anyhow::Result<()> {
    let mut embeddings = MockEmbeddingsProvider::new();
    embeddings
        .expect_compute()
        .times(1)
        .returning(|_| Ok(vec![1.0, 2.0, 3.0].into()));

    let mut mock = MockSearcher::new();
    mock.expect_search()
        .times(1)
        .withf(|_: &SearchingParams, qe: &Option<Tokens>| *qe == Some(vec![1.0, 2.0, 3.0].into()))
        .returning(|_, _| Ok(Pagination::new(None, vec![])));

    let usecase = SearcherUseCase::new(Arc::new(mock), Arc::new(embeddings));
    assert!(usecase.search_document_parts(params).await.is_ok());
    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_semantic_provider_error_returns_internal_error(
    #[from(build_semantic_searching_params)] params: SearchingParams,
) -> anyhow::Result<()> {
    let mut embeddings = MockEmbeddingsProvider::new();
    embeddings
        .expect_compute()
        .times(1)
        .returning(|_| Err(TokenizerError::Request(anyhow!("unavailable"))));

    let mut mock = MockSearcher::new();
    mock.expect_search().never();

    let usecase = SearcherUseCase::new(Arc::new(mock), Arc::new(embeddings));
    let result = usecase.search_document_parts(params).await;
    assert!(matches!(result, Err(SearchError::InternalError(_))));
    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_hybrid_computes_embeddings_and_searches(
    #[from(build_hybrid_searching_params)] params: SearchingParams,
) -> anyhow::Result<()> {
    let mut embeddings = MockEmbeddingsProvider::new();
    embeddings
        .expect_compute()
        .times(1)
        .returning(|_| Ok(vec![1.0, 2.0, 3.0].into()));

    let mut mock = MockSearcher::new();
    mock.expect_search()
        .times(1)
        .withf(|_: &SearchingParams, qe: &Option<Tokens>| *qe == Some(vec![1.0, 2.0, 3.0].into()))
        .returning(|_, _| Ok(Pagination::new(None, vec![])));

    let usecase = SearcherUseCase::new(Arc::new(mock), Arc::new(embeddings));
    assert!(usecase.search_document_parts(params).await.is_ok());
    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_hybrid_empty_embeddings_still_searches(
    #[from(build_hybrid_searching_params)] params: SearchingParams,
) -> anyhow::Result<()> {
    let mut embeddings = MockEmbeddingsProvider::new();
    embeddings
        .expect_compute()
        .times(1)
        .returning(|_| Ok(Tokens::default()));

    let mut mock = MockSearcher::new();
    mock.expect_search()
        .times(1)
        .withf(|_: &SearchingParams, qe: &Option<Tokens>| qe.is_some())
        .returning(|_, _| Ok(Pagination::new(None, vec![])));

    let usecase = SearcherUseCase::new(Arc::new(mock), Arc::new(embeddings));
    let pagination = usecase.search_document_parts(params).await?;
    assert!(pagination.founded.is_empty());
    assert!(pagination.scroll_id.is_none());
    Ok(())
}

#[tokio::test]
async fn test_retrieve_rejects_traversal_path() -> anyhow::Result<()> {
    let params = SearchingParams::new(
        vec!["test-index-1".into()],
        SearchKindParams::Retrieve(
            RetrieveIndexDocumentsParamsBuilder::default()
                .path(Some("user-123/../../secret.txt".to_string()))
                .build()?,
        ),
        build_result_searching_params(),
        None,
    );

    let mut mock = MockSearcher::new();
    mock.expect_search().never();

    let usecase = SearcherUseCase::new(Arc::new(mock), Arc::new(MockEmbeddingsProvider::new()));
    let result = usecase.search_document_parts(params).await;

    assert!(matches!(result, Err(SearchError::ValidationError(_))));
    Ok(())
}

#[tokio::test]
async fn test_retrieve_rejects_absolute_path() -> anyhow::Result<()> {
    let params = SearchingParams::new(
        vec!["test-index-1".into()],
        SearchKindParams::Retrieve(
            RetrieveIndexDocumentsParamsBuilder::default()
                .path(Some("/etc/passwd".to_string()))
                .build()?,
        ),
        build_result_searching_params(),
        None,
    );

    let mut mock = MockSearcher::new();
    mock.expect_search().never();

    let usecase = SearcherUseCase::new(Arc::new(mock), Arc::new(MockEmbeddingsProvider::new()));
    let result = usecase.search_document_parts(params).await;

    assert!(matches!(result, Err(SearchError::ValidationError(_))));
    Ok(())
}

#[tokio::test]
async fn test_retrieve_with_valid_path_searches() -> anyhow::Result<()> {
    let params = SearchingParams::new(
        vec!["test-index-1".into()],
        SearchKindParams::Retrieve(
            RetrieveIndexDocumentsParamsBuilder::default()
                .path(Some("user-123/doc.txt".to_string()))
                .build()?,
        ),
        build_result_searching_params(),
        None,
    );

    let mut mock = MockSearcher::new();
    mock.expect_search()
        .times(1)
        .returning(|_, _| Ok(Pagination::new(None, vec![])));

    let usecase = SearcherUseCase::new(Arc::new(mock), Arc::new(MockEmbeddingsProvider::new()));
    assert!(usecase.search_document_parts(params).await.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_fulltext_rejects_traversal_prefix() -> anyhow::Result<()> {
    let params = SearchingParams::new(
        vec!["test-index-1".into()],
        SearchKindParams::FullText(
            FullTextSearchingParamsBuilder::default()
                .query(Some("query".to_string()))
                .prefix(Some("user-123/../../secret.txt".to_string()))
                .build()?,
        ),
        build_result_searching_params(),
        None,
    );

    let mut mock = MockSearcher::new();
    mock.expect_search().never();

    let usecase = SearcherUseCase::new(Arc::new(mock), Arc::new(MockEmbeddingsProvider::new()));
    let result = usecase.search_document_parts(params).await;

    assert!(matches!(result, Err(SearchError::ValidationError(_))));
    Ok(())
}

#[tokio::test]
async fn test_fulltext_rejects_absolute_prefix() -> anyhow::Result<()> {
    let params = SearchingParams::new(
        vec!["test-index-1".into()],
        SearchKindParams::FullText(
            FullTextSearchingParamsBuilder::default()
                .query(Some("query".to_string()))
                .prefix(Some("/etc/passwd".to_string()))
                .build()?,
        ),
        build_result_searching_params(),
        None,
    );

    let mut mock = MockSearcher::new();
    mock.expect_search().never();

    let usecase = SearcherUseCase::new(Arc::new(mock), Arc::new(MockEmbeddingsProvider::new()));
    let result = usecase.search_document_parts(params).await;

    assert!(matches!(result, Err(SearchError::ValidationError(_))));
    Ok(())
}

#[tokio::test]
async fn test_fulltext_with_valid_prefix_searches() -> anyhow::Result<()> {
    let params = SearchingParams::new(
        vec!["test-index-1".into()],
        SearchKindParams::FullText(
            FullTextSearchingParamsBuilder::default()
                .query(Some("query".to_string()))
                .prefix(Some("user-123/doc.txt".to_string()))
                .build()?,
        ),
        build_result_searching_params(),
        None,
    );

    let mut mock = MockSearcher::new();
    mock.expect_search()
        .times(1)
        .returning(|_, _| Ok(Pagination::new(None, vec![])));

    let usecase = SearcherUseCase::new(Arc::new(mock), Arc::new(MockEmbeddingsProvider::new()));
    assert!(usecase.search_document_parts(params).await.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_semantic_rejects_traversal_prefix() -> anyhow::Result<()> {
    let params = SearchingParams::new(
        vec!["test-index-1".into()],
        SearchKindParams::Semantic(
            SemanticSearchingParamsBuilder::default()
                .query("query".to_string())
                .knn_amount(10)
                .min_score(None)
                .prefix(Some("user-123/../../secret.txt".to_string()))
                .build()?,
        ),
        build_result_searching_params(),
        None,
    );

    let mut mock = MockSearcher::new();
    mock.expect_search().never();

    let usecase = SearcherUseCase::new(Arc::new(mock), Arc::new(MockEmbeddingsProvider::new()));
    let result = usecase.search_document_parts(params).await;

    assert!(matches!(result, Err(SearchError::ValidationError(_))));
    Ok(())
}

#[tokio::test]
async fn test_semantic_with_valid_prefix_searches() -> anyhow::Result<()> {
    let params = SearchingParams::new(
        vec!["test-index-1".into()],
        SearchKindParams::Semantic(
            SemanticSearchingParamsBuilder::default()
                .query("query".to_string())
                .knn_amount(10)
                .min_score(None)
                .prefix(Some("user-123/doc.txt".to_string()))
                .build()?,
        ),
        build_result_searching_params(),
        None,
    );

    let mut embeddings = MockEmbeddingsProvider::new();
    embeddings
        .expect_compute()
        .times(1)
        .returning(|_| Ok(vec![1.0, 2.0, 3.0].into()));

    let mut mock = MockSearcher::new();
    mock.expect_search()
        .times(1)
        .returning(|_, _| Ok(Pagination::new(None, vec![])));

    let usecase = SearcherUseCase::new(Arc::new(mock), Arc::new(embeddings));
    assert!(usecase.search_document_parts(params).await.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_hybrid_rejects_traversal_prefix() -> anyhow::Result<()> {
    let params = SearchingParams::new(
        vec!["test-index-1".into()],
        SearchKindParams::Hybrid(
            HybridSearchingParamsBuilder::default()
                .query("query".to_string())
                .knn_amount(10)
                .min_score(None)
                .prefix(Some("user-123/../../secret.txt".to_string()))
                .build()?,
        ),
        build_result_searching_params(),
        None,
    );

    let mut mock = MockSearcher::new();
    mock.expect_search().never();

    let usecase = SearcherUseCase::new(Arc::new(mock), Arc::new(MockEmbeddingsProvider::new()));
    let result = usecase.search_document_parts(params).await;

    assert!(matches!(result, Err(SearchError::ValidationError(_))));
    Ok(())
}

#[tokio::test]
async fn test_hybrid_with_valid_prefix_searches() -> anyhow::Result<()> {
    let params = SearchingParams::new(
        vec!["test-index-1".into()],
        SearchKindParams::Hybrid(
            HybridSearchingParamsBuilder::default()
                .query("query".to_string())
                .knn_amount(10)
                .min_score(None)
                .prefix(Some("user-123/doc.txt".to_string()))
                .build()?,
        ),
        build_result_searching_params(),
        None,
    );

    let mut embeddings = MockEmbeddingsProvider::new();
    embeddings
        .expect_compute()
        .times(1)
        .returning(|_| Ok(vec![1.0, 2.0, 3.0].into()));

    let mut mock = MockSearcher::new();
    mock.expect_search()
        .times(1)
        .returning(|_, _| Ok(Pagination::new(None, vec![])));

    let usecase = SearcherUseCase::new(Arc::new(mock), Arc::new(embeddings));
    assert!(usecase.search_document_parts(params).await.is_ok());
    Ok(())
}
