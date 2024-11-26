use derive_builder::Builder;
use serde_derive::Serialize;

#[derive(Clone, Builder, Serialize)]
pub struct MoreLikeThisQuery {
    like: String,
    min_doc_freq: i32,
    min_term_freq: i32,
    max_query_terms: i32,
    fields: Vec<String>,
}

impl Default for MoreLikeThisQuery {
    fn default() -> Self {
        MoreLikeThisQueryBuilder::default()
            .like("".to_string())
            .min_doc_freq(1)
            .min_term_freq(1)
            .max_query_terms(25)
            .fields(Vec::default())
            .build()
            .unwrap()
    }
}

impl MoreLikeThisQuery {
    pub fn with_query(mut self, query: &str) -> Self {
        self.like = query.to_string();
        self
    }

    pub fn with_fields(mut self, fields: Vec<String>) -> Self {
        self.fields = fields;
        self
    }

    pub fn with_opts(mut self, doc_freq: i32, term_freq: i32, max_terms: i32) -> Self {
        self.min_doc_freq = doc_freq;
        self.min_term_freq = term_freq;
        self.max_query_terms = max_terms;
        self
    }

    pub fn build(self) -> Self {
        self
    }
}
