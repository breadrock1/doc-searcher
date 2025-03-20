use elasticsearch::http::response::Response;
use elasticsearch::Elasticsearch;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::engine::elastic::helper::extractor::SearchQueryBuilder;
use crate::engine::elastic::ElasticClient;
use crate::engine::error::PaginatedResult;
use crate::engine::form::{FulltextParams, SemanticParams};
use crate::engine::model::{Document, DocumentVectors, DocumentsTrait, Paginated};

#[async_trait::async_trait]
pub trait Searcher<T: DocumentsTrait + serde::Serialize> {
    type Params;

    async fn search(
        es_cxt: Arc<RwLock<Elasticsearch>>,
        query: &Value,
        params: &Self::Params,
    ) -> PaginatedResult<T>;
}

#[async_trait::async_trait]
impl Searcher<Document> for Document {
    type Params = FulltextParams;

    async fn search(
        es_cxt: Arc<RwLock<Elasticsearch>>,
        query: &Value,
        params: &Self::Params,
    ) -> PaginatedResult<Document> {
        let scroll = params.scroll_lifetime();
        let results = params.result_size();
        let indexes = params.folder_ids().split(',').collect::<Vec<&str>>();
        let response =
            ElasticClient::search_request(es_cxt, query, Some(scroll), &indexes, results).await?;
        let documents = extract_searcher_result::<Document>(response).await?;
        Ok(documents)
    }
}

#[async_trait::async_trait]
impl Searcher<DocumentVectors> for DocumentVectors {
    type Params = SemanticParams;

    async fn search(
        es_cxt: Arc<RwLock<Elasticsearch>>,
        query: &Value,
        params: &Self::Params,
    ) -> PaginatedResult<DocumentVectors> {
        let scroll = params.scroll_lifetime();
        let size = params.result_size();
        let indexes = params.folder_ids().split(',').collect::<Vec<&str>>();
        let response =
            ElasticClient::search_knn_request(es_cxt, query, Some(scroll), &indexes, size).await?;
        let documents = extract_searcher_result::<DocumentVectors>(response).await?;
        Ok(documents)
    }
}

pub(crate) async fn extract_searcher_result<T>(response: Response) -> PaginatedResult<T>
where
    T: SearchQueryBuilder<T> + DocumentsTrait + serde::Serialize,
{
    let common_object = response.json::<Value>().await?;
    let document_json = &common_object[&"hits"][&"hits"];
    let scroll_id = common_object[&"_scroll_id"]
        .as_str()
        .map_or_else(|| None, |x| Some(x.to_string()));

    let default_vec: &Vec<Value> = &Vec::default();
    let own_document = document_json.to_owned();
    let json_array = own_document.as_array().unwrap_or_else(|| {
        tracing::warn!("returned empty json array from elastic search result");
        default_vec
    });

    let mut extracted_values = Vec::default();
    for doc_value in json_array.iter() {
        let extracted = T::extract_from_response(doc_value).await;
        extracted_values.push(extracted);
    }

    let founded_documents = extracted_values.into_iter().flatten().collect::<Vec<T>>();

    Ok(Paginated::new_with_opt_id(founded_documents, scroll_id))
}
