use crate::context::SearchContext;
use crate::errors::{WebError, WebResponse};
use crate::wrappers::document::{Document, HighlightEntity};
use crate::wrappers::search_params::*;

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
    let build_query_fn = |params: &SearchParameters| -> Value { build_search_query(params) };
    search_documents(&elastic, es_parameters, &["*"], &build_query_fn).await
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
    let build_query_fn = |params: &SearchParameters| -> Value { build_search_query(params) };
    search_documents(&elastic, es_parameters, indexes.as_slice(), &build_query_fn).await
}

#[post("/search-similar")]
async fn search_similar_docs(
    cxt: web::Data<SearchContext>,
    form: web::Json<SearchParameters>,
) -> WebResponse<web::Json<Vec<Document>>> {
    let elastic = cxt.get_cxt().read().await;
    let es_parameters = &form.0;
    let build_query_fn =
        |params: &SearchParameters| -> Value { build_search_similar_query(params) };
    search_documents(&elastic, es_parameters, &["*"], &build_query_fn).await
}

#[post("/search-similar/{bucket_names}")]
async fn search_similar_docs_target(
    cxt: web::Data<SearchContext>,
    path: web::Path<String>,
    form: web::Json<SearchParameters>,
) -> WebResponse<web::Json<Vec<Document>>> {
    let elastic = cxt.get_cxt().read().await;
    let es_parameters = &form.0;
    let indexes: Vec<&str> = path.split(',').collect();
    let build_query_fn =
        |params: &SearchParameters| -> Value { build_search_similar_query(params) };
    search_documents(&elastic, es_parameters, indexes.as_slice(), &build_query_fn).await
}

async fn search_documents(
    elastic: &Elasticsearch,
    es_parameters: &SearchParameters,
    indexes: &[&str],
    build_query_fn: &dyn Fn(&SearchParameters) -> Value,
) -> WebResponse<web::Json<Vec<Document>>> {
    let result_size = es_parameters.result_size;
    let result_offset = es_parameters.result_offset;
    let body_value = build_query_fn(es_parameters);
    let response_result = elastic
        .search(SearchParts::Index(indexes))
        .from(result_offset)
        .size(result_size)
        .body(body_value)
        .pretty(true)
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
        .map(parse_document_highlight)
        .map(Result::ok)
        .filter(Option::is_some)
        .flatten()
        .collect()
}

fn parse_document_highlight(value: &Value) -> Result<Document, serde_json::Error> {
    let source_value = value[&"_source"].to_owned();
    let mut document = Document::deserialize(source_value)?;

    let highlight_value = value[&"highlight"].to_owned();
    let highlight_entity = HighlightEntity::deserialize(highlight_value).ok();

    document.append_highlight(highlight_entity);
    Ok(document)
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
                    "document_type": "*",
                    "document_path": "*",
                    "document_extension": "*",
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
        },
        "highlight" : {
            "fields" : {
                "entity_data" : {
                    "fragment_size" : 3
                }
            }
        }
    })
}

fn build_search_similar_query(parameters: &SearchParameters) -> Value {
    let ssdeep_hash = &parameters.query;
    println!("Need find by this: {:?}", ssdeep_hash);
    json!({
        "query": {
            "more_like_this" : {
                "fields" : [
                    "entity_data",
                    "document_ssdeep_hash",
                ],
                "like" : ssdeep_hash,
                "min_doc_freq": 1,
                "min_term_freq" : 1,
                "max_query_terms" : 25,
            }
        }
    })
}

#[cfg(test)]
mod documents_endpoints {
    use crate::context::SearchContext;
    use crate::es_client::{build_elastic, build_service, init_service_parameters};
    use crate::wrappers::document::Document;
    use crate::wrappers::search_params::*;

    use actix_web::test::TestRequest;
    use actix_web::{test, web, App};
    use serde_json::json;

    #[test]
    async fn build_application() {
        let service_parameters = init_service_parameters().unwrap();
        let es_host = service_parameters.es_host();
        let es_user = service_parameters.es_user();
        let es_passwd = service_parameters.es_passwd();
        let service_port = service_parameters.service_port();
        let service_addr = service_parameters.service_address();

        let elastic = build_elastic(es_host, es_user, es_passwd).unwrap();
        let cxt = SearchContext::_new(elastic);
        let app = App::new()
            .app_data(web::Data::new(cxt))
            .service(build_service());

        let test_app = test::init_service(app).await;
        let test_bucket_name = "test_bucket";
        for document_index in 1..5 {
            let document_size = 1024 + document_index;
            let test_document_name = &format!("test_document_{}", document_index);
            let create_document_resp = TestRequest::post()
                .uri("/searcher/document/new")
                .set_json(&json!({
                    "bucket_uuid": test_bucket_name,
                    "bucket_path": "/tmp/test_document",
                    "document_name": test_document_name,
                    "document_path": "/tmp/dir/",
                    "document_size": document_size,
                    "document_type": "document",
                    "document_extension": ".docx",
                    "document_permissions": 777,
                    "document_created": "2023-09-15T00:00:00Z",
                    "document_modified": "2023-09-15T00:00:00Z",
                    "document_md5_hash": test_document_name,
                    "document_ssdeep_hash": "3a:34gh5",
                    "entity_data": "Using skip_serializing does not skip deserializing the field.",
                    "entity_keywords": ["document", "report"]
                }))
                .send_request(&test_app)
                .await;
        }

        // Found documents request by document name
        let mut search_params = SearchParameters::default();
        search_params.query = "document".to_string();
        let search_resp = TestRequest::post()
            .uri("/searcher/search")
            .set_json(&search_params)
            .send_request(&test_app)
            .await;

        let founded_documents: Vec<Document> = test::read_body_json(search_resp).await;
        assert_eq!(founded_documents.len() > 0, true);

        // Found documents request by document name with filter
        let mut search_params = SearchParameters::default();
        search_params.query = "document".to_string();
        search_params.document_size_from = 1026;
        let search_resp = TestRequest::post()
            .uri("/searcher/search")
            .set_json(&search_params)
            .send_request(&test_app)
            .await;

        let founded_documents: Vec<Document> = test::read_body_json(search_resp).await;
        assert_eq!(founded_documents.len() >= 1, true);

        // Found documents request by document name and bucket name
        let mut search_params = SearchParameters::default();
        search_params.query = "document".to_string();
        let search_resp = TestRequest::post()
            .uri(&format!("/searcher/search/{}", test_bucket_name))
            .set_json(&search_params)
            .send_request(&test_app)
            .await;

        let founded_documents: Vec<Document> = test::read_body_json(search_resp).await;
        assert_eq!(founded_documents.len() >= 4, true);

        // Found documents request by document name and bucket name
        let mut search_params = SearchParameters::default();
        search_params.query = "does not skip".to_string();
        let search_resp = TestRequest::post()
            .uri("/searcher/search")
            .set_json(&search_params)
            .send_request(&test_app)
            .await;

        let founded_documents: Vec<Document> = test::read_body_json(search_resp).await;
        assert_eq!(founded_documents.len() >= 4, true);
    }
}
