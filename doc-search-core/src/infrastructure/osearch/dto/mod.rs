mod document;
pub use document::SourceDocument;

// Переэкспорт для Task 4/5 (knn-запросы); пока не используется по пути — allow по прецеденту ниже.
#[allow(unused_imports)]
pub use document::EmbeddingsNested;

mod founded;
pub use founded::FoundedDocumentInfo;

mod index;
pub use index::IndexInformation;

mod metadata;

#[allow(unused_imports)]
pub use metadata::{
    Class, Group, Icons, Location, Pipeline, Reference, SourceDocumentMetadata, Subject,
};

mod params;

pub use params::{FullTextQueryParams, FullTextQueryParamsBuilder};
pub use params::{HybridQueryParams, HybridQueryParamsBuilder};
pub use params::{RetrieveAllDocPartsQueryParams, RetrieveAllDocPartsQueryParamsBuilder};
pub use params::{RetrieveIndexDocsQueryParams, RetrieveIndexDocsQueryParamsBuilder};
pub use params::{SemanticQueryParams, SemanticQueryParamsBuilder};
