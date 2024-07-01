use crate::errors::{Successful, WebError, WebErrorEntity, WebResult};
use crate::forms::documents::document::Document;
use crate::forms::documents::forms::DocumentType;
use crate::forms::pagination::forms::{DeletePaginationsForm, PaginateNextForm};
use crate::services::searcher::elastic::context::ElasticContext;
use crate::services::searcher::elastic::helper;
use crate::services::searcher::elastic::searcher::helper as s_helper;
use crate::services::searcher::service::{PaginatedResult, PaginatorService};

use elasticsearch::{ClearScrollParts, ScrollParts};
use serde_json::Value;

#[async_trait::async_trait]
impl PaginatorService for ElasticContext {
    async fn delete_session(&self, form: &DeletePaginationsForm) -> WebResult<Successful> {
        let ids = form.get_sessions();
        let elastic = self.get_cxt().read().await;
        let response = elastic
            .clear_scroll(ClearScrollParts::ScrollId(ids.as_slice()))
            .send()
            .await
            .map_err(WebError::from)?;

        helper::parse_elastic_response(response).await
    }
    async fn paginate(
        &self,
        scroll_form: &PaginateNextForm,
        doc_type: &DocumentType,
    ) -> PaginatedResult<Value> {
        if doc_type.is_vector_type() {
            let msg = "Can't paginate vectors search result";
            log::error!("Failed while paginate: {}", msg);
            let entity = WebErrorEntity::new(msg.to_string());
            return Err(WebError::PaginationError(entity));
        }

        let elastic = self.get_cxt().read().await;
        let response_result = elastic
            .scroll(ScrollParts::ScrollId(scroll_form.get_scroll_id()))
            .pretty(true)
            .send()
            .await;

        if response_result.is_err() {
            let err = response_result.err().unwrap();
            log::error!("Failed to get next pagination: {}", err.to_string());
            let entity = WebErrorEntity::new(err.to_string());
            return Err(WebError::PaginationError(entity));
        }

        let response = response_result.unwrap();
        let paginated = s_helper::extract_elastic_response::<Document>(response).await;
        Ok(helper::to_unified_pag(paginated, doc_type))
    }
}
