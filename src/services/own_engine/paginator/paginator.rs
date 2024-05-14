use crate::errors::{JsonResponse, PaginateResponse, SuccessfulResponse, WebResult};
use crate::forms::documents::document::Document;
use crate::forms::pagination::{AllScrollsForm, NextScrollForm, Paginated};
use crate::services::own_engine::context::OtherContext;
use crate::services::service;

use actix_web::web;

#[async_trait::async_trait]
impl service::PaginatorService for OtherContext {
    async fn get_pagination_ids(&self) -> JsonResponse<Vec<String>> {
        let def_vals: Vec<String> = Vec::default();
        Ok(web::Json(def_vals))
    }

    async fn delete_pagination(&self, _ids: &AllScrollsForm) -> WebResult {
        Ok(SuccessfulResponse::success("Ok"))
    }

    async fn paginate(&self, _curr_scroll: &NextScrollForm) -> PaginateResponse<Vec<Document>> {
        let paginate = Paginated::new_with_id(Vec::default(), "id".to_string());
        Ok(web::Json(paginate))
    }
}
