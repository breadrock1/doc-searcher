use crate::errors::{WebError, WebErrorEntity};
use crate::forms::documents::preview::DocumentPreview;
use crate::forms::documents::DocumentsTrait;
use crate::forms::pagination::pagination::Paginated;
use crate::forms::searcher::s_params::SearchParams;
use crate::services::searcher::elastic::context::ContextOptions;
use crate::services::searcher::elastic::searcher::extractor::SearcherTrait;

use elasticsearch::http::response::Response;
use elasticsearch::{Elasticsearch, SearchParts};
use serde_json::Value;

pub(crate) async fn search<T>(
    elastic: &Elasticsearch,
    s_params: &SearchParams,
    cxt_opts: &ContextOptions,
    indexes: &[&str],
) -> Result<Paginated<Vec<T>>, WebError>
where
    T: DocumentsTrait + SearcherTrait<T>,
{
    let body_value = T::build_query(s_params, cxt_opts).await;
    let response = send_search_request(elastic, s_params, &body_value, indexes).await?;
    if !response.status_code().is_success() {
        let msg = response.json::<Value>().await.unwrap();
        let msg = serde_json::to_string_pretty(&msg).unwrap();
        let entity = WebErrorEntity::new(msg);
        return Err(WebError::SearchError(entity));
    }
    Ok(extract_elastic_response(response).await)
}

pub(crate) async fn search_all<T>(
    elastic: &Elasticsearch,
    s_params: &SearchParams,
    cxt_opts: &ContextOptions,
    indexes: &[&str],
) -> Result<Paginated<Vec<T>>, WebError>
where
    T: DocumentsTrait + SearcherTrait<T>,
{
    let body_value = DocumentPreview::build_query(s_params, cxt_opts).await;
    let response = send_search_request(elastic, s_params, &body_value, indexes).await?;
    if !response.status_code().is_success() {
        let msg = response.json::<Value>().await.unwrap();
        let msg = serde_json::to_string_pretty(&msg).unwrap();
        let entity = WebErrorEntity::new(msg.to_string());
        return Err(WebError::SearchError(entity));
    }
    Ok(extract_elastic_response::<T>(response).await)
}

pub(crate) async fn send_search_request(
    elastic: &Elasticsearch,
    es_params: &SearchParams,
    body_value: &Value,
    indexes: &[&str],
) -> Result<Response, elasticsearch::Error> {
    let (result_size, result_offset) = es_params.get_results_params();
    elastic
        .search(SearchParts::Index(indexes))
        .from(result_offset)
        .size(result_size)
        .body(body_value)
        .pretty(true)
        .scroll(es_params.get_scroll())
        .allow_no_indices(true)
        .send()
        .await
}

pub(crate) async fn extract_elastic_response<T>(response: Response) -> Paginated<Vec<T>>
where
    T: DocumentsTrait + SearcherTrait<T>,
{
    let common_object = response.json::<Value>().await.unwrap();
    let document_json = &common_object[&"hits"][&"hits"];
    let scroll_id = common_object[&"_scroll_id"]
        .as_str()
        .map_or_else(|| None, |x| Some(x.to_string()));

    let own_document = document_json.to_owned();
    let default_vec: Vec<Value> = Vec::default();
    let json_array = own_document.as_array().unwrap_or(&default_vec);

    let mut extracted_values: Vec<Result<T, WebError>> = Vec::default();
    for doc_value in json_array.iter() {
        extracted_values.push(T::extract_from_response(doc_value).await);
    }

    let founded_documents = extracted_values
        .into_iter()
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect::<Vec<T>>();

    Paginated::new_with_opt_id(founded_documents, scroll_id)
}
