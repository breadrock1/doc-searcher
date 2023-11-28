use crate::endpoints::ContextData;
use crate::errors::WebResponse;
use crate::wrappers::document::Document;
use crate::wrappers::search_params::*;

use actix_web::{post, web};

#[post("/search-similar")]
async fn search_similar_docs(
    cxt: ContextData,
    form: web::Json<SearchParams>,
) -> WebResponse<web::Json<Vec<Document>>> {
    let client = cxt.get_ref();
    let search_form = form.0;
    client.similar_all(&search_form).await
}

#[post("/search-similar/{bucket_names}")]
async fn search_similar_docs_target(
    cxt: ContextData,
    path: web::Path<String>,
    form: web::Json<SearchParams>,
) -> WebResponse<web::Json<Vec<Document>>> {
    let client = cxt.get_ref();
    let search_form = form.0;
    let buckets = path.as_ref();
    client.similar_bucket(buckets.as_str(), &search_form).await
}

#[cfg(test)]
mod similarities_endpoints {
    use crate::searcher::elastic::build_elastic_client;
    use crate::searcher::elastic::context::ElasticContext;
    use crate::service::{build_service, init_service_parameters};
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

        let elastic = build_elastic_client(es_host, es_user, es_passwd).unwrap();
        let cxt = ElasticContext::_new(elastic);
        let app = App::new()
            .app_data(web::Data::new(cxt))
            .service(build_service());

        let test_app = test::init_service(app).await;
        let test_bucket_name = "test_bucket";
        for document_index in 1..5 {
            let document_size = 1024 + document_index;
            let test_document_name = &format!("test_document_{}", document_index);
            let _create_document_resp = TestRequest::post()
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
        let mut search_params = SearchParams::default();
        search_params.query = "document".to_string();
        let search_resp = TestRequest::post()
            .uri("/searcher/search")
            .set_json(&search_params)
            .send_request(&test_app)
            .await;

        let founded_documents: Vec<Document> = test::read_body_json(search_resp).await;
        assert_eq!(founded_documents.len() > 0, true);

        // Found documents request by document name with filter
        let mut search_params = SearchParams::default();
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
        let mut search_params = SearchParams::default();
        search_params.query = "document".to_string();
        let search_resp = TestRequest::post()
            .uri(&format!("/searcher/search/{}", test_bucket_name))
            .set_json(&search_params)
            .send_request(&test_app)
            .await;

        let founded_documents: Vec<Document> = test::read_body_json(search_resp).await;
        assert_eq!(founded_documents.len() >= 4, true);

        // Found documents request by document name and bucket name
        let mut search_params = SearchParams::default();
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
