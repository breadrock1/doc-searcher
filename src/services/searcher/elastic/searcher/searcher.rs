use crate::forms::documents::document::Document;
use crate::forms::documents::embeddings::DocumentVectors;
use crate::forms::documents::preview::DocumentPreview;
use crate::forms::documents::similar::DocumentSimilar;
use crate::forms::searcher::s_params::SearchParams;
use crate::services::searcher::elastic::context::ElasticContext;
use crate::services::searcher::elastic::searcher::helper;
use crate::services::searcher::service::{PaginatedResult, SearcherService};

#[async_trait::async_trait]
impl SearcherService for ElasticContext {
    async fn search_previews(&self, s_params: &SearchParams) -> PaginatedResult<DocumentPreview> {
        let cxt_opts = self.get_options().as_ref();
        let elastic = self.get_cxt().read().await;
        let folders = s_params.get_folders(true);
        let indexes = folders.split(',').collect::<Vec<&str>>();
        helper::search::<DocumentPreview>(&elastic, s_params, cxt_opts, indexes.as_slice()).await
    }
    async fn search_fulltext(&self, s_params: &SearchParams) -> PaginatedResult<Document> {
        let cxt_opts = self.get_options().as_ref();
        let elastic = self.get_cxt().read().await;
        let folders = s_params.get_folders(false);
        let indexes = folders.split(',').collect::<Vec<&str>>();
        helper::search::<Document>(&elastic, s_params, cxt_opts, indexes.as_slice()).await
    }
    async fn search_semantic(&self, s_params: &SearchParams) -> PaginatedResult<DocumentVectors> {
        let cxt_opts = self.get_options().as_ref();
        let elastic = self.get_cxt().read().await;
        let folders = s_params.get_folders(true);
        let indexes = folders.split(',').collect::<Vec<&str>>();
        helper::search::<DocumentVectors>(&elastic, s_params, cxt_opts, indexes.as_slice()).await
    }
    async fn search_similar(&self, s_params: &SearchParams) -> PaginatedResult<DocumentSimilar> {
        let cxt_opts = self.get_options().as_ref();
        let elastic = self.get_cxt().read().await;
        let folders = s_params.get_folders(true);
        let indexes = folders.split(',').collect::<Vec<&str>>();
        helper::search::<DocumentSimilar>(&elastic, s_params, cxt_opts, indexes.as_slice()).await
    }
}
