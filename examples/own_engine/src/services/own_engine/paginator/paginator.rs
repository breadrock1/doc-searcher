use crate::errors::{Successful, WebResult};
use crate::forms::documents::document::Document;
use crate::forms::pagination::forms::{AllScrollsForm, NextScrollForm};
use crate::forms::pagination::pagination::Paginated;
use crate::services::own_engine::context::OtherContext;
use crate::services::service::{PaginatedResult, PaginatorService};

#[async_trait::async_trait]
impl PaginatorService for OtherContext {
    async fn get_pagination_ids(&self) -> WebResult<Vec<String>> {
        Ok(Vec::default())
    }

    async fn delete_pagination(&self, _ids: &AllScrollsForm) -> WebResult<Successful> {
        Ok(Successful::success("Ok"))
    }

    async fn paginate(&self, _curr_scroll: &NextScrollForm) -> PaginatedResult<Document> {
        let paginate = Paginated::new_with_id(Vec::default(), "id".to_string());
        Ok(paginate)
    }
}
