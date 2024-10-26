use crate::elastic::{ElasticClient, EsCxt};
use crate::searcher::elastic::extractor::SearchQueryBuilder;
use crate::searcher::errors::PaginatedResult;
use crate::searcher::forms::{FulltextParams, SemanticParams};
use crate::searcher::models::Paginated;
use crate::storage::models::{Document, DocumentVectors, DocumentsTrait};

use elasticsearch::http::response::Response;
use serde_json::Value;

#[async_trait::async_trait]
pub trait Searcher<T: DocumentsTrait + serde::Serialize> {
    type Params;

    async fn search(es_cxt: EsCxt, query: &Value, params: &Self::Params) -> PaginatedResult<T>;
}

#[async_trait::async_trait]
impl Searcher<Document> for Document {
    type Params = FulltextParams;

    async fn search(
        es_cxt: EsCxt,
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
        es_cxt: EsCxt,
        query: &Value,
        params: &Self::Params,
    ) -> PaginatedResult<DocumentVectors> {
        let scroll = params.scroll_lifetime();
        let results = params.result_size();
        let indexes = params.folder_ids().split(',').collect::<Vec<&str>>();
        let response =
            ElasticClient::search_request(es_cxt, query, Some(scroll), &indexes, results).await?;
        let documents = extract_searcher_result::<DocumentVectors>(response).await?;
        Ok(documents)
    }
}

pub(super) async fn extract_searcher_result<T>(response: Response) -> PaginatedResult<T>
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
