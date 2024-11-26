use serde_derive::Serialize;

#[derive(Clone, Default, Serialize)]
pub struct HighlightQuery {
    order: String,
    fields: HighlightFields,
}

#[derive(Clone, Serialize)]
struct HighlightFields {
    content: HighlightContent,
}

#[derive(Clone, Serialize)]
struct HighlightContent {
    pre_tags: Vec<String>,
    post_tags: Vec<String>,
}

impl Default for HighlightFields {
    fn default() -> Self {
        let default_vec = vec!["".to_string()];
        HighlightFields {
            content: HighlightContent {
                pre_tags: default_vec.clone(),
                post_tags: default_vec,
            },
        }
    }
}

impl HighlightQuery {
    pub fn set_order(mut self, order: &str) -> Self {
        self.order = order.to_string();
        self
    }

    pub fn append_pre_tag(mut self, tag: &str) -> Self {
        self.fields.content.pre_tags.push(tag.to_string());
        self
    }

    pub fn append_post_tag(mut self, tag: &str) -> Self {
        self.fields.content.post_tags.push(tag.to_string());
        self
    }

    pub fn build(self) -> Self {
        self
    }
}
