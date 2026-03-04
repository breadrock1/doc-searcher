use doc_search_core::domain::searcher::models::{RetrieveIndexDocumentsParams, SearchingParams};
use doc_search_core::domain::storage::models::{CreateIndexParams, LargeDocument};

use crate::server::httpserver::api::v1::form::{
    CreateDocumentForm, CreateIndexForm, FullTextSearchForm, HybridSearchForm,
    RetrieveDocumentForm, SemanticSearchForm, UpdateDocumentForm,
};
use crate::server::ServerResult;

use super::fixtures::form::*;

#[rstest::rstest]
#[case(create_index_form(), true)]
#[case(create_index_form_with_knn(), true)]
fn test_create_index_form_mapping(
    #[case] form: CreateIndexForm,
    #[case] is_success: bool,
) -> anyhow::Result<()> {
    let result: ServerResult<CreateIndexParams> = form.try_into();
    assert_eq!(result.is_ok(), is_success);
    Ok(())
}

#[rstest::rstest]
#[case(create_document_form(), true)]
#[case(create_document_form_with_metadata(), true)]
fn test_create_document_form_mapping(
    #[case] form: CreateDocumentForm,
    #[case] is_success: bool,
) -> anyhow::Result<()> {
    println!("{}", serde_json::to_string(&form).unwrap());
    let result: ServerResult<LargeDocument> = form.try_into();
    assert_eq!(result.is_ok(), is_success);
    Ok(())
}

#[rstest::rstest]
#[case(update_document_form(), true)]
#[case(update_document_form_with_metadata(), true)]
fn test_update_document_form_mapping(
    #[case] form: UpdateDocumentForm,
    #[case] is_success: bool,
) -> anyhow::Result<()> {
    let result: ServerResult<LargeDocument> = form.try_into();
    assert_eq!(result.is_ok(), is_success);
    Ok(())
}

#[rstest::rstest]
#[case(create_retrieve_document_form(), true)]
#[case(create_retrieve_document_form_with_filter(), true)]
fn test_retrieve_documents_form_mapping(
    #[case] form: RetrieveDocumentForm,
    #[case] is_success: bool,
) -> anyhow::Result<()> {
    let result: ServerResult<RetrieveIndexDocumentsParams> = form.try_into();
    assert_eq!(result.is_ok(), is_success);
    Ok(())
}

#[rstest::rstest]
#[case(create_fulltext_search_form(), true)]
#[case(create_fulltext_search_form_with_filter(), true)]
fn test_fulltext_search_form_mapping(
    #[case] form: FullTextSearchForm,
    #[case] is_success: bool,
) -> anyhow::Result<()> {
    let result: ServerResult<SearchingParams> = form.try_into();
    assert_eq!(result.is_ok(), is_success);
    Ok(())
}

#[rstest::rstest]
#[case(create_semantic_search_form(), true)]
#[case(create_semantic_search_form_with_filter(), true)]
fn test_semantic_search_form_mapping(
    #[case] form: SemanticSearchForm,
    #[case] is_success: bool,
) -> anyhow::Result<()> {
    let result: ServerResult<SearchingParams> = form.try_into();
    assert_eq!(result.is_ok(), is_success);
    Ok(())
}

#[rstest::rstest]
#[case(create_hybrid_search_form(), true)]
#[case(create_hybrid_search_form_with_filter(), true)]
fn test_hybrid_search_form_mapping(
    #[case] form: HybridSearchForm,
    #[case] is_success: bool,
) -> anyhow::Result<()> {
    let result: ServerResult<SearchingParams> = form.try_into();
    assert_eq!(result.is_ok(), is_success);
    Ok(())
}
