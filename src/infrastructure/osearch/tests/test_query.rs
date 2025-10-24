use rstest::*;
use serde_json::json;

use crate::application::structures::params::*;
use crate::application::tests::fixture::params::*;
use crate::infrastructure::osearch::query::{QueryBuilder, QueryBuilderParams};

#[rstest]
fn test_build_simplest_query_from_retrieve_params(
    #[from(build_simple_retrieve_params)] params: RetrieveDocumentParams,
) -> anyhow::Result<()> {
    let query_params = QueryBuilderParams::from(&params);
    let query = params.build_query(query_params);

    let compare_query = build_comparable_query();
    assert_eq!(query, compare_query);

    Ok(())
}

#[rstest]
fn test_build_with_path_query_from_retrieve_params(
    #[from(build_with_path_retrieve_params)] params: RetrieveDocumentParams,
) -> anyhow::Result<()> {
    let query_params = QueryBuilderParams::from(&params);
    let query = params.build_query(query_params);

    let mut compare_query = build_comparable_query();
    compare_query["query"] = json!({
        "bool": {
            "filter": [],
            "must": [
                {
                    "match": {
                        "file_path": DOCUMENT_PATH,
                    }
                }
            ],
        }
    });

    assert_eq!(query, compare_query);

    Ok(())
}

#[rstest]
fn test_build_with_filter_query_from_retrieve_params(
    #[from(build_with_filter_retrieve_params)] params: RetrieveDocumentParams,
) -> anyhow::Result<()> {
    let query_params = QueryBuilderParams::from(&params);
    let query = params.build_query(query_params);

    let mut compare_query = build_comparable_query();
    compare_query["query"]["bool"]["filter"] = json!([
        {
            "range": {
                "created_at": {
                    "gte": CURRENT_TIMESTAMP,
                    "lte": CURRENT_TIMESTAMP,
                }
            }
        },
        {
            "range": {
                "file_size": {
                    "gte": 0,
                    "lte": 4096,
                }
            }
        },
    ]);

    assert_eq!(query, compare_query);

    Ok(())
}

#[rstest]
fn test_build_simple_fulltext_search_params(
    #[from(build_simple_fulltext_params)] params: FullTextSearchParams,
) -> anyhow::Result<()> {
    let query_params = QueryBuilderParams::from(&params);
    let query = params.build_query(query_params);

    let mut compare_query = build_comparable_query();
    compare_query["highlight"] = json!({
        "fields": {
            "content": {
                "post_tags": [""],
                "pre_tags": [""],
            }
        }
    });
    assert_eq!(query, compare_query);

    Ok(())
}

#[rstest]
fn test_build_with_path_query_from_fulltext_params(
    #[from(build_with_path_retrieve_params)] params: RetrieveDocumentParams,
) -> anyhow::Result<()> {
    let query_params = QueryBuilderParams::from(&params);
    let query = params.build_query(query_params);

    let mut compare_query = build_comparable_query();
    compare_query["query"] = json!({
        "bool": {
            "filter": [],
            "must": [
                {
                    "match": {
                        "file_path": DOCUMENT_PATH,
                    }
                }
            ],
        }
    });

    assert_eq!(query, compare_query);

    Ok(())
}

#[rstest]
fn test_build_with_filter_query_from_fulltext_params(
    #[from(build_with_filter_retrieve_params)] params: RetrieveDocumentParams,
) -> anyhow::Result<()> {
    let query_params = QueryBuilderParams::from(&params);
    let query = params.build_query(query_params);

    let mut compare_query = build_comparable_query();
    compare_query["query"]["bool"]["filter"] = json!([
        {
            "range": {
                "created_at": {
                    "gte": CURRENT_TIMESTAMP,
                    "lte": CURRENT_TIMESTAMP,
                }
            }
        },
        {
            "range": {
                "file_size": {
                    "gte": 0,
                    "lte": 4096,
                }
            }
        },
    ]);

    println!("{:?}", compare_query);
    assert_eq!(query, compare_query);

    Ok(())
}
