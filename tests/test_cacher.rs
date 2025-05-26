#[cfg(test)]
#[cfg(feature = "enable-cacher-redis")]
mod test_redis_client {
    use doc_search::cacher::redis::RedisClient;
    use doc_search::cacher::CacherService;
    use doc_search::config::ServiceConfig;
    use doc_search::engine::model::Document;
    use doc_search::ServiceConnect;

    #[tokio::test]
    async fn test_redis_cacher_client() -> anyhow::Result<()> {
        let s_config = ServiceConfig::new()?;

        let cacher_config = s_config.cacher().redis();
        let cacher = RedisClient::connect(cacher_config).await?;

        let test_doc = Document::default();
        cacher.insert(test_doc.document_id(), &test_doc).await;

        let cached_doc_opt: Option<Document> = cacher.load(test_doc.document_id()).await;
        assert!(cached_doc_opt.is_some());

        let cached_doc = cached_doc_opt.unwrap();
        assert_eq!(cached_doc.document_id(), "98ac9896be35f47fb8442580cd9839b4");

        Ok(())
    }
}
