use crate::searcher::models::SearchParams;
use crate::storage::forms::DocumentType;

use derive_builder::Builder;
use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Builder, Deserialize, Serialize, IntoParams, ToSchema)]
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
            .query_tokens(Some(Vec::default()))
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

impl FulltextParams {
    pub fn builder() -> FulltextParamsBuilder {
        FulltextParamsBuilder::default()
    }
}

#[derive(Builder, Deserialize, Serialize, IntoParams, ToSchema)]
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
            .query(value.query)
            .query_tokens(Some(Vec::default()))
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
            .knn_amount(None)
            .knn_candidates(None)
            .show_all(None)
            .build()
            .unwrap()
    }
}

impl AllRecordsParams {
    pub fn builder() -> AllRecordsParamsBuilder {
        AllRecordsParamsBuilder::default()
    }
}

#[derive(Builder, Deserialize, Serialize, IntoParams, ToSchema)]
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
            .query_tokens(Some(Vec::default()))
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

impl SemanticParams {
    pub fn builder() -> SemanticParamsBuilder {
        SemanticParamsBuilder::default()
    }
}

#[derive(Default, Deserialize, IntoParams, ToSchema)]
pub struct SearchQuery {
    document_type: Option<DocumentType>,
}

impl SearchQuery {
    pub fn get_type(&self) -> DocumentType {
        self.document_type.clone().unwrap_or(DocumentType::Document)
    }
}

#[derive(Builder, Deserialize, Serialize, IntoParams, ToSchema)]
pub struct PaginateNextForm {
    #[schema(example = "FGluY2x1ZGVfY29udGV4dF91dWlkDXF1ZXJ5QW5kRmV0Y2gBFmOSWhk")]
    scroll_id: String,
    #[schema(example = "1m")]
    lifetime: String,
}

impl PaginateNextForm {
    pub fn builder() -> PaginateNextFormBuilder {
        PaginateNextFormBuilder::default()
    }

    pub fn get_scroll_id(&self) -> &str {
        self.scroll_id.as_str()
    }

    pub fn get_lifetime(&self) -> &str {
        self.lifetime.as_str()
    }
}

#[derive(Builder, Deserialize, Serialize, IntoParams, ToSchema)]
pub struct DeletePaginationsForm {
    #[schema(example = "FGluY2x1ZGVfY29udGV4dF91dWlkDXF1ZXJ5QW5kRmV0Y2gBFmOSWhk")]
    sessions: Vec<String>,
}

impl DeletePaginationsForm {
    pub fn builder() -> DeletePaginationsFormBuilder {
        DeletePaginationsFormBuilder::default()
    }

    pub fn get_sessions(&self) -> Vec<&str> {
        self.sessions.iter().map(String::as_str).collect()
    }
}
