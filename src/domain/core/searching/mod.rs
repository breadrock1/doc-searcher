mod error;
pub use error::{SearchError, SearchResult};

mod model;
pub use model::{Paginated, PaginatedBuilder, PaginatedBuilderError};
pub use model::{FoundedDocument, FoundedDocumentBuilder, FoundedDocumentBuilderError};

mod params;
pub use params::{CreateIndexParams, CreateIndexParamsBuilder, CreateIndexParamsBuilderError};
pub use params::{KnnIndexParams, KnnIndexParamsBuilder, KnnIndexParamsBuilderError};
pub use params::{FilterParams, FilterParamsBuilder, FilterParamsBuilderError};
pub use params::{ResultParams, ResultParamsBuilder, ResultParamsBuilderError};
pub use params::{RetrieveParams, RetrieveParamsBuilder, RetrieveParamsBuilderError};
pub use params::{FullTextSearchParams, FullTextSearchParamsBuilder, FullTextSearchParamsBuilderError};
pub use params::{HybridSearchParams, HybridSearchParamsBuilder, HybridSearchParamsBuilderError};
pub use params::{SemanticSearchParams, SemanticSearchParamsBuilder, SemanticSearchParamsBuilderError};
pub use params::{PaginateParams, PaginateParamsBuilder, PaginateParamsBuilderError};

mod service;
pub use service::{ISearcher, IPaginate};
