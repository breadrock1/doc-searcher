use crate::storage::models::DEFAULT_FOLDER_ID;
use crate::storage::models::{Document, DocumentPreview, DocumentVectors};

use derive_builder::Builder;
use getset::{CopyGetters, Getters};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};

#[derive(Clone, Default, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum DocumentType {
    #[default]
    Document,
    Preview,
    Vectors,
}

impl DocumentType {
    pub fn document_to_value(&self, document: &Document) -> Result<Value, serde_json::Error> {
        match self {
            DocumentType::Preview => serde_json::to_value(DocumentPreview::from(document)),
            DocumentType::Vectors => serde_json::to_value(DocumentVectors::from(document)),
            DocumentType::Document => serde_json::to_value(document),
        }
    }
}

#[derive(Default, Deserialize, IntoParams, ToSchema)]
pub struct DocumentTypeQuery {
    document_type: Option<DocumentType>,
}

impl DocumentTypeQuery {
    pub fn get_type(&self) -> DocumentType {
        self.document_type.clone().unwrap_or(DocumentType::Document)
    }
}

#[derive(Builder, Debug, Deserialize, Serialize, Getters, CopyGetters, IntoParams, ToSchema)]
#[getset(get = "pub")]
pub struct FulltextParams {
    #[schema(example = "Hello world")]
    query: String,

    #[schema(example = "test-folder")]
    folder_ids: String,

    #[schema(example = "document")]
    document_type: Option<String>,

    #[schema(example = "txt")]
    document_extension: Option<String>,

    #[getset(skip)]
    #[schema(example = 0)]
    document_size_to: Option<i64>,

    #[getset(skip)]
    #[schema(example = 0)]
    document_size_from: Option<i64>,

    #[getset(skip)]
    #[schema(example = "2024-04-26T11:14:55Z")]
    created_date_to: Option<String>,

    #[getset(skip)]
    #[schema(example = "2024-04-02T13:51:32Z")]
    created_date_from: Option<String>,

    #[getset(skip)]
    #[schema(example = 10)]
    result_size: i64,

    #[getset(skip)]
    #[schema(example = 0)]
    result_offset: i64,

    #[schema(example = "1m")]
    scroll_lifetime: String,
}

impl FulltextParams {
    pub fn builder() -> FulltextParamsBuilder {
        FulltextParamsBuilder::default()
    }

    pub fn document_size(&self) -> (Option<i64>, Option<i64>) {
        (self.document_size_from, self.document_size_to)
    }

    pub fn document_dates(&self) -> (Option<String>, Option<String>) {
        (self.created_date_from.clone(), self.created_date_to.clone())
    }

    pub fn result_size(&self) -> (i64, i64) {
        (self.result_size, self.result_offset)
    }
}

#[derive(Builder, Debug, Deserialize, Serialize, Getters, CopyGetters, IntoParams, ToSchema)]
#[getset(get = "pub")]
pub struct SemanticParams {
    #[schema(example = "Show me something like ...")]
    query: String,

    #[getset(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    query_tokens: Option<Vec<f64>>,

    #[schema(example = "test-folder")]
    folder_ids: String,

    #[getset(skip)]
    #[schema(example = 0)]
    document_size_from: i64,

    #[getset(skip)]
    #[schema(example = 4096)]
    document_size_to: i64,

    #[getset(skip)]
    #[schema(example = 5)]
    #[serde(skip_serializing_if = "Option::is_none")]
    knn_amount: Option<u16>,

    #[getset(skip)]
    #[schema(example = 100)]
    #[serde(skip_serializing_if = "Option::is_none")]
    knn_candidates: Option<u32>,

    #[getset(skip)]
    #[getset(get_copy = "pub")]
    #[schema(example = 100)]
    #[serde(skip_serializing_if = "Option::is_none")]
    is_grouped: Option<bool>,

    #[schema(example = "1m")]
    scroll_lifetime: String,
}

impl SemanticParams {
    pub fn builder() -> SemanticParamsBuilder {
        SemanticParamsBuilder::default()
    }

    pub fn candidates(&self) -> u32 {
        self.knn_candidates.unwrap_or(5)
    }

    pub fn knn_amount(&self) -> u16 {
        self.knn_amount.unwrap_or(100)
    }

    pub fn query_tokens(&self) -> Vec<f64> {
        self.query_tokens.clone().unwrap_or_default()
    }

    pub fn set_tokens(&mut self, tokens: Vec<f64>) {
        self.query_tokens = Some(tokens);
    }

    pub fn document_size(&self) -> (i64, i64) {
        (self.document_size_from, self.document_size_to)
    }

    pub fn result_size(&self) -> (i64, i64) {
        (self.document_size_from, 100)
    }
}

impl Default for SemanticParams {
    fn default() -> Self {
        SemanticParams::builder()
            .query("Show me something like ...".to_string())
            .query_tokens(Some(Vec::default()))
            .folder_ids(DEFAULT_FOLDER_ID.to_string())
            .document_size_from(0)
            .scroll_lifetime("10m".to_string())
            .knn_amount(Some(5))
            .knn_candidates(Some(100))
            .build()
            .unwrap()
    }
}

#[derive(Builder, Debug, Deserialize, Serialize, Getters, IntoParams, ToSchema)]
#[getset(get = "pub")]
pub struct ScrollNextForm {
    #[schema(example = "FGluY2x1ZGVfY29udGV4dF91dWlkDXF1ZXJ5QW5kRmV0Y2gBFmOSWhk")]
    scroll_id: String,
    #[schema(example = "1m")]
    lifetime: String,
}

impl ScrollNextForm {
    pub fn builder() -> ScrollNextFormBuilder {
        ScrollNextFormBuilder::default()
    }
}

#[derive(Builder, Deserialize, Serialize, Getters, IntoParams, ToSchema)]
#[getset(get = "pub")]
pub struct DeleteScrollsForm {
    #[schema(example = "FGluY2x1ZGVfY29udGV4dF91dWlkDXF1ZXJ5QW5kRmV0Y2gBFmOSWhk")]
    sessions: Vec<String>,
}

impl DeleteScrollsForm {
    pub fn builder() -> DeleteScrollsFormBuilder {
        DeleteScrollsFormBuilder::default()
    }
}
