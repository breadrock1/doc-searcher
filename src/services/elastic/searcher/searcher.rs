use crate::errors::PaginateResponse;
use crate::forms::documents::document::Document;
use crate::forms::s_params::SearchParams;
use crate::services::elastic::context::ElasticContext;
use crate::services::elastic::searcher::helper;
use crate::services::service::SearcherService;

#[async_trait::async_trait]
impl SearcherService for ElasticContext {
    async fn search(&self, s_params: &SearchParams) -> PaginateResponse<Vec<Document>> {
        let elastic = self.get_cxt().read().await;
        let body_value = helper::build_search_query(s_params);
        let folders = s_params.get_folders(false);
        let indexes = folders.split(',').collect::<Vec<&str>>();
        match helper::search_documents(&elastic, s_params, &body_value, indexes.as_slice()).await {
            Ok(documents) => Ok(documents),
            Err(err) => {
                log::error!("Failed while searching documents: {}", err);
                Err(err)
            }
        }
    }
    async fn search_tokens(&self, s_params: &SearchParams) -> PaginateResponse<Vec<Document>> {
        let elastic = self.get_cxt().read().await;
        let body_value = helper::build_semantic_query(s_params);
        let folders = s_params.get_folders(true);
        let indexes = folders.split(',').collect::<Vec<&str>>();
        match helper::search_documents(&elastic, s_params, &body_value, indexes.as_slice()).await {
            Ok(documents) => Ok(documents),
            Err(err) => {
                log::error!("Failed while searching (semantic) documents: {}", err);
                Err(err)
            }
        }
    }
    async fn similarity(&self, s_params: &SearchParams) -> PaginateResponse<Vec<Document>> {
        let elastic = self.get_cxt().read().await;
        let body_value = helper::build_search_similar_query(s_params);
        let folders = s_params.get_folders(true);
        let indexes = folders.split(',').collect::<Vec<&str>>();
        match helper::search_documents(&elastic, s_params, &body_value, indexes.as_slice()).await {
            Ok(documents) => Ok(documents),
            Err(err) => {
                log::error!("Failed while searching similar documents: {}", err);
                Err(err)
            }
        }
    }
}
