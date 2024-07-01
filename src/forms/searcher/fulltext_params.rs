use crate::forms::searcher::s_params::SearchParams;
use crate::forms::TestExample;

use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Serialize, IntoParams, ToSchema)]
pub struct FulltextParams {
    #[schema(example = "Hello world")]
    query: String,
    #[schema(example = "test-folder")]
    folder_ids: Option<String>,
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
    #[schema(example = 0)]
    result_offset: i64,
    #[schema(example = "1m")]
    scroll_lifetime: String,
}

impl From<FulltextParams> for SearchParams {
    fn from(value: FulltextParams) -> Self {
        SearchParams::builder()
            .query(value.query)
            .folder_ids(value.folder_ids)
            .document_type(value.document_type)
            .document_extension(value.document_extension)
            .document_size_to(value.document_size_to)
            .document_size_from(value.document_size_from)
            .created_date_to(value.created_date_to)
            .created_date_from(value.created_date_from)
            .result_size(value.result_size)
            .result_offset(value.result_offset)
            .scroll_lifetime(value.scroll_lifetime)
            .knn_amount(None)
            .knn_candidates(None)
            .build()
            .unwrap()
    }
}

impl TestExample<FulltextParams> for FulltextParams {
    fn test_example(_value: Option<&str>) -> FulltextParams {
        FulltextParams {
            query: "Hello World".to_string(),
            folder_ids: Some("test-folder".to_string()),
            document_type: "document".to_string(),
            document_extension: "txt".to_string(),
            created_date_to: "2024-04-26T11:14:55Z".to_string(),
            created_date_from: "2024-04-02T13:51:32Z".to_string(),
            scroll_lifetime: "1m".to_string(),
            document_size_to: 0,
            document_size_from: 4096,
            result_size: 25,
            result_offset: 0,
        }
    }
}
