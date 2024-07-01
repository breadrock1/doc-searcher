pub(crate) mod clusters;
pub(crate) mod documents;
pub mod folders;
pub(crate) mod pagination;
pub(crate) mod schemas;
pub(crate) mod searcher;

pub trait TestExample<T> {
    fn test_example(value: Option<&str>) -> T;
}
