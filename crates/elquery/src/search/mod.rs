use crate::search::match_all_query::BoolMatchAllQuery;
use crate::search::multi_match_query::BoolMultiMatchQuery;
use crate::search::must_query::BoolMustQuery;
use crate::search::should_query::BoolShouldQuery;

pub mod match_all_query;
pub mod should_query;
pub mod more_like_query;
pub mod multi_match_query;
pub mod must_query;

pub trait SearchQueryTrait {}
impl SearchQueryTrait for BoolMustQuery {}
impl SearchQueryTrait for BoolShouldQuery {}
impl SearchQueryTrait for BoolMatchAllQuery {}
impl SearchQueryTrait for BoolMultiMatchQuery {}
