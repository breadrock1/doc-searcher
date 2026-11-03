use std::ops::Deref;

/// Represents vector vllm for semantic search.
///
/// Contains the numerical representation of text for similarity calculations.
///
/// # Fields
/// * `knn` - Vector of floating-point numbers representing the text embedding
///
/// # Example
/// ```
/// use doc_search_core::shared::kernel::Tokens;
///
/// let tokens = Tokens {
///     knn: vec![0.123, 0.456, 0.789, -0.123],
/// };
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tokens {
    pub knn: Vec<f64>,
}

impl Deref for Tokens {
    type Target = Vec<f64>;

    fn deref(&self) -> &Self::Target {
        &self.knn
    }
}

impl From<Vec<f64>> for Tokens {
    fn from(tokens: Vec<f64>) -> Self {
        Tokens { knn: tokens }
    }
}
