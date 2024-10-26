use doc_search::searcher::elastic::extractor::SearchQueryBuilder;
use doc_search::searcher::forms::{FulltextParams, SemanticParams};
use doc_search::storage::elastic::retrieve::Retrieve;
use doc_search::storage::forms::RetrieveParams;
use doc_search::storage::models::{Document, DocumentVectors, InfoFolder};

const DOCUMENT_QUERY: &str = "{\"_source\":{\"exclude\":[\"embeddings\"]},\"highlight\":{\"fields\":{\"content\":{\"post_tags\":[\"\"],\"pre_tags\":[\"\"]}},\"order\":\"\"},\"query\":{\"bool\":{\"filter\":{\"bool\":{\"must\":[{\"range\":{\"document_created\":{\"gte\":\"2025-04-26T11:14:55Z\",\"lte\":\"2024-04-26T11:14:55Z\"}}},{\"range\":{\"document_size\":{\"gte\":4096,\"lte\":0}}},{\"term\":{\"document_extension\":\"txt\"}},{\"term\":{\"document_type\":\"document\"}}]}},\"must\":{\"multi_match\":{\"fields\":[\"content\",\"document_path\"],\"query\":\"Some query\"}}}}}";
const SEMANTIC_QUERY: &str = "{\"knn\":{\"field\":\"embeddings.vector\",\"k\":5,\"num_candidates\":100,\"query_vector\":[]},\"size\":0}";
const ALL_RECORDS_QUERY: &str = "{\"query\":{\"bool\":{\"filter\":{\"bool\":{\"must\":[{\"exists\":{\"field\":\"folder_type\"}}]}},\"must\":{\"match_all\":{}}}}}";

#[tokio::test]
async fn test_fulltext_build_query() -> Result<(), anyhow::Error> {
    let s_params = build_fulltext_params();
    let build_query = Document::build_search_query(&s_params).await;
    let query = serde_json::to_string(&build_query)?;
    assert_eq!(DOCUMENT_QUERY, query);
    Ok(())
}

#[tokio::test]
async fn test_semantic_build_query() -> Result<(), anyhow::Error> {
    let s_params = build_semantic_params();
    let build_query = DocumentVectors::build_search_query(&s_params).await;
    let query = serde_json::to_string(&build_query)?;
    assert_eq!(SEMANTIC_QUERY, query);
    Ok(())
}

#[tokio::test]
async fn test_retrieve_build_query() -> Result<(), anyhow::Error> {
    let s_params = build_retrieve_params();
    let build_query = InfoFolder::build_retrieve_query(&s_params).await;
    let query = serde_json::to_string(&build_query)?;
    assert_eq!(ALL_RECORDS_QUERY, query);
    Ok(())
}

fn build_semantic_params() -> SemanticParams {
    SemanticParams::builder()
        .query("Some query".to_string())
        .query_tokens(None)
        .folder_ids("test-folder-vector".to_string())
        .document_size_to(37000)
        .document_size_from(0)
        .scroll_lifetime("1m".to_string())
        .knn_amount(Some(5))
        .knn_candidates(Some(100))
        .is_grouped(Some(false))
        .build()
        .unwrap()
}

fn build_fulltext_params() -> FulltextParams {
    FulltextParams::builder()
        .query("Some query".to_string())
        .folder_ids("test-folder-vector".to_string())
        .document_type(Some("document".to_string()))
        .document_extension(Some("txt".to_string()))
        .created_date_to(Some("2024-04-26T11:14:55Z".to_string()))
        .created_date_from(Some("2025-04-26T11:14:55Z".to_string()))
        .scroll_lifetime("1m".to_string())
        .document_size_to(Some(0))
        .document_size_from(Some(4096))
        .result_size(25)
        .result_offset(0)
        .build()
        .unwrap()
}

fn build_retrieve_params() -> RetrieveParams {
    RetrieveParams::builder()
        .query(Some("Some query".to_string()))
        .document_extension(Some("txt".to_string()))
        .created_date_to(Some("2024-04-26T11:14:55Z".to_string()))
        .created_date_from(Some("2025-04-26T11:14:55Z".to_string()))
        .document_size_to(Some(0))
        .document_size_from(Some(4096))
        .result_size(25)
        .result_offset(0)
        .is_show_all(true)
        .build()
        .unwrap()
}
