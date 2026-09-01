use rstest::rstest;

use crate::domain::searcher::models::{DocumentPartEntrails, Embeddings};
use crate::domain::searcher::tests::fixture::document::{
    build_document_part_entrails, build_founded_document,
};
use crate::domain::searcher::tests::fixture::{DOC_ID, DOC_PART_ID, INDEX};

const LARGE_DOC_ID: &str = "29346839246dsf987a1173sfa7sd781h";

#[rstest]
fn test_embeddings_from_vec(
    #[values(
        vec![0.1_f64, 0.2324, 0.3],
        vec![0.0_f64, 0.1, -0.3324],
        vec![-0.1_f64, 0.345435, 0.0],
    )]
    tokens: Vec<f64>,
) {
    let embeddings: Embeddings = tokens.clone().into();
    assert_eq!(tokens, embeddings.knn);
}

#[rstest]
fn test_founded_document_debug(
    #[from(build_document_part_entrails)] document_part: DocumentPartEntrails,
) {
    let found = build_founded_document(document_part);
    println!("{:#?}", found);

    assert_eq!(found.id.as_str(), DOC_ID);
    assert_eq!(found.index.as_str(), INDEX);
    assert_eq!(found.document.doc_part_id, DOC_PART_ID);
    assert_eq!(found.document.large_doc_id.as_string(), LARGE_DOC_ID);
}
