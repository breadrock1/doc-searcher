use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[allow(unused_imports)]
use serde_json::json;

use crate::application::structures::params::{
    CreateIndexParams, CreateIndexParamsBuilder, FilterParams, FilterParamsBuilder,
    FullTextSearchParams, FullTextSearchParamsBuilder, HybridSearchParams,
    HybridSearchParamsBuilder, KnnIndexParams, KnnIndexParamsBuilder, PaginateParams,
    PaginateParamsBuilder, PaginateParamsBuilderError, ResultParams, ResultParamsBuilder,
    RetrieveDocumentParams, RetrieveDocumentParamsBuilder, SemanticSearchParams,
    SemanticSearchParamsBuilder,
};
use crate::application::structures::{Document, DocumentBuilder, Embeddings};
use crate::infrastructure::httpserver::api::v1::models::response::EmbeddingsSchema;

const EMPTY_INDEX_ID: &str = "";

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct CreateIndexForm {
    #[schema(example = "Test Folder")]
    name: String,
    #[schema(example = "./")]
    path: String,
    knn: Option<KnnIndexForm>,
}

impl TryFrom<CreateIndexForm> for CreateIndexParams {
    type Error = anyhow::Error;

    fn try_from(form: CreateIndexForm) -> Result<Self, Self::Error> {
        let knn_form = match form.knn {
            None => None,
            Some(params) => Some(params.try_into()?),
        };

        let form = CreateIndexParamsBuilder::default()
            .id(EMPTY_INDEX_ID.to_string())
            .name(form.name)
            .path(form.path)
            .knn(knn_form)
            .build()?;

        Ok(form)
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct CreateDocumentForm {
    #[schema(example = "test-document.docx")]
    file_name: String,
    #[schema(example = "./test-document.docx")]
    file_path: String,
    #[schema(example = 1024)]
    file_size: u32,
    #[schema(example = "There is some content data")]
    content: String,
    #[schema(example = 1750957115)]
    created_at: i64,
    #[schema(example = 1750957115)]
    modified_at: i64,
}

impl TryFrom<CreateDocumentForm> for Document {
    type Error = anyhow::Error;

    fn try_from(form: CreateDocumentForm) -> Result<Self, Self::Error> {
        let form = DocumentBuilder::default()
            .file_name(form.file_name)
            .file_path(form.file_path)
            .file_size(form.file_size)
            .content(Some(form.content))
            .created_at(form.created_at)
            .modified_at(form.modified_at)
            .chunked_text(None)
            .embeddings(None)
            .build()?;

        Ok(form)
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct UpdateDocumentForm {
    #[schema(example = "test-document.docx")]
    file_name: String,
    #[schema(example = "./test-document.docx")]
    file_path: String,
    #[schema(example = 1024)]
    file_size: u32,
    #[schema(example = 1750957115)]
    created_at: i64,
    #[schema(nullable, example = "There is some content data")]
    content: Option<String>,
    #[schema(nullable, example = json!(vec!["There is some content data"]))]
    chunked_text: Option<Vec<String>>,
    embeddings: Option<Vec<EmbeddingsSchema>>,
}

impl TryFrom<UpdateDocumentForm> for Document {
    type Error = anyhow::Error;

    fn try_from(form: UpdateDocumentForm) -> Result<Self, Self::Error> {
        let embeddings = form
            .embeddings
            .map(|it| {
                Some(
                    it.into_iter()
                        .map(|e| e.into())
                        .collect::<Vec<Embeddings>>(),
                )
            })
            .unwrap_or_default();

        let modified_dt = chrono::Utc::now().timestamp();
        let form = DocumentBuilder::default()
            .file_name(form.file_name)
            .file_path(form.file_path)
            .file_size(form.file_size)
            .content(form.content)
            .created_at(form.created_at)
            .modified_at(modified_dt)
            .chunked_text(form.chunked_text)
            .embeddings(embeddings)
            .build()?;

        Ok(form)
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct KnnIndexForm {
    #[schema(example = 100)]
    knn_ef_searcher: u32,
    #[schema(example = 768)]
    knn_dimension: u32,
    #[schema(example = 50)]
    token_limit: u32,
    #[schema(example = 0.2)]
    overlap_rate: f32,
}

impl TryFrom<KnnIndexForm> for KnnIndexParams {
    type Error = anyhow::Error;

    fn try_from(form: KnnIndexForm) -> Result<Self, Self::Error> {
        let form = KnnIndexParamsBuilder::default()
            .knn_ef_searcher(form.knn_ef_searcher)
            .knn_dimension(form.knn_dimension)
            .token_limit(form.token_limit)
            .overlap_rate(form.overlap_rate)
            .build()?;

        Ok(form)
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct FilterForm {
    #[schema(example = 0)]
    size_from: Option<u32>,
    #[schema(example = 1024)]
    size_to: Option<u32>,
    #[schema(example = 1750957115)]
    created_from: Option<i64>,
    #[schema(example = 1750957215)]
    created_to: Option<i64>,
    #[schema(example = 1750957115)]
    modified_from: Option<i64>,
    #[schema(example = 1750957215)]
    modified_to: Option<i64>,
}

impl TryFrom<FilterForm> for FilterParams {
    type Error = anyhow::Error;

    fn try_from(form: FilterForm) -> Result<Self, Self::Error> {
        let form = FilterParamsBuilder::default()
            .size_from(form.size_from)
            .size_to(form.size_to)
            .created_from(form.created_from)
            .created_to(form.created_to)
            .modified_from(form.modified_from)
            .modified_to(form.modified_to)
            .build()?;

        Ok(form)
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct ResultForm {
    #[schema(example = "desc")]
    order: String,
    #[schema(example = 10)]
    size: u32,
    #[schema(example = 0)]
    offset: u32,
    #[schema(example = false)]
    include_extra_fields: Option<bool>,
    #[schema(example = 3)]
    highlight_items: Option<u32>,
    #[schema(example = 600)]
    highlight_item_size: Option<u32>,
}

impl From<ResultForm> for ResultParams {
    fn from(form: ResultForm) -> ResultParams {
        ResultParamsBuilder::default()
            .order(form.order)
            .size(form.size.into())
            .offset(form.offset.into())
            .include_extra_fields(form.include_extra_fields)
            .highlight_items(form.highlight_items)
            .highlight_item_size(form.highlight_item_size)
            .build()
            .unwrap()
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct ShortResultForm {
    #[schema(example = "desc")]
    order: String,
    #[schema(example = 10)]
    size: u32,
    #[schema(example = 0)]
    offset: u32,
    #[schema(example = false)]
    include_extra_fields: Option<bool>,
}

impl From<ShortResultForm> for ResultParams {
    fn from(form: ShortResultForm) -> ResultParams {
        ResultParamsBuilder::default()
            .order(form.order)
            .size(form.size.into())
            .offset(form.offset.into())
            .include_extra_fields(form.include_extra_fields)
            .highlight_items(None)
            .highlight_item_size(None)
            .build()
            .unwrap()
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct RetrieveDocumentForm {
    #[schema(example = "./test-document.docx")]
    path: Option<String>,
    filter: Option<FilterForm>,
    result: ShortResultForm,
}

impl TryFrom<RetrieveDocumentForm> for RetrieveDocumentParams {
    type Error = anyhow::Error;

    fn try_from(form: RetrieveDocumentForm) -> Result<Self, Self::Error> {
        let filter_params = match form.filter {
            None => None,
            Some(filter) => Some(filter.try_into()?),
        };

        let schema = RetrieveDocumentParamsBuilder::default()
            .path(form.path)
            .filter(filter_params)
            .result(form.result.into())
            .build()?;

        Ok(schema)
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct FullTextSearchForm {
    #[schema(example = "Hello world")]
    query: Option<String>,
    filter: Option<FilterForm>,
    result: ResultForm,
    #[schema(example = "test-folder-1,test-folder-2")]
    indexes: String,
}

impl TryFrom<FullTextSearchForm> for FullTextSearchParams {
    type Error = anyhow::Error;

    fn try_from(form: FullTextSearchForm) -> Result<Self, Self::Error> {
        let filter = match form.filter {
            None => None,
            Some(params) => Some(params.try_into()?),
        };

        let schema = FullTextSearchParamsBuilder::default()
            .query(form.query)
            .filter(filter)
            .result(form.result.into())
            .indexes(form.indexes)
            .build()?;

        Ok(schema)
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct HybridSearchForm {
    #[schema(example = "Hello world")]
    query: String,
    #[schema(example = 5)]
    knn_amount: Option<u16>,
    #[schema(example = "test-folder-1,test-folder-2")]
    indexes: String,
    #[schema(example = "PRh30JcBW8Qg3Gf4I6Ku")]
    model_id: Option<String>,
    #[schema(example = 0.7)]
    min_score: Option<f32>,
    result: ResultForm,
    filter: Option<FilterForm>,
}

impl TryFrom<HybridSearchForm> for HybridSearchParams {
    type Error = anyhow::Error;

    fn try_from(form: HybridSearchForm) -> Result<Self, Self::Error> {
        let filter = match form.filter {
            None => None,
            Some(params) => Some(params.try_into()?),
        };

        let params = HybridSearchParamsBuilder::default()
            .query(form.query)
            .knn_amount(form.knn_amount)
            .result(form.result.into())
            .indexes(form.indexes)
            .model_id(form.model_id)
            .filter(filter)
            .min_score(form.min_score)
            .build()?;

        Ok(params)
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct SemanticSearchForm {
    #[schema(example = "Hello world")]
    query: String,
    #[schema(example = 100)]
    knn_amount: Option<u16>,
    #[schema(example = "test-folder-1,test-folder-2")]
    indexes: String,
    #[schema(example = "PRh30JcBW8Qg3Gf4I6Ku")]
    model_id: Option<String>,
    #[schema(example = 0.7)]
    min_score: Option<f32>,
    tokens: Option<Vec<f64>>,
    result: ShortResultForm,
    filter: Option<FilterForm>,
}

impl TryFrom<SemanticSearchForm> for SemanticSearchParams {
    type Error = anyhow::Error;

    fn try_from(form: SemanticSearchForm) -> Result<Self, Self::Error> {
        let filter = match form.filter {
            None => None,
            Some(params) => Some(params.try_into()?),
        };

        let params = SemanticSearchParamsBuilder::default()
            .query(form.query)
            .tokens(form.tokens)
            .knn_amount(form.knn_amount)
            .filter(filter)
            .result(form.result.into())
            .indexes(form.indexes)
            .model_id(form.model_id)
            .min_score(form.min_score)
            .build()?;

        Ok(params)
    }
}

#[derive(Deserialize, Serialize, IntoParams, ToSchema)]
pub struct PaginateForm {
    #[schema(example = "FGluY2x1ZGVfY29udGV4dF91dWlkDXF1ZXJ5QW5kRmV0Y2gBFmOSWhk")]
    scroll_id: String,
    #[schema(example = "5m")]
    lifetime: String,
}

impl TryFrom<PaginateForm> for PaginateParams {
    type Error = PaginateParamsBuilderError;

    fn try_from(form: PaginateForm) -> Result<Self, Self::Error> {
        PaginateParamsBuilder::default()
            .scroll_id(form.scroll_id)
            .lifetime(form.lifetime)
            .build()
    }
}
