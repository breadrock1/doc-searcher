use crate::server::httpserver::api::v1::form::{CreateIndexForm, KnnIndexForm};

pub const TEST_INDEX_ID: &str = "test-index";

pub fn create_index_form() -> CreateIndexForm {
    CreateIndexForm {
        id: TEST_INDEX_ID.to_string(),
        knn: None,
    }
}

pub fn create_index_form_with_knn() -> CreateIndexForm {
    CreateIndexForm {
        id: TEST_INDEX_ID.to_string(),
        knn: Some(create_index_knn()),
    }
}

fn create_index_knn() -> KnnIndexForm {
    KnnIndexForm {
        knn_dimension: 768,
        token_limit: 700,
        overlap_rate: 0.2,
    }
}
