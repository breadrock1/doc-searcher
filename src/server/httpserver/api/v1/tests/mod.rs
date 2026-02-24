pub mod fixtures;
pub mod stubs;

mod test_form;
mod test_routers_document;
mod test_routers_index;
mod test_routers_searcher;
mod test_schema;

pub const TEST_CONTENT_TYPE: &str = "application/json";
pub const RESPONSE_BODY_SIZE_LIMIT: usize = usize::MAX;
