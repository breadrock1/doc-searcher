use doc_search::errors::{Successful, WebResult};
use doc_search::forms::documents::document::Document;
use doc_search::forms::pagination::forms::{AllScrollsForm, NextScrollForm};
use doc_search::forms::pagination::pagination::Paginated;
use doc_search::services::own_engine::context::OtherContext;
use doc_search::services::service::{PaginatedResult, PaginatorService};

#[async_trait::async_trait]
impl PaginatorService for OtherContext {
    async fn delete_pagination(&self, _ids: &AllScrollsForm) -> WebResult<Successful> {
        Ok(Successful::success("Ok"))
    }

    async fn paginate(&self, _curr_scroll: &NextScrollForm) -> PaginatedResult<Document> {
        let paginate = Paginated::new_with_id(Vec::default(), "id".to_string());
        Ok(paginate)
    }
}
