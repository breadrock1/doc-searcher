use std::fmt::Display;

/// A large document identifier.
///
/// Represents the unique identifier of a complete document that may be split
/// into multiple parts for processing and storage.
#[derive(Clone, Debug)]
pub struct LargeDocumentId(pub String);

impl LargeDocumentId {
    pub fn as_string(&self) -> &str {
        &self.0
    }
}

impl Display for LargeDocumentId {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", &self.0)
    }
}

///A document part identifier.
///
/// Represents the unique identifier of an individual document part,
/// typically combining the large document ID with the part number.
#[derive(Clone, Debug)]
pub struct DocumentPartId(pub String);

impl DocumentPartId {
    pub fn as_string(&self) -> &str {
        &self.0
    }
}

/// A search index identifier.
///
/// Represents the unique name or ID of a search index where
/// document parts are stored and queried.
#[derive(Clone, Debug)]
pub struct IndexId(pub String);

impl IndexId {
    pub fn as_string(&self) -> &str {
        &self.0
    }
}
