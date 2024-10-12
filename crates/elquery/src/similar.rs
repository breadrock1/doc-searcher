use crate::search::more_like_query::MoreLikeThisQuery;

use serde_derive::Serialize;

#[derive(Clone, Default, Serialize)]
pub struct SimilarQuery {
    more_like_this: MoreLikeThisQuery,
}

#[allow(dead_code)]
impl SimilarQuery {
    pub fn set_query(mut self, query: MoreLikeThisQuery) -> Self {
        self.more_like_this = query;
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

