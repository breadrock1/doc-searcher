mod ids;
pub use ids::DocumentPartId;
pub use ids::IndexId;
pub use ids::LargeDocumentId;

pub mod metadata;

#[cfg(test)]
pub(crate) mod tests;
