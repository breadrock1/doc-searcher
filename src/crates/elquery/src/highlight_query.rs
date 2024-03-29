use serde_derive::Serialize;

#[derive(Default, Serialize)]
pub struct HighlightOrder {
    order: String,
    fields: DocumentEntity,
}

#[derive(Serialize)]
struct DocumentEntity {
    content: HighlightTags,
}

#[derive(Serialize)]
struct HighlightTags {
    pre_tags: Vec<String>,
    post_tags: Vec<String>,
}

impl Default for DocumentEntity {
    fn default() -> Self {
        let highlight_tags = HighlightTags {
            pre_tags: vec!["".to_string()],
            post_tags: vec!["".to_string()],
        };
        DocumentEntity {
            content: highlight_tags,
        }
    }
}
