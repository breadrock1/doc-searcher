use crate::filter_query::{FilterDateQuery, FilterItem, FilterMust, FilterShould};
use crate::highlight_query::HighlightOrder;
use crate::sort_query::SortQuery;
use serde_json::Value;
use crate::bool_query_must::BoolQueryMust;
use crate::bool_query_should::BoolQueryShould;

pub struct CommonQuery {
    query: BoolQuery,
    sort: Option<SortQuery>,
    highlight: Option<HighlightOrder>,
    min_score: Option<u32>,
}

pub struct BoolQuery {
    bool: BoolQueryItems,
}

pub struct BoolQueryItems {
    // #[serde(flatter)]
    must: Option<BoolQueryMust>,
    // #[serde(flatter)]
    should: Option<BoolQueryShould>,
    // #[serde(flatter)]
    filter: Option<CommonFilter>,
}

pub struct CommonFilter {
    bool: BoolFilter,
}

pub struct BoolFilter {
    // #[serde(flatter)]
    must: Option<FilterMust>,
    // #[serde(flatter)]
    should: Option<FilterShould>,
}


