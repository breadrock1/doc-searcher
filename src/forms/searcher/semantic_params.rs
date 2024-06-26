use crate::forms::searcher::s_params::SearchParams;
use crate::forms::TestExample;

use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Serialize, IntoParams, ToSchema)]
pub struct SemanticParams {
    #[schema(example = "12:JOGnP+EfzRR00C+guy:DIFJrukvZRRWWATP+Eo70y")]
    query: String,
    #[schema(example = "test-folder")]
    folder_ids: Option<String>,
    #[schema(example = 0)]
    document_size_from: i64,
    #[schema(example = 5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    knn_amount: Option<u16>,
    #[schema(example = 100)]
    #[serde(skip_serializing_if = "Option::is_none")]
    knn_candidates: Option<u32>,
    #[schema(example = "1m")]
    scroll_lifetime: String,
}

impl From<SemanticParams> for SearchParams {
    fn from(value: SemanticParams) -> Self {
        SearchParams::builder()
            .query(value.query)
            .folder_ids(value.folder_ids)
            .document_size_from(value.document_size_from)
            .knn_amount(value.knn_amount)
            .knn_candidates(value.knn_candidates)
            .scroll_lifetime(value.scroll_lifetime)
            .result_size(25)
            .document_size_to(0)
            .result_offset(0)
            .document_type(String::default())
            .document_extension(String::default())
            .created_date_to(String::default())
            .created_date_from(String::default())
            .build()
            .unwrap()
    }
}

impl TestExample<SemanticParams> for SemanticParams {
    fn test_example(_value: Option<&str>) -> SemanticParams {
        SemanticParams {
            query: "12:JOGnP+EfzRR00C+guy:DIFJrukvZRRWWATP+Eo70y".to_string(),
            folder_ids: Some("test-folder".to_string()),
            document_size_from: 0,
            knn_amount: Some(5),
            knn_candidates: Some(100),
            scroll_lifetime: "1m".to_string(),
        }
    }
}
