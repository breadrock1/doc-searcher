extern crate elquery;

#[cfg(test)]
mod test {
    use elquery::filter_query::{CommonFilter, FilterRange, FilterTerm};
    use elquery::highlight_query::HighlightOrder;
    use elquery::search_query::MultiMatchQuery;
    use elquery::similar_query::SimilarQuery;

    #[test]
    fn build_highlight_query() {
        let highlight_order = HighlightOrder::default();
        let query_value = serde_json::to_value(highlight_order).unwrap();
        let query_string = serde_json::to_string(&query_value).unwrap();
        assert_eq!(
            "{\"fields\":{\"content\":{\"post_tags\":[\"\"],\"pre_tags\":[\"\"]}},\"order\":\"\"}",
            query_string
        );
    }

    #[test]
    fn build_search_query() {
        let match_query = MultiMatchQuery::new("Some query string");
        let query_value = serde_json::to_value(match_query).unwrap();
        let query_string = serde_json::to_string(&query_value).unwrap();
        assert_eq!("{\"multi_match\":{\"fields\":[\"content\",\"document_path\"],\"operator\":\"or\",\"query\":\"Some query string\"}}", query_string);
    }

    #[test]
    fn build_similar_query() {
        let some_hash = "ssdeep_hash".to_string();
        let some_field = vec!["hash_dield".to_string()];
        let similar_query = SimilarQuery::new(some_hash, some_field);
        let query_value = serde_json::to_value(similar_query).unwrap();
        let query_string = serde_json::to_string(&query_value).unwrap();
        assert_eq!("{\"more_like_this\":{\"fields\":[\"hash_dield\"],\"like\":\"ssdeep_hash\",\"max_query_terms\":25,\"min_doc_freq\":1,\"min_term_freq\":1}}", query_string);
    }

    #[test]
    fn build_filter_query() {
        // let some_date_str = "2022-11-23 00:00:00";
        let common_ = CommonFilter::new()
            // .with_date::<FilterRange, CreateDateQuery>("created", some_date_str, "")
            .with_range::<FilterRange>("document_size", 512, 1024)
            .with_term::<FilterTerm>("document_extension", ".txt")
            .with_term::<FilterTerm>("document_type", "document")
            .build();

        let query_value = serde_json::to_value(common_).unwrap();
        let query_string = serde_json::to_string(&query_value).unwrap();
        assert_eq!("{\"bool\":{\"must\":[{\"range\":{\"document_size\":{\"gte\":512,\"lte\":1024}}},{\"term\":{\"document_extension\":\".txt\"}},{\"term\":{\"document_type\":\"document\"}}]}}", query_string);
    }
}
