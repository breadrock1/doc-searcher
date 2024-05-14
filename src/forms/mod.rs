pub(crate) mod cluster;
pub(crate) mod documents;
pub(crate) mod folder;
pub(crate) mod pagination;
pub(crate) mod preview;
pub(crate) mod s_params;
pub(crate) mod schemas;

pub trait TestExample<T> {
    fn test_example(value: Option<&str>) -> T;
}
