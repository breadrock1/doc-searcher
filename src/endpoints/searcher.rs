use crate::context::SearchContext;
use crate::errors::WebResponse;
use crate::wrappers::{
    Document, DocumentSizeQuery, MultiMatchQuery, QueryString, SearchParameters,
};

use actix_web::{post, web};
use elasticsearch::http::response::Response;
use elasticsearch::SearchParts;
use serde::Deserialize;
use serde_json::{json, Value};

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

        // "document_created":
    });

    // let doc_created_to = es_parameters.created_date_to;
    // let doc_created_from = es_parameters.created_date_from;
    // if doc_created_from > 0i64 || doc_created_from > 0i64 {
    //
    // }

    let query_str = QueryString::new(parameters.query.clone());
    let match_query = MultiMatchQuery::new(query_str);
    let match_value = serde_json::to_value(match_query).unwrap();

    let body_value = json!({
        "query": {
            "bool": {
                "must": match_value,
                "filter": common_filter
            }
        }
    });

    body_value
}

async fn parse_search_result(response: Response) -> Vec<Document> {
    let common_object = response.json::<Value>().await.unwrap();
    let document_json = &common_object[&"hits"][&"hits"];
    let json_array = document_json.as_array();
    if json_array.is_none() {
        return Vec::default();
    }

    let founded_documents: Vec<Document> = json_array
        .unwrap()
        .into_iter()
        .map(|value| value[&"_source"].to_owned())
        .map(Document::deserialize)
        .map(Result::ok)
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect();

    founded_documents
}

#[post("/search")]
async fn search_all(
    cxt: web::Data<SearchContext>,
    form: web::Json<SearchParameters>,
) -> WebResponse<web::Json<Vec<Document>>> {
    let elastic = cxt.get_cxt().lock().unwrap();
    let es_parameters = &form.0;
    let result_size = es_parameters.result_size;
    let result_offset = es_parameters.result_offset;
    let body_value = build_search_query(es_parameters);

    let response = elastic
        .search(SearchParts::Index(&["*"]))
        .from(result_offset)
        .size(result_size)
        .body(body_value)
        .allow_no_indices(true)
        .send()
        .await
        .unwrap();

    let documents = parse_search_result(response).await;
    Ok(web::Json(documents))
}

#[post("/search/{bucket_names}")]
async fn search_target(
    cxt: web::Data<SearchContext>,
    path: web::Path<String>,
    form: web::Json<SearchParameters>,
) -> WebResponse<web::Json<Vec<Document>>> {
    let elastic = cxt.get_cxt().lock().unwrap();
    let es_parameters = &form.0;
    let result_size = es_parameters.result_size;
    let result_offset = es_parameters.result_offset;
    let body_value = build_search_query(es_parameters);

    let indexes: Vec<&str> = path.split(",").collect();
    let response = elastic
        .search(SearchParts::Index(indexes.as_slice()))
        .from(result_offset)
        .size(result_size)
        .body(body_value)
        .allow_no_indices(true)
        .send()
        .await
        .unwrap();

    let documents = parse_search_result(response).await;
    Ok(web::Json(documents))
}
