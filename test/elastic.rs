#[cfg(test)]
mod tests {
    use elasticsearch::{BulkParts, DeleteParts, Elasticsearch, IndexParts, SearchParts};
    use elasticsearch::auth::Credentials;
    use elasticsearch::cert::CertificateValidation;
    use elasticsearch::http::request::JsonBody;
    use elasticsearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
    use elasticsearch::http::Url;
    use serde_json::{json, Value};
    use tokio;

    fn build_elastic() -> Elasticsearch {
        let es_url = Url::parse("https://localhost:9200").unwrap();
        let conn_pool = SingleNodeConnectionPool::new(es_url);
        let creds = Credentials::Basic("elastic".into(), "MomQSpuUJk0ANHBTjSKM".into());
        let validation = CertificateValidation::None;
        let transport = TransportBuilder::new(conn_pool)
            .auth(creds)
            .cert_validation(validation)
            .build()
            .unwrap();

        Elasticsearch::new(transport)
    }

    #[tokio::test]
    async fn create_index() {
        let es_client = build_elastic();
        let response = es_client
            .index(IndexParts::IndexId("documents", "1"))
            .body(json!({
                "documents": {
                    "_source": { "enabled": false },
                    "properties": {
                        "bucket_uuid": { "type": "string" },
                        "bucket_path": { "type": "string" },
                        "document_name": { "type": "string" },
                        "document_path": { "type": "string" },
                        "document_size": { "type": "integer" },
                        "document_type": { "type": "string" },
                        "document_extension": { "type": "string" },
                        "document_permissions": { "type": "integer" },
                        "document_created": { "type": "date" },
                        "document_modified": { "type": "date" },
                        "document_md5_hash": { "type": "string" },
                        "document_ssdeep_hash": { "type": "string" },
                        "entity_keywords": [],
                    }
                }
            }))
            .send()
            .await
            .unwrap();

        let successful = response.status_code().is_success();
        assert_eq!(successful, true);
    }

    #[tokio::test]
    async fn push_documents() {
        let es_client = build_elastic();
        let mut body: Vec<JsonBody<_>> = Vec::with_capacity(6);
        body.push(json!({"index": { "_id": 1 }}).into());
        body.push(json!({
            "bucket_uuid": "Y2xvdWQtZW5kcG9pbnQuZ",
            "bucket_path": "/tmp/test_bucket",
            "document_name": "document_1",
            "document_path": "./",
            "document_size": 1024,
            "document_type": "document",
            "document_extension": ".docx",
            "document_permissions": 777,
            "document_created": "2023-09-15T00:00:00Z",
            "document_modified": "2023-09-15T00:00:00Z",
            "document_md5_hash": "79054025255fb1a26e4bc422aef54eb4",
            "document_ssdeep_hash": "3a:34gh5",
            "entity_keywords": ["document", "report"],
        }).into());

        body.push(json!({"index": { "_id": 2 }}).into());
        body.push(json!({
            "bucket_uuid": "W5kcG9pbnQuZXhhbXB",
            "bucket_path": "/tmp/test_bucket",
            "document_name": "document_2",
            "document_path": "./",
            "document_size": 4096,
            "document_type": "document",
            "document_extension": ".docx",
            "document_permissions": 777,
            "document_created": "2023-09-15T00:00:00Z",
            "document_modified": "2023-09-15T00:00:00Z",
            "document_md5_hash": "77dfmg8s5255fb1a26e4bc422aef54eb4",
            "document_ssdeep_hash": "3a:34gh5",
            "entity_keywords": ["document", "report"],
        }).into());

        body.push(json!({"index": { "_id": 3 }}).into());
        body.push(json!({
            "bucket_uuid": "zZGFkZjgyM2YwNTM4ODQ5N2V",
            "bucket_path": "/tmp/test_bucket",
            "document_name": "document_3",
            "document_path": "./",
            "document_size": 2048,
            "document_type": "document",
            "document_extension": ".docx",
            "document_permissions": 777,
            "document_created": "2022-08-15T00:00:00Z",
            "document_modified": "2023-08-15T00:00:00Z",
            "document_md5_hash": "79054025255fb1d8fb4bc422aef54eb4",
            "document_ssdeep_hash": "3a:34gh5",
            "entity_keywords": ["document", "report"],
        }).into());

        let response = es_client
            .bulk(BulkParts::Index("documents"))
            .body(body)
            .send()
            .await
            .unwrap();

        assert_eq!(response.status_code().is_success(), true);
    }

    #[tokio::test]
    async fn delete_document() {
        let es_client = build_elastic();
        let response = es_client
            .delete(DeleteParts::IndexId("documents", "1"))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status_code().is_success(), true);
    }

    #[tokio::test]
    async fn delete_documents() {
        let es_client = build_elastic();
        for document_id in 2..4 {
            let doc_id_str = document_id.to_string();
            let response = es_client
                .delete(DeleteParts::IndexId("documents", doc_id_str.as_str()))
                .send()
                .await
                .unwrap();

            assert_eq!(response.status_code().is_success(), true);
        }
    }

    #[tokio::test]
    async fn test_elastic_client() {
        let es_client = build_elastic();
        let response = es_client
            .search(SearchParts::Index(&["documents"]))
            .from(0)
            .size(10)
            .body(json!({
                "query": {
                    "match": {
                        "bucket_path": "/tmp/test_bucket",
                    }
                }
            }))
            .allow_no_indices(true)
            .send()
            .await
            .unwrap();

        let response_body = response.json::<Value>().await.unwrap();
        println!("{:?}", response_body);

        // let successful = response.status_code().is_success();
        // assert_eq!(successful, true);
    }
}