use derive_builder::Builder;

pub type SearchIndexes = Vec<String>;

pub struct SearchingParams {
    indexes: SearchIndexes,
    kind: SearchKindParams,
    result: ResultParams,
    filter: Option<FilterParams>,
}

impl SearchingParams {
    pub fn new(
        indexes: Vec<String>,
        kind: SearchKindParams,
        result: ResultParams,
        filter: Option<FilterParams>,
    ) -> Self {
        Self {
            indexes,
            kind,
            result,
            filter,
        }
    }

    pub fn get_indexes(&self) -> &[String] {
        self.indexes.as_slice()
    }

    pub fn get_kind(&self) -> &SearchKindParams {
        &self.kind
    }

    pub fn get_result(&self) -> &ResultParams {
        &self.result
    }

    pub fn get_filter(&self) -> Option<&FilterParams> {
        self.filter.as_ref()
    }
}

pub enum SearchKindParams {
    Retrieve(RetrieveIndexDocumentsParams),
    FullText(FullTextSearchingParams),
    Semantic(SemanticSearchingParams),
    Hybrid(HybridSearchingParams),
}

#[derive(Debug, Builder)]
pub struct RetrieveIndexDocumentsParams {
    pub path: Option<String>,
}

#[derive(Debug, Builder)]
pub struct FullTextSearchingParams {
    pub query: Option<String>,
}

#[derive(Debug, Builder)]
pub struct SemanticSearchingParams {
    pub query: String,
    pub knn_amount: u16,
    pub min_score: Option<f32>,
    pub model_id: Option<String>,
    pub tokens: Option<Vec<f64>>,
}

#[derive(Debug, Builder)]
pub struct HybridSearchingParams {
    pub query: String,
    pub knn_amount: u16,
    pub min_score: Option<f32>,
    pub model_id: Option<String>,
}

#[derive(Debug, Builder)]
pub struct PaginationParams {
    pub scroll_id: String,
}

#[derive(Clone, Debug, Builder)]
pub struct FilterParams {
    pub doc_part_id: Option<usize>,
    pub size_from: Option<u32>,
    pub size_to: Option<u32>,
    pub created_from: Option<i64>,
    pub created_to: Option<i64>,
    pub modified_from: Option<i64>,
    pub modified_to: Option<i64>,
}

#[derive(Clone, Default, Debug, Builder)]
pub struct ResultParams {
    pub size: i64,
    pub offset: i64,
    pub order: ResultOrder,
    pub highlight_items: Option<u16>,
    pub highlight_item_size: Option<u32>,
    pub include_extra_fields: Option<bool>,
}

#[derive(Clone, Debug, Default)]
pub enum ResultOrder {
    ASC,
    #[default]
    DESC,
}
