use crate::filter::must_filter::BoolMustFilter;
use crate::filter::should_filter::BoolShouldFilter;

pub mod must_filter;
pub mod should_filter;

pub trait FilterQueryTrait {}
impl FilterQueryTrait for BoolMustFilter {}
impl FilterQueryTrait for BoolShouldFilter {}
