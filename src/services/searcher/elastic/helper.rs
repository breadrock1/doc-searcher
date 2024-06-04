use crate::errors::{Successful, WebError, WebResult};
use crate::services::searcher::elastic::context::ContextOptions;

use elasticsearch::http::headers::HeaderMap;
use elasticsearch::http::response::Response;
use elasticsearch::http::Method;
use elasticsearch::Elasticsearch;
use serde_json::{json, Value};

pub(crate) async fn send_llm_request(cxt_opts: &ContextOptions, query: &str) -> Vec<f64> {
    let target_url = format!("{}/embed", cxt_opts.llm_address());
    let response_result = reqwest::Client::new()
        .post(target_url)
        .json(&json!({
            "inputs": query,
            "normalize": false,
            "truncate": false
        }))
        .send()
        .await;

    if response_result.is_err() {
        let err = response_result.err().unwrap();
        log::error!("Failed to get embeddings: {}", err);
        return Vec::default();
    }

    let response = response_result
        .unwrap()
        .json::<Vec<Vec<f64>>>()
        .await
        .unwrap();

    let data1 = response.first().unwrap();
    data1.to_vec()
}

pub(crate) async fn send_elrequest(
    elastic: &Elasticsearch,
    method: Method,
    body: Option<&[u8]>,
    target_url: &str,
) -> Result<Response, WebError> {
    let response = elastic
        .send(
            method,
            target_url,
            HeaderMap::new(),
            Option::<&Value>::None,
            body,
            None,
        )
        .await
        .map_err(WebError::from)?;

    match response.status_code().is_success() {
        false => Err(extract_exception(response).await),
        true => Ok(response),
    }
}

pub(crate) async fn parse_elastic_response(response: Response) -> WebResult<Successful> {
    if !response.status_code().is_success() {
        return Err(extract_exception(response).await);
    }

    let txt = response.text().await.unwrap();
    log::warn!("{}", txt.as_str());
    Ok(Successful::success(txt.as_str()))
}

pub(crate) async fn extract_exception(response: Response) -> WebError {
    return WebError::UnknownError(response.text().await.unwrap());
    // let exception_res = response.exception().await;
    // if exception_res.is_err() {
    //     let err = exception_res.err().unwrap();
    //     return WebError::UnknownError(err.to_string());
    // }
    // 
    // match exception_res.unwrap() {
    //     None => WebError::UnknownError("Unknown error".to_string()),
    //     Some(exception) => WebError::from(exception),
    // }
}
