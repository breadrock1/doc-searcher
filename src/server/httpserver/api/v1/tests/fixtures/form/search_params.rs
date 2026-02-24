use crate::server::httpserver::api::v1::form::{
    FilterForm, FullTextSearchForm, HybridSearchForm, ResultForm, RetrieveDocumentForm,
    SemanticSearchForm, ShortResultForm,
};

pub fn create_retrieve_document_form() -> RetrieveDocumentForm {
    RetrieveDocumentForm {
        path: Some("./test-document.docx".to_string()),
        filter: None,
        result: create_result_form(),
    }
}

pub fn create_retrieve_document_form_with_filter() -> RetrieveDocumentForm {
    RetrieveDocumentForm {
        path: Some("./test-document.docx".to_string()),
        filter: Some(create_filter_form()),
        result: create_result_form(),
    }
}

pub fn create_fulltext_search_form() -> FullTextSearchForm {
    FullTextSearchForm {
        query: Some("find something".to_string()),
        indexes: "test-index-1,test-index-2".to_string(),
        filter: None,
        result: create_result_form(),
    }
}

pub fn create_fulltext_search_form_with_filter() -> FullTextSearchForm {
    FullTextSearchForm {
        query: Some("find something".to_string()),
        indexes: "test-index-1,test-index-2".to_string(),
        filter: Some(create_filter_form()),
        result: create_result_form(),
    }
}

pub fn create_semantic_search_form() -> SemanticSearchForm {
    SemanticSearchForm {
        query: "find something".to_string(),
        indexes: "test-index-1,test-index-2".to_string(),
        knn_amount: 100,
        model_id: Some("lsdfblsbgdds".to_string()),
        tokens: Some(vec![]),
        filter: None,
        result: create_short_result_form(),
    }
}

pub fn create_semantic_search_form_with_filter() -> SemanticSearchForm {
    SemanticSearchForm {
        query: "find something".to_string(),
        indexes: "test-index-1,test-index-2".to_string(),
        knn_amount: 100,
        model_id: Some("lsdfblsbgdds".to_string()),
        tokens: Some(vec![]),
        filter: Some(create_filter_form()),
        result: create_short_result_form(),
    }
}

pub fn create_hybrid_search_form() -> HybridSearchForm {
    HybridSearchForm {
        query: "find something".to_string(),
        indexes: "test-index-1,test-index-2".to_string(),
        knn_amount: 100,
        model_id: Some("lsdfblsbgdds".to_string()),
        filter: None,
        result: create_result_form(),
        min_score: Some(0.6),
    }
}

pub fn create_hybrid_search_form_with_filter() -> HybridSearchForm {
    HybridSearchForm {
        query: "find something".to_string(),
        indexes: "test-index-1,test-index-2".to_string(),
        knn_amount: 100,
        model_id: Some("lsdfblsbgdds".to_string()),
        filter: Some(create_filter_form()),
        result: create_result_form(),
        min_score: Some(0.6),
    }
}

fn create_short_result_form() -> ShortResultForm {
    ShortResultForm {
        order: "desc".to_string(),
        size: 10,
        offset: 0,
        include_extra_fields: Some(false),
    }
}

fn create_result_form() -> ResultForm {
    ResultForm {
        order: "desc".to_string(),
        size: 10,
        offset: 0,
        include_extra_fields: Some(false),
        highlight_items: Some(3),
        highlight_item_size: Some(100),
    }
}

fn create_filter_form() -> FilterForm {
    FilterForm {
        doc_part_id: Some(1),
        size_from: Some(0),
        size_to: Some(1024),
        created_from: Some(1750957115),
        created_to: Some(1750957215),
        modified_from: Some(1750957115),
        modified_to: Some(1750957215),
        pipeline_id: Some(1),
        source: Some("source-name".to_string()),
        semantic_source: Some("semantic-source".to_string()),
        distance: Some("80km".to_string()),
        location_coordinates: Some(vec![45.99, 29.43]),
        document_class: Some("war".to_string()),
        document_class_probability: Some(0.8),
    }
}
