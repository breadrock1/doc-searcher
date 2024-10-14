use doc_search::searcher::models::SearchParams;
use doc_search::searcher::SearcherTrait;
use doc_search::storage::models::{Document, DocumentPreview, DocumentVectors, InfoFolder};

const DOCUMENT_QUERY: &str = "{\"_source\":{\"exclude\":[\"embeddings\"]},\"highlight\":{\"fields\":{\"content\":{\"post_tags\":[\"\"],\"pre_tags\":[\"\"]}},\"order\":\"\"},\"query\":{\"bool\":{\"filter\":{\"bool\":{\"must\":[{\"range\":{\"document_created\":{\"gte\":\"2024-04-26T11:14:55Z\",\"lte\":\"2025-04-26T11:14:55Z\"}}},{\"range\":{\"document_size\":{\"gte\":0,\"lte\":37000}}},{\"term\":{\"document_extension\":\"txt\"}},{\"term\":{\"document_type\":\"document\"}}]}},\"must\":{\"multi_match\":{\"fields\":[\"content\",\"document_path\"],\"query\":\"Some query\"}}}}}";
const PREVIEW_QUERY: &str = "{\"_source\":{\"exclude\":[\"embeddings\"]},\"query\":{\"bool\":{\"filter\":{\"bool\":{\"must\":[{\"range\":{\"document_created\":{\"gte\":\"2024-04-26T11:14:55Z\",\"lte\":\"2025-04-26T11:14:55Z\"}}},{\"range\":{\"document_size\":{\"gte\":0,\"lte\":37000}}},{\"term\":{\"document_extension\":\"txt\"}}]}},\"should\":[{\"multi_match\":{\"fields\":[\"document_name\",\"document_path\"],\"minimum_should_match\":\"50%\",\"query\":\"Some query\",\"type\":\"phrase_prefix\"}}]}},\"sort\":[{\"document_created\":{\"format\":\"strict_date_optional_time_nanos\",\"order\":\"desc\"}}]}";
const SEMANTIC_QUERY: &str = "{\"knn\":{\"field\":\"embeddings.vector\",\"k\":5,\"num_candidates\":100,\"query_vector\":[]},\"size\":0}";
const ALL_RECORDS_QUERY: &str = "{\"query\":{\"bool\":{\"filter\":{\"bool\":{\"must\":[{\"exists\":{\"field\":\"folder_type\"}}]}},\"must\":{\"match_all\":{}}}}}";
const NON_SYSTEM_RECORDS_QUERY: &str = "{\"query\":{\"bool\":{\"filter\":{\"bool\":{\"must\":[{\"term\":{\"is_system\":\"false\"}},{\"exists\":{\"field\":\"folder_type\"}}]}},\"must\":{\"match_all\":{}}}}}";

#[tokio::test]
async fn test_document_build_query() -> Result<(), anyhow::Error> {
    let s_params = build_search_params();
    let build_query = Document::build_query(&s_params).await;
    let query = serde_json::to_string(&build_query)?;
    assert_eq!(DOCUMENT_QUERY, query);
    Ok(())
}

#[tokio::test]
async fn test_document_preview_build_query() -> Result<(), anyhow::Error> {
    let s_params = build_search_params();
    let build_query = DocumentPreview::build_query(&s_params).await;
    let query = serde_json::to_string(&build_query)?;
    assert_eq!(PREVIEW_QUERY, query);
    Ok(())
}

#[tokio::test]
async fn test_document_vectors_build_query() -> Result<(), anyhow::Error> {
    let s_params = build_search_params();
    let build_query = DocumentVectors::build_query(&s_params).await;
    let query = serde_json::to_string(&build_query)?;
    assert_eq!(SEMANTIC_QUERY, query);
    Ok(())
}

#[tokio::test]
async fn test_info_folder_system_build_query() -> Result<(), anyhow::Error> {
    let s_params = build_search_params();
    let build_query = InfoFolder::build_query(&s_params).await;
    let query = serde_json::to_string(&build_query)?;
    assert_eq!(ALL_RECORDS_QUERY, query);
    Ok(())
}

#[tokio::test]
async fn test_info_folder_build_query() -> Result<(), anyhow::Error> {
    let mut s_params = build_search_params();
    s_params.set_show_all(false);

    let build_query = InfoFolder::build_query(&s_params).await;
    let query = serde_json::to_string(&build_query)?;
    assert_eq!(NON_SYSTEM_RECORDS_QUERY, query);

    Ok(())
}

fn build_search_params() -> SearchParams {
    SearchParams::builder()
        .query("Some query".to_string())
        .query_tokens(Some(Vec::default()))
        .folder_ids(Some("test-folder-id".to_string()))
        .document_type("document".to_string())
        .document_extension("txt".to_string())
        .created_date_to("2025-04-26T11:14:55Z".to_string())
        .created_date_from("2024-04-26T11:14:55Z".to_string())
        .document_size_to(37000)
        .document_size_from(0)
        .result_size(25)
        .result_offset(0)
        .scroll_lifetime("1m".to_string())
        .knn_amount(Some(5))
        .knn_candidates(Some(100))
        .show_all(Some(true))
        .build()
        .unwrap()
}
