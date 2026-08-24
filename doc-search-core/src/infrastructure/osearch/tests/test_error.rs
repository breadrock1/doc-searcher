use anyhow::anyhow;
use rstest::rstest;

use crate::domain::searcher::SearchError;
use crate::domain::storage::StorageError;
use crate::infrastructure::osearch::error::OSearchError;

const ERROR_MESSAGE: &str = "boom";

fn build_osearch_error() -> anyhow::Error {
    anyhow!(ERROR_MESSAGE)
}

#[rstest]
#[case::authentication(
    OSearchError::AuthenticationFailed(build_osearch_error()),
    "authentication failed: boom"
)]
#[case::index_not_found(
    OSearchError::IndexNotFound(build_osearch_error()),
    "index has not been founded: boom"
)]
#[case::document_not_found(
    OSearchError::DocumentNotFound(build_osearch_error()),
    "document has not been found: boom"
)]
#[case::document_already_exists(
    OSearchError::DocumentAlreadyExists(build_osearch_error()),
    "document already exists: boom"
)]
#[case::validation(
    OSearchError::ValidationError(build_osearch_error()),
    "validation error: boom"
)]
#[case::build_query(
    OSearchError::BuildQueryError(build_osearch_error()),
    "failed to build query: boom"
)]
#[case::execution(
    OSearchError::ExecutionError(build_osearch_error()),
    "execution error: boom"
)]
#[case::connection(
    OSearchError::ConnectionError(build_osearch_error()),
    "connection error: boom"
)]
#[case::undeclared(
    OSearchError::UndeclaredError(build_osearch_error()),
    "returned undeclared error from opensearch: boom"
)]
fn test_osearch_error_display(#[case] err: OSearchError, #[case] expected: &str) {
    assert_eq!(expected, format!("{err}"));
}

#[rstest]
#[case(OSearchError::AuthenticationFailed(build_osearch_error()), "auth")]
#[case(OSearchError::IndexNotFound(build_osearch_error()), "index")]
#[case(OSearchError::DocumentNotFound(build_osearch_error()), "doc_not_found")]
#[case(
    OSearchError::DocumentAlreadyExists(build_osearch_error()),
    "doc_exists"
)]
#[case(OSearchError::ValidationError(build_osearch_error()), "validation")]
#[case(OSearchError::BuildQueryError(build_osearch_error()), "internal")]
#[case(OSearchError::ExecutionError(build_osearch_error()), "internal")]
#[case(OSearchError::ConnectionError(build_osearch_error()), "internal")]
#[case(OSearchError::UndeclaredError(build_osearch_error()), "internal")]
fn test_from_osearch_error_for_storage_error(#[case] err: OSearchError, #[case] expected: &str) {
    let storage_err = StorageError::from(err);

    let is_internal = matches!(storage_err, StorageError::InternalError(_));
    match expected {
        "auth" => assert!(matches!(storage_err, StorageError::AuthenticationFailed(_))),
        "index" => assert!(matches!(storage_err, StorageError::IndexNotFound(_))),
        "doc_not_found" => assert!(matches!(storage_err, StorageError::DocumentNotFound(_))),
        "doc_exists" => assert!(matches!(
            storage_err,
            StorageError::DocumentAlreadyExists(_)
        )),
        "validation" => assert!(matches!(storage_err, StorageError::ValidationError(_))),
        "internal" => assert!(is_internal),
        _ => panic!("unexpected expected marker {expected}"),
    }
}

#[rstest]
#[case(OSearchError::AuthenticationFailed(build_osearch_error()), "auth")]
#[case(OSearchError::IndexNotFound(build_osearch_error()), "index")]
#[case(OSearchError::DocumentNotFound(build_osearch_error()), "internal")]
#[case(OSearchError::DocumentAlreadyExists(build_osearch_error()), "internal")]
#[case(OSearchError::ValidationError(build_osearch_error()), "validation")]
#[case(OSearchError::BuildQueryError(build_osearch_error()), "internal")]
#[case(OSearchError::ExecutionError(build_osearch_error()), "service")]
#[case(OSearchError::ConnectionError(build_osearch_error()), "connection")]
#[case(OSearchError::UndeclaredError(build_osearch_error()), "unknown")]
fn test_from_osearch_error_for_search_error(#[case] err: OSearchError, #[case] expected: &str) {
    let search_err = SearchError::from(err);

    match expected {
        "auth" => assert!(matches!(search_err, SearchError::AuthenticationFailed(_))),
        "index" => assert!(matches!(search_err, SearchError::IndexNotFound(_))),
        "internal" => assert!(matches!(search_err, SearchError::InternalError(_))),
        "validation" => assert!(matches!(search_err, SearchError::ValidationError(_))),
        "service" => assert!(matches!(search_err, SearchError::ServiceError(_))),
        "connection" => assert!(matches!(search_err, SearchError::ConnectionError(_))),
        "unknown" => assert!(matches!(search_err, SearchError::UnknownError(_))),
        _ => panic!("unexpected expected marker {expected}"),
    }
}

#[test]
fn test_from_file_error_for_storage_error() {
    let io_err = std::fs::File::open("__definitely_missing_file__").expect_err("missing file");
    let opensearch_err = opensearch::Error::from(io_err);
    let storage_err = StorageError::from(opensearch_err);
    assert!(matches!(storage_err, StorageError::InternalError(_)));
}

#[test]
fn test_from_json_error_for_storage_error() {
    let json_err = serde_json::from_str::<u8>("boom").expect_err("invalid json");
    let opensearch_err = opensearch::Error::from(json_err);
    let storage_err = StorageError::from(opensearch_err);
    assert!(matches!(storage_err, StorageError::InternalError(_)));
}
