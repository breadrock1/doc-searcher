pub mod bucket;
pub mod cluster;
pub mod document;
pub mod file_form;
pub mod lang_chain;
pub mod schema;
pub mod scroll;
pub mod search_params;

pub trait TestExample<T> {
    fn test_example(value: Option<&str>) -> T;
}
