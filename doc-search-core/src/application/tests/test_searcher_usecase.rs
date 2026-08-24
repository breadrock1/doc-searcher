use anyhow::Context;
use rstest::rstest;
use std::sync::Arc;

use crate::application::tests::mock::searcher::MockSearcher;
use crate::application::usecase::searcher::SearcherUseCase;
use crate::domain::searcher::SearchError;
use crate::domain::searcher::models::{Pagination, PaginationParamsBuilder, SearchingParams};
use crate::domain::searcher::tests::fixture::params::build_full_text_searching_params;

#[rstest]
#[tokio::test]
async fn test_search_document_parts_success(
    #[from(build_full_text_searching_params)] params: SearchingParams,
) -> anyhow::Result<()> {
    let mut mock = MockSearcher::new();
    mock.expect_search()
        .times(1)
        .returning(|_| Ok(Pagination::new(None, vec![])));

    let usecase = SearcherUseCase::new(Arc::new(mock));
    let result = usecase.search_document_parts(&params).await;

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
        .returning(|_| Err(SearchError::InternalError(anyhow::anyhow!("boom"))));

    let usecase = SearcherUseCase::new(Arc::new(mock));
    let result = usecase.search_document_parts(&params).await;

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

    let usecase = SearcherUseCase::new(Arc::new(mock));
    let params = PaginationParamsBuilder::default()
        .scroll_id("scroll-value".to_string())
        .build()
        .context("failed to build pagination params")?;

    let result = usecase.load_next_pagination(&params).await;

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

    let usecase = SearcherUseCase::new(Arc::new(mock));
    let params = PaginationParamsBuilder::default()
        .scroll_id("scroll-value".to_string())
        .build()
        .context("failed to build pagination params")?;

    let result = usecase.load_next_pagination(&params).await;

    assert!(result.is_err());
    Ok(())
}
