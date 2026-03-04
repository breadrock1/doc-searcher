use crate::server::httpserver::api::v1::form::{
    Class, CreateDocumentForm, Group, Icons, Location, Metadata, Subject, UpdateDocumentForm,
};

pub fn create_document_form() -> CreateDocumentForm {
    CreateDocumentForm {
        file_name: "test-document.docx".to_string(),
        file_path: "./test-document.docx".to_string(),
        file_size: 1024,
        created_at: chrono::Utc::now().timestamp(),
        modified_at: chrono::Utc::now().timestamp(),
        content: "There is content".to_string(),
        metadata: None,
    }
}

pub fn create_document_form_with_metadata() -> CreateDocumentForm {
    let metadata = create_metadata();
    let mut create_doc_form = create_document_form();
    create_doc_form.metadata = Some(metadata);
    create_doc_form
}

pub fn update_document_form() -> UpdateDocumentForm {
    let create_doc_form = create_document_form();
    UpdateDocumentForm {
        file_name: create_doc_form.file_name,
        file_path: create_doc_form.file_path,
        file_size: create_doc_form.file_size,
        created_at: create_doc_form.created_at,
        content: create_doc_form.content,
        metadata: create_doc_form.metadata,
    }
}

pub fn update_document_form_with_metadata() -> UpdateDocumentForm {
    let create_doc_form = create_document_form_with_metadata();
    UpdateDocumentForm {
        file_name: create_doc_form.file_name,
        file_path: create_doc_form.file_path,
        file_size: create_doc_form.file_size,
        created_at: create_doc_form.created_at,
        content: create_doc_form.content,
        metadata: create_doc_form.metadata,
    }
}

fn create_metadata() -> Metadata {
    let location = Location {
        name: "location-name".to_string(),
        latitude: 10.0,
        longitude: 20.0,
    };

    let subject = Subject {
        name: "subject".to_string(),
    };

    let class = Class {
        name: "class".to_string(),
        probability: 1.0,
    };

    let icon = Icons {
        name: "icon".to_string(),
    };

    let group = Group {
        name: "group".to_string(),
    };

    Metadata {
        photo: Some("./attachment.jpeg".to_string()),
        source: Some("source".to_string()),
        semantic_source: Some("semantic-source".to_string()),
        summary: Some("summary".to_string()),
        locations: Some(vec![location]),
        subjects: Some(vec![subject]),
        classes: Some(vec![class]),
        icons: Some(vec![icon]),
        groups: Some(vec![group]),
        pipelines: Some(vec!["pipeline".to_string()]),
        references: Some(vec!["reference".to_string()]),
    }
}
