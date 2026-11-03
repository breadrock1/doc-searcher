#[derive(Clone, Debug, Default, PartialEq)]
pub struct Reranked {
    pub doc_part_id: usize,
    pub score: f64,
}
