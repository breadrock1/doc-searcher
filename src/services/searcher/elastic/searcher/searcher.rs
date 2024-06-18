use crate::forms::documents::document::Document;
use crate::forms::documents::embeddings::DocumentVectors;
use crate::forms::documents::forms::DocumentType;
use crate::forms::documents::similar::DocumentSimilar;
use crate::forms::searcher::s_params::SearchParams;
use crate::services::searcher::elastic::context::ElasticContext;
use crate::services::searcher::elastic::helper;
use crate::services::searcher::elastic::searcher::helper as s_helper;
use crate::services::searcher::service::{PaginatedResult, SearcherService};

use serde_json::Value;

#[async_trait::async_trait]
impl SearcherService for ElasticContext {
    async fn search_records(&self, s_params: &SearchParams, doc_type: &DocumentType) -> PaginatedResult<Value> {
        let cxt_opts = self.get_options().as_ref();
        let elastic = self.get_cxt().read().await;
        let folders = s_params.get_folders(true);
        let indexes = folders.split(',').collect::<Vec<&str>>();
        let paginated =
            s_helper::search_all::<Document>(&elastic, s_params, cxt_opts, indexes.as_slice()).await?;
        Ok(helper::to_unified_pag(paginated, doc_type))
    }
    async fn search_fulltext(&self, s_params: &SearchParams, doc_type: &DocumentType) -> PaginatedResult<Value> {
        let cxt_opts = self.get_options().as_ref();
        let elastic = self.get_cxt().read().await;
        let folders = s_params.get_folders(true);
        let indexes = folders.split(',').collect::<Vec<&str>>();
        let paginated = 
            s_helper::search::<Document>(&elastic, s_params, cxt_opts, indexes.as_slice()).await?;
        Ok(helper::to_unified_pag(paginated, doc_type))
    }
    async fn search_similar(&self, s_params: &SearchParams, doc_type: &DocumentType) -> PaginatedResult<Value> {
        let cxt_opts = self.get_options().as_ref();
        let elastic = self.get_cxt().read().await;
        let folders = s_params.get_folders(true);
        let indexes = folders.split(',').collect::<Vec<&str>>();
        let similar= 
            s_helper::search::<DocumentSimilar>(&elastic, s_params, cxt_opts, indexes.as_slice()).await?;
        let similar_docs = helper::similar_to_doc(similar);
        Ok(helper::to_unified_pag(similar_docs, doc_type))
    }
    async fn search_semantic(&self, s_params: &SearchParams, doc_type: &DocumentType) -> PaginatedResult<Value> {
        let cxt_opts = self.get_options().as_ref();
        let elastic = self.get_cxt().read().await;
        let folders = s_params.get_folders(true);
        let indexes = folders.split(',').collect::<Vec<&str>>();
        let paginated = 
            s_helper::search::<DocumentVectors>(&elastic, s_params, cxt_opts, indexes.as_slice()).await?;
        
        match doc_type {
            DocumentType::GroupedVectors => Ok(helper::vec_to_grouped_value(paginated)),
            _ => Ok(helper::vec_to_value(paginated)),
        }
    }
}
