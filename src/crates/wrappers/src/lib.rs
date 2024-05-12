pub mod cluster;
pub mod document;
pub mod folder;
pub mod s_params;
pub mod schema;
pub mod scroll;

pub trait TestExample<T> {
    fn test_example(value: Option<&str>) -> T;
}
