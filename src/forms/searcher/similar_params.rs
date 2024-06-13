use crate::forms::TestExample;
use crate::forms::searcher::s_params::SearchParams;

use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Serialize, IntoParams, ToSchema)]
pub struct SimilarParams {
    #[schema(example = "12:JOGnP+EfzRR00C+guy:DIFJrukvZRRWWATP+Eo70y")]
    query: String,
    #[schema(example = "test_folder")]
    folder_ids: Option<String>,
    #[schema(example = 10)]
    result_size: i64,
}

impl From<SimilarParams> for SearchParams {
    fn from(value: SimilarParams) -> Self {
        SearchParams::builder()
            .query(value.query)
            .folder_ids(value.folder_ids)
            .result_size(value.result_size)
            .document_type(String::default())
            .document_extension(String::default())
            .document_size_to(0)
            .document_size_from(0)
            .created_date_to(String::default())
            .created_date_from(String::default())
            .result_offset(0)
            .scroll_lifetime(String::default())
            .knn_amount(None)
            .knn_candidates(None)
            .build()
            .unwrap()
    }
}

impl TestExample<SimilarParams> for SimilarParams {
    fn test_example(value: Option<&str>) -> SimilarParams {
        SimilarParams {
            query:  value.unwrap().to_string(),
            folder_ids: Some("test_folder".to_string()),
            result_size: 25,
        }
    }
}

