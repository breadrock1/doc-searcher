use serde::Deserialize;
use serde_json::{json, Value};

use crate::application::dto::{
    Document, FullTextSearchParams, Paginated, QueryBuilder, RetrieveDocumentParams,
    SemanticSearchParams, SemanticSearchWithTokensParams,
};
use crate::application::services::storage::error::StorageResult;

impl QueryBuilder for RetrieveDocumentParams {
    fn build_query(&self) -> Value {
        let must = match self.path() {
            None => json!([{"match_all": {}}]),
            Some(path) => json!([
                {
                    "term": {
                        "file_path.keyword": path
                    }
                }
            ]),
        };

        json!({
            "query": {
                "bool": {
                    "must": must,
                    "filter": [
                        {
                            "range": {
                                "created_at": {
                                    "gte": "2024-01-01T00:00:00Z",
                                    "lte": "2024-06-25T23:59:59Z"
                                }
                            }
                        },
                        {
                            "range": {
                                "file_size": {
                                    "gte": 0,
                                    "lte": 1048576
                                }
                            }
                        }
                    ]
                }
            },
            "sort": [
                {
                    "created_at": {
                        "order": "desc"
                    }
                }
            ]
        })
    }
}

impl QueryBuilder for FullTextSearchParams {
    fn build_query(&self) -> Value {
        let query = match self.query() {
            None => "*",
            Some(value) => value,
        };

        json!({
            "query": {
                "bool": {
                    "match": {
                        "content": query,
                    },
                    "filter": [
                        {
                            "range": {
                                "created_at": {
                                    "gte": "2024-01-01T00:00:00Z",
                                    "lte": "2024-06-25T23:59:59Z"
                                }
                            }
                        },
                        {
                            "range": {
                                "file_size": {
                                    "gte": 0,
                                    "lte": 1048576
                                }
                            }
                        }
                    ]
                }
            },
            "sort": [
                {
                    "created_at": {
                        "order": "desc"
                    }
                }
            ]
        })
    }
}

impl QueryBuilder for SemanticSearchParams {
    fn build_query(&self) -> Value {
        let size = self.result().size();
        let candidates = self.knn_candidates();
        let knn_amount = self.knn_amount();
        let query_vector = self.query();

        json!({
            "size": size,
            "knn": {
                "field": "embeddings.vector",
                "k": knn_amount,
                "num_candidates": candidates,
                "query_vector": query_vector
            }
        })
    }
}

impl QueryBuilder for SemanticSearchWithTokensParams {
    fn build_query(&self) -> Value {
        let size = self.result().size();
        let candidates = self.knn_candidates();
        let knn_amount = self.knn_amount();
        let query_vector = self.tokens();

        json!({
            "size": size,
            "knn": {
                "field": "embeddings.vector",
                "k": knn_amount,
                "num_candidates": candidates,
                "query_vector": query_vector
            }
        })
    }
}

pub(super) fn extract_document(value: &Value) -> StorageResult<Document> {
    let source_value = value[&"_source"].to_owned();
    let document = Document::deserialize(source_value)?;
    // let highlight_value = value[&"highlight"].to_owned();
    // let highlight_entity = HighlightEntity::deserialize(highlight_value).ok();
    // document.set_highlight(highlight_entity);
    Ok(document)
}

pub(super) async fn extract_founded_docs(
    common_object: Value,
) -> StorageResult<Paginated<Vec<Document>>> {
    let document_json = &common_object[&"hits"][&"hits"];
    let scroll_id = common_object[&"_scroll_id"]
        .as_str()
        .map_or_else(|| None, |x| Some(x.to_string()));

    let default_vec: &Vec<Value> = &Vec::default();
    let own_document = document_json.to_owned();
    let json_array = own_document.as_array().unwrap_or_else(|| {
        tracing::warn!("returned empty json array from elastic search result");
        default_vec
    });

    let documents = json_array
        .into_iter()
        .filter_map(|it| extract_document(it).ok())
        .collect::<Vec<Document>>();

    let documents = Paginated::builder()
        .scroll_id(scroll_id)
        .founded(documents)
        .build()
        .unwrap();

    Ok(documents)
}
