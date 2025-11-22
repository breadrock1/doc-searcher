use rstest::fixture;
use serde_json::{Value, json};

pub const INDEX_ID: &str = "test-folder";
pub const DOCUMENT_ID: &str = "29346839246dsf987a1173sfa7sd781h";
pub const DOCUMENT_PART_ID: &str = "kwejbrw46dsf987a1173sfa7sd781h";
pub const SCROLL_ID: &str = "dksfsjvJHZVFDskjdbfsdfsdfdsg";

#[fixture]
pub fn build_osearch_search_result() -> Value {
    json!({
        "_scroll_id": SCROLL_ID,
        "hits": {
            "hits": [
                {
                    "_id": DOCUMENT_PART_ID,
                    "_index": INDEX_ID,
                    "_score": 0.7,
                    "highlight": {
                        "content": [
                            "There is some highlight",
                        ]
                    },
                    "_source": {
                        "large_doc_id": DOCUMENT_ID,
                        "file_name": "test-document.docx",
                        "file_path": "./test-document.docx",
                        "file_size": 1024,
                        "created_at": 1750957215,
                        "modified_at": 1750957215,
                        "content": "There is some highlight content",
                        "chunked_text": [],
                        "embeddings": [],
                        "doc_part_id": 0,
                    }
                }
            ]
        }
    })
}
