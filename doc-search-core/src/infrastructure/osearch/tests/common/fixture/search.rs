use rstest::fixture;
use serde_json::Value;

const SEARCHING_RESULT: &[u8] = include_bytes!("../../resources/searching-result.json");

#[fixture]
pub fn build_full_search_result() -> Value {
    serde_json::from_slice(SEARCHING_RESULT).expect("failed to load searching result fixture data")
}

#[fixture]
pub fn build_search_result_without_scroll_id() -> Value {
    let mut searching_result = build_full_search_result();
    searching_result["_scroll_id"] = Value::Null;
    searching_result
}

#[fixture]
pub fn build_search_result_without_metadata() -> Value {
    let mut searching_result = build_full_search_result();
    searching_result["hits"]["hits"]
        .as_array_mut()
        .expect("expected array of hits search result")
        .iter_mut()
        .for_each(|it| {
            let mut _object = it.as_object_mut().expect("failed to get object");
            _object["_source"]["metadata"] = Value::Null;
        });

    searching_result
}
