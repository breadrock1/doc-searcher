#[cfg(test)]
#[cfg(feature = "enable-cacher")]
mod test_redis_client {
    use doc_search::cacher::redis::RedisClient;
    use doc_search::cacher::CacherService;
    use doc_search::config::ServiceConfig;
    use doc_search::storage::models::Document;
    use doc_search::swagger::examples::TestExample;
    use doc_search::Connectable;

    #[tokio::test]
    async fn test_redis_cacher_client() -> Result<(), anyhow::Error> {
        let s_config = ServiceConfig::new()?;

        let cacher_config = s_config.cacher();
        let cacher = RedisClient::connect(cacher_config)?;

        let test_doc = Document::test_example(None);
        cacher.insert(test_doc.document_id(), &test_doc).await;

        let cached_doc_opt: Option<Document> = cacher.load(test_doc.document_id()).await;
        assert!(cached_doc_opt.is_some());

        let cached_doc = cached_doc_opt.unwrap();
        assert_eq!(cached_doc.document_id(), "98ac9896be35f47fb8442580cd9839b4");

        Ok(())
    }
}
