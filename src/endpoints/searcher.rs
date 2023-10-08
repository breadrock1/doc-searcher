use crate::context::SearchContext;
use crate::errors::{WebError, WebResponse};
use crate::wrappers::*;

use actix_web::{post, web};
use elasticsearch::http::response::Response;
use elasticsearch::{Elasticsearch, SearchParts};
use serde::Deserialize;
use serde_json::{json, Value};

#[post("/search")]
async fn search_all(
    cxt: web::Data<SearchContext>,
    form: web::Json<SearchParameters>,
) -> WebResponse<web::Json<Vec<Document>>> {
    let elastic = cxt.get_cxt().read().await;
    let es_parameters = &form.0;
    search_documents(&elastic, es_parameters, &["*"]).await
}

#[post("/search/{bucket_names}")]
async fn search_target(
    cxt: web::Data<SearchContext>,
    path: web::Path<String>,
    form: web::Json<SearchParameters>,
) -> WebResponse<web::Json<Vec<Document>>> {
    let elastic = cxt.get_cxt().read().await;
    let es_parameters = &form.0;
    let indexes: Vec<&str> = path.split(',').collect();
    search_documents(&elastic, es_parameters, indexes.as_slice()).await
}

async fn search_documents(
    elastic: &Elasticsearch,
    es_parameters: &SearchParameters,
    indexes: &[&str],
) -> WebResponse<web::Json<Vec<Document>>> {
    let result_size = es_parameters.result_size;
    let result_offset = es_parameters.result_offset;
    let body_value = build_search_query(es_parameters);
    let response_result = elastic
        .search(SearchParts::Index(indexes))
        .from(result_offset)
        .size(result_size)
        .body(body_value)
        .allow_no_indices(true)
        .send()
        .await;

    match response_result {
        Err(err) => {
            let web_err = WebError::SearchFailed(err.to_string());
            Err(web_err)
        }
        Ok(response) => {
            let documents = parse_search_result(response).await;
            Ok(web::Json(documents))
        }
    }
}

async fn parse_search_result(response: Response) -> Vec<Document> {
    let common_object = response.json::<Value>().await.unwrap();
    let document_json = &common_object[&"hits"][&"hits"];
    let own_document = document_json.to_owned();
    let default_vec: Vec<Value> = Vec::default();
    let json_array = own_document.as_array().unwrap_or(&default_vec);

    json_array
        .iter()
        .map(|value| value[&"_source"].to_owned())
        .map(Document::deserialize)
        .map(Result::ok)
        .filter(Option::is_some)
        .flatten()
        .collect()
}

fn build_search_query(parameters: &SearchParameters) -> Value {
    let doc_size_to = parameters.document_size_to;
    let doc_size_from = parameters.document_size_from;
    let doc_size_query = DocumentSizeQuery::new(doc_size_from, doc_size_to);
    let doc_size_value = serde_json::to_value(doc_size_query).unwrap();

    let common_filter = json!({
        "bool": {
            "must": {
                "range": {
                    "document_size": doc_size_value,
                },
            },
            "should": {
                "term": {
                    "document_extension": "*"
                }
            }
        }
    });

    let query_str = QueryString::new(parameters.query.clone());
    let match_query = MultiMatchQuery::new(query_str);
    let match_value = serde_json::to_value(match_query).unwrap();

    json!({
        "query": {
            "bool": {
                "must": match_value,
                "filter": common_filter
            }
        }
    })
}
