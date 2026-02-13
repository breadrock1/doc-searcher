use doc_search_core::domain::searcher::models::{
    FilterParams, FilterParamsBuilder, FullTextSearchingParamsBuilder,
    HybridSearchingParamsBuilder, PaginationParams, PaginationParamsBuilder, ResultOrder,
    ResultParams, ResultParamsBuilder, RetrieveIndexDocumentsParams,
    RetrieveIndexDocumentsParamsBuilder, SearchKindParams, SearchingParams,
    SemanticSearchingParamsBuilder,
};
use gset::Getset;
use serde_derive::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[allow(unused_imports)]
use serde_json::json;

use crate::server::ServerError;

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct FilterForm {
    #[schema(example = 0)]
    doc_part_id: Option<usize>,
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
    #[schema(example = 1)]
    pipeline_id: Option<i64>,
    #[schema(example = "source-name")]
    source: Option<String>,
    #[schema(example = "semantic-source-name")]
    semantic_source: Option<String>,
    #[schema(example = "80km")]
    distance: Option<String>,
    #[schema(example = json!([45.99, 29.43]))]
    location_coordinates: Option<Vec<f64>>,
    #[schema(example = "war")]
    document_class: Option<String>,
    #[schema(example = 0.8)]
    document_class_probability: Option<f64>,
}

impl TryFrom<FilterForm> for FilterParams {
    type Error = ServerError;

    fn try_from(form: FilterForm) -> Result<Self, Self::Error> {
        FilterParamsBuilder::default()
            .doc_part_id(form.doc_part_id)
            .size_from(form.size_from)
            .size_to(form.size_to)
            .created_from(form.created_from)
            .created_to(form.created_to)
            .modified_from(form.modified_from)
            .modified_to(form.modified_to)
            .pipeline_id(form.pipeline_id)
            .source(form.source)
            .semantic_source(form.semantic_source)
            .distance(form.distance)
            .location_coords(form.location_coordinates)
            .doc_class(form.document_class)
            .doc_class_probability(form.document_class_probability)
            .build()
            .map_err(|err| ServerError::IncorrectInputForm(err.to_string()))
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
    highlight_items: Option<u16>,
    #[schema(example = 600)]
    highlight_item_size: Option<u32>,
}

impl TryFrom<ResultForm> for ResultParams {
    type Error = ServerError;

    fn try_from(form: ResultForm) -> Result<Self, Self::Error> {
        let result_order = convert_string_to_result_form(form.order);
        ResultParamsBuilder::default()
            .order(result_order)
            .size(form.size.into())
            .offset(form.offset.into())
            .include_extra_fields(form.include_extra_fields)
            .highlight_items(form.highlight_items)
            .highlight_item_size(form.highlight_item_size)
            .build()
            .map_err(|err| ServerError::IncorrectInputForm(err.to_string()))
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

impl TryFrom<ShortResultForm> for ResultParams {
    type Error = ServerError;

    fn try_from(form: ShortResultForm) -> Result<Self, Self::Error> {
        let result_order = convert_string_to_result_form(form.order);
        ResultParamsBuilder::default()
            .order(result_order)
            .size(form.size.into())
            .offset(form.offset.into())
            .include_extra_fields(form.include_extra_fields)
            .highlight_items(None)
            .highlight_item_size(None)
            .build()
            .map_err(|err| ServerError::IncorrectInputForm(err.to_string()))
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct RetrieveDocumentForm {
    #[schema(example = "./test-document.docx")]
    pub path: Option<String>,
    pub filter: Option<FilterForm>,
    pub result: ResultForm,
}

impl TryFrom<RetrieveDocumentForm> for RetrieveIndexDocumentsParams {
    type Error = ServerError;

    fn try_from(form: RetrieveDocumentForm) -> Result<Self, Self::Error> {
        RetrieveIndexDocumentsParamsBuilder::default()
            .path(form.path)
            .build()
            .map_err(|err| ServerError::IncorrectInputForm(err.to_string()))
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct FullTextSearchForm {
    #[schema(example = "Hello world")]
    query: Option<String>,
    #[schema(example = "test-folder-1,test-folder-2")]
    indexes: String,
    filter: Option<FilterForm>,
    result: ResultForm,
}

impl TryFrom<FullTextSearchForm> for SearchingParams {
    type Error = ServerError;

    fn try_from(form: FullTextSearchForm) -> Result<Self, Self::Error> {
        let indexes = form
            .indexes
            .split(',')
            .map(String::from)
            .collect::<Vec<String>>();
        let filter_params = form
            .filter
            .map(|it| FilterParams::try_from(it).ok())
            .unwrap_or_default();
        let result = form.result.try_into()?;
        let full_text_params = FullTextSearchingParamsBuilder::default()
            .query(form.query)
            .build()
            .map_err(|err| ServerError::IncorrectInputForm(err.to_string()))?;

        Ok(SearchingParams::new(
            indexes,
            SearchKindParams::FullText(full_text_params),
            result,
            filter_params,
        ))
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct HybridSearchForm {
    #[schema(example = "Hello world")]
    query: String,
    #[schema(example = 5)]
    knn_amount: u16,
    #[schema(example = "test-folder-1,test-folder-2")]
    indexes: String,
    #[schema(example = "PRh30JcBW8Qg3Gf4I6Ku")]
    model_id: Option<String>,
    #[schema(example = 0.7)]
    min_score: Option<f32>,
    result: ResultForm,
    filter: Option<FilterForm>,
}

impl TryFrom<HybridSearchForm> for SearchingParams {
    type Error = ServerError;

    fn try_from(form: HybridSearchForm) -> Result<Self, Self::Error> {
        let indexes = form
            .indexes
            .split(',')
            .map(String::from)
            .collect::<Vec<String>>();
        let filter_params = form
            .filter
            .map(|it| FilterParams::try_from(it).ok())
            .unwrap_or_default();
        let result = form.result.try_into()?;
        let hybrid_params = HybridSearchingParamsBuilder::default()
            .query(form.query)
            .knn_amount(form.knn_amount)
            .model_id(form.model_id)
            .min_score(form.min_score)
            .build()
            .map_err(|err| ServerError::IncorrectInputForm(err.to_string()))?;

        Ok(SearchingParams::new(
            indexes,
            SearchKindParams::Hybrid(hybrid_params),
            result,
            filter_params,
        ))
    }
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct SemanticSearchForm {
    #[schema(example = "Hello world")]
    query: String,
    #[schema(example = 100)]
    knn_amount: u16,
    #[schema(example = "test-folder-1,test-folder-2")]
    indexes: String,
    #[schema(example = "PRh30JcBW8Qg3Gf4I6Ku")]
    model_id: Option<String>,
    #[schema(nullable)]
    tokens: Option<Vec<f64>>,
    result: ShortResultForm,
    filter: Option<FilterForm>,
}

impl TryFrom<SemanticSearchForm> for SearchingParams {
    type Error = ServerError;

    fn try_from(form: SemanticSearchForm) -> Result<Self, Self::Error> {
        let indexes = form
            .indexes
            .split(',')
            .map(String::from)
            .collect::<Vec<String>>();
        let filter_params = form
            .filter
            .map(|it| FilterParams::try_from(it).ok())
            .unwrap_or_default();
        let result = form.result.try_into()?;
        let semantic_params = SemanticSearchingParamsBuilder::default()
            .query(form.query)
            .tokens(form.tokens)
            .knn_amount(form.knn_amount)
            .model_id(form.model_id)
            .min_score(None)
            .build()
            .map_err(|err| ServerError::IncorrectInputForm(err.to_string()))?;

        Ok(SearchingParams::new(
            indexes,
            SearchKindParams::Semantic(semantic_params),
            result,
            filter_params,
        ))
    }
}

#[derive(Deserialize, Serialize, IntoParams, ToSchema)]
pub struct PaginateForm {
    #[schema(example = "FGluY2x1ZGVfY29udGV4dF91dWlkDXF1ZXJ5QW5kRmV0Y2gBFmOSWhk")]
    scroll_id: String,
}

impl TryFrom<PaginateForm> for PaginationParams {
    type Error = ServerError;

    fn try_from(form: PaginateForm) -> Result<Self, Self::Error> {
        PaginationParamsBuilder::default()
            .scroll_id(form.scroll_id)
            .build()
            .map_err(|err| ServerError::IncorrectInputForm(err.to_string()))
    }
}

fn convert_string_to_result_form(order: String) -> ResultOrder {
    match order.to_lowercase().as_str() {
        "asc" => ResultOrder::ASC,
        "desc" => ResultOrder::DESC,
        _ => ResultOrder::ASC,
    }
}
