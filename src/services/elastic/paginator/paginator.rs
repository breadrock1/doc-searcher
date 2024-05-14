use crate::errors::{JsonResponse, PaginateResponse, SuccessfulResponse, WebError};
use crate::forms::document::Document;
use crate::forms::scroll::{AllScrollsForm, NextScrollForm};
use crate::services::elastic::{context, helper};
use crate::services::searcher::PaginatorService;

use actix_web::web;
use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::Method;
use elasticsearch::{ClearScrollParts, ScrollParts};
use serde_json::Value;

#[async_trait::async_trait]
impl PaginatorService for context::ElasticContext {
    async fn get_pagination_ids(&self) -> JsonResponse<Vec<String>> {
        let elastic = self.get_cxt().read().await;
        let response_result = elastic
            .send(
                Method::Post,
                "/_search/scroll=10m",
                HeaderMap::new(),
                Option::<&Value>::None,
                Some(b"".as_ref()),
                None,
            )
            .await;

        match response_result {
            Ok(response) => {
                log::info!("Pag Ids: {}", response.text().await.unwrap());
                let def_vals: Vec<String> = Vec::default();
                Ok(web::Json(def_vals))
            }
            Err(err) => {
                log::error!("Failed while searching documents: {}", err);
                Err(WebError::SearchError(err.to_string()))
            }
        }
    }
    async fn delete_pagination(
        &self,
        ids: &AllScrollsForm,
    ) -> Result<SuccessfulResponse, WebError> {
        let elastic = self.get_cxt().read().await;
        let response = elastic
            .clear_scroll(ClearScrollParts::ScrollId(&ids.as_slice()))
            .send()
            .await
            .map_err(WebError::from)?;

        helper::parse_elastic_response(response).await
    }
    async fn paginate(&self, scroll_form: &NextScrollForm) -> PaginateResponse<Vec<Document>> {
        let elastic = self.get_cxt().read().await;
        let response_result = elastic
            .scroll(ScrollParts::ScrollId(scroll_form.get_scroll_id()))
            .pretty(true)
            .send()
            .await;

        if response_result.is_err() {
            let err = response_result.err().unwrap();
            log::error!("Failed to get next pagination: {}", err.to_string());
            return Err(WebError::PaginationError(err.to_string()));
        }

        let response = response_result.unwrap();
        Ok(web::Json(helper::parse_search_result(response).await))
    }
}
