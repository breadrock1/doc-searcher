use derive_builder::Builder;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct SimilarQuery {
    more_like_this: LikeThisQuery,
}

#[derive(Builder, Serialize)]
struct LikeThisQuery {
    like: String,
    min_doc_freq: i32,
    min_term_freq: i32,
    max_query_terms: i32,
    fields: Vec<String>,
}

impl SimilarQuery {
    pub fn new(query: String, fields: Vec<String>) -> Self {
        SimilarQuery {
            more_like_this: LikeThisQueryBuilder::default()
                .like(query)
                .min_doc_freq(1)
                .min_term_freq(1)
                .max_query_terms(25)
                .fields(fields)
                .build()
                .unwrap(),
        }
    }
}
