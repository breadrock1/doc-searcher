extern crate wrappers;

#[cfg(test)]
mod schema_tests {
    use crate::wrappers::schema::*;

    #[test]
    fn test_document_schema() {
        let doc_schema = DocumentSchema::default();
        println!("{}", serde_json::to_string_pretty(&doc_schema).unwrap());
    }

    #[test]
    fn test_document_preview_schema() {
        let doc_preview_schema = DocumentPreviewSchema::default();
        println!(
            "{}",
            serde_json::to_string_pretty(&doc_preview_schema).unwrap()
        );
    }
}
