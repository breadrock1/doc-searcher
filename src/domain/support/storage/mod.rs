mod error;
pub use error::{StorageError, StorageResult};

mod model;
pub use model::{Index, IndexBuilder, IndexBuilderError};
pub use model::{DocumentPart, DocumentPartBuilder, DocumentPartBuilderError};
pub use model::Embeddings;

mod params;

mod service;
pub use service::{IDocumentStorage, IIndexStorage};
