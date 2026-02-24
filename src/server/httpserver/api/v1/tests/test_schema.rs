use doc_search_core::domain::searcher::models::{DocumentPartEntrails, FoundedDocument};
use doc_search_core::domain::storage::models::{DocumentPart, IndexId};

use crate::server::httpserver::api::v1::schema::{
    DocumentPartSchema, FoundedDocumentPartSchema, IndexSchema,
};
use crate::server::ServerResult;

use super::fixtures::schema::*;

#[rstest::rstest]
#[case(index_schema())]
fn test_index_schema_mappings(#[case] form: IndexSchema) -> anyhow::Result<()> {
    let result: IndexId = form.clone().into();
    assert_eq!(form.id, result);
    Ok(())
}

#[rstest::rstest]
#[case(document_part_schema(), true)]
fn test_document_part_schema_mapping(
    #[case] form: DocumentPartSchema,
    #[case] is_success: bool,
) -> anyhow::Result<()> {
    let result: ServerResult<DocumentPart> = form.clone().try_into();
    assert_eq!(result.is_ok(), is_success);

    let result: ServerResult<DocumentPartSchema> = result?.try_into();
    assert_eq!(result.is_ok(), is_success);

    let result: ServerResult<DocumentPartEntrails> = form.try_into();
    assert_eq!(result.is_ok(), is_success);

    let result: ServerResult<DocumentPartSchema> = result?.try_into();
    assert_eq!(result.is_ok(), is_success);

    Ok(())
}

#[rstest::rstest]
#[case(founded_document_schema(), true)]
fn test_founded_documents_schema(
    #[case] form: FoundedDocumentPartSchema,
    #[case] is_success: bool,
) -> anyhow::Result<()> {
    let result: ServerResult<FoundedDocument> = form.try_into();
    assert_eq!(result.is_ok(), is_success);

    let result: ServerResult<FoundedDocumentPartSchema> = result?.try_into();
    assert_eq!(result.is_ok(), is_success);

    Ok(())
}
