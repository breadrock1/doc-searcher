use crate::server::httpserver::api::v1::schema::IndexSchema;

pub fn index_schema() -> IndexSchema {
    IndexSchema {
        id: "test-index".to_string(),
    }
}
