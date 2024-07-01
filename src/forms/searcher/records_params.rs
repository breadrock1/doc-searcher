use crate::forms::searcher::s_params::SearchParams;
use crate::forms::TestExample;

use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Serialize, IntoParams, ToSchema)]
pub struct AllRecordsParams {
    #[schema(example = "Test Folder name or path")]
    query: String,
    #[schema(example = "test-folder")]
    folder_id: Option<String>,
    #[schema(example = "document")]
    document_type: String,
    #[schema(example = "txt")]
    document_extension: String,
    #[schema(example = 0)]
    document_size_to: i64,
    #[schema(example = 0)]
    document_size_from: i64,
    #[schema(example = "2024-04-26T11:14:55Z")]
    created_date_to: String,
    #[schema(example = "2024-04-02T13:51:32Z")]
    created_date_from: String,
    #[schema(example = 10)]
    result_size: i64,
    #[schema(example = "1m")]
    scroll_lifetime: String,
}

impl From<AllRecordsParams> for SearchParams {
    fn from(value: AllRecordsParams) -> Self {
        SearchParams::builder()
            .folder_ids(value.folder_id)
            .document_type(value.document_type)
            .document_extension(value.document_extension)
            .document_size_to(value.document_size_to)
            .document_size_from(value.document_size_from)
            .created_date_to(value.created_date_to)
            .created_date_from(value.created_date_from)
            .result_size(value.result_size)
            .scroll_lifetime(value.scroll_lifetime)
            .result_offset(0)
            .query(value.query)
            .knn_amount(None)
            .knn_candidates(None)
            .show_all(None)
            .build()
            .unwrap()
    }
}

impl TestExample<AllRecordsParams> for AllRecordsParams {
    fn test_example(_value: Option<&str>) -> AllRecordsParams {
        AllRecordsParams {
            query: "Test Folder name".to_string(),
            folder_id: Some("test-folder".to_string()),
            document_type: "document".to_string(),
            document_extension: "txt".to_string(),
            created_date_to: "2024-04-26T11:14:55Z".to_string(),
            created_date_from: "2024-04-02T13:51:32Z".to_string(),
            scroll_lifetime: "1m".to_string(),
            document_size_to: 0,
            document_size_from: 4096,
            result_size: 25,
        }
    }
}
