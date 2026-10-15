use crate::domain::searcher::models::SearchingParams;
use crate::shared::kernel::Tokens;

pub struct EnsuredTokens {
    pub params: SearchingParams,
    pub tokens: Option<Tokens>,
}

impl EnsuredTokens {
    pub fn new(params: SearchingParams, tokens: Option<Tokens>) -> Self {
        Self { params, tokens }
    }
}
