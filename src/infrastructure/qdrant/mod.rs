mod config;
mod error;
mod dto;

pub use config::QdrantConfig;

use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use anyhow::anyhow;
use qdrant_client::{Payload, Qdrant};
use qdrant_client::qdrant::{Document as QDocument, DocumentBuilder as QDocumentBuilder, NamedVectors};
use qdrant_client::qdrant::{CreateCollection, Datatype, DeleteCollection, DeleteCollectionBuilder, DeletePointsBuilder, Distance, GetCollectionInfoRequest, HnswConfigDiff, HnswConfigDiffBuilder, PointStruct, QuantizationConfig, ScoredPoint, SearchPoints, UpsertPoints, UpsertPointsBuilder, VectorParams, Vectors, VectorsConfig};
use qdrant_client::qdrant::points_selector::PointsSelectorOneOf;
use qdrant_client::qdrant::RecommendExample::{PointId, Vector};
use qdrant_client::qdrant::vectors_config::Config;
use uuid::uuid;

use crate::application::services::storage::{DocumentManager, DocumentSearcher, IndexManager, PaginateManager, PaginateResult, StorageError, StorageResult};
use crate::application::structures::{Document, DocumentBuilder, FoundedDocument, Index, IndexBuilder, PaginatedBuilder, StoredDocument};
use crate::application::structures::params::{CreateIndexParams, FullTextSearchParams, HybridSearchParams, KnnIndexParams, PaginateParams, RetrieveDocumentParams, SemanticSearchParams};
use crate::infrastructure::qdrant::dto::SourceDocument;
use crate::ServiceConnect;

#[derive(Clone)]
pub struct QdrantClient {
    client: Arc<Qdrant>,
    options: Arc<QdrantConfig>,
}

#[async_trait::async_trait]
impl ServiceConnect for QdrantClient {
    type Client = Self;
    type Config = QdrantConfig;
    type Error = anyhow::Error;

    async fn connect(config: &Self::Config) -> Result<Self::Client, Self::Error> {
        let client = Qdrant::from_url(config.address()).build()?;

        tracing::debug!(url=config.address(), "connected to qdrant server");

        Ok(QdrantClient {
            client: Arc::new(client),
            options: Arc::new(config.to_owned()),
        })
    }
}

#[async_trait::async_trait]
impl IndexManager for QdrantClient {
    async fn create_index(&self, index: &CreateIndexParams) -> StorageResult<String> {
        let default_knn = KnnIndexParams::default();
        let knn_options = index.knn().as_ref().unwrap_or(&default_knn);

        let vector_params = VectorParams {
            size: knn_options.knn_dimension() as u64,
            distance: Distance::Cosine.into(),
            on_disk: Some(true),
            datatype: Some(Datatype::Float32.into()),
            ..Default::default()
            // hnsw_config: Some(HnswConfigDiff::default()),
            // quantization_config: Some(QuantizationConfig::default()),
            // multivector_config: ::core::option::Option<MultiVectorConfig>,
        };

        let vectors_config = VectorsConfig {
            config: Some(Config::Params(vector_params)),
        };

        let create_collection_options = CreateCollection {
            collection_name: index.id().clone(),
            vectors_config: Some(vectors_config),
            ..Default::default()
        };

        let response = self
            .client
            .create_collection(create_collection_options)
            .await?;

        if !response.result {
            let err = anyhow!("failed to create index: {response:?}");
            return Err(StorageError::InternalError(err))
        }

        Ok(index.id().clone())
    }

    async fn delete_index(&self, id: &str) -> StorageResult<()> {
        let options = DeleteCollectionBuilder::new(id.clone())
            .build();

        let response = self
            .client
            .delete_collection(options)
            .await?;

        if !response.result {
            let err = anyhow!("failed to create index: {response:?}");
            return Err(StorageError::InternalError(err))
        }

        Ok(())
    }

    async fn get_all_indexes(&self) -> StorageResult<Vec<Index>> {
        let collections = self
            .client
            .list_collections()
            .await?;

        let indexes = collections.collections
            .into_iter()
            .map(|it| IndexBuilder::default()
                .id(it.name.clone())
                .name(it.name.clone())
                .path(it.name)
                .build()
                .unwrap())
            .collect::<Vec<Index>>();

        Ok(indexes)
    }

    async fn get_index(&self, id: &str) -> StorageResult<Index> {
        let options = GetCollectionInfoRequest {
            collection_name: id.to_string(),
        };

        let response = self
            .client
            .collection_info(options)
            .await?;

        let Some(_) = response.result else {
            let err = anyhow!("failed to get collection info: {response:?}");
            return Err(StorageError::IndexNotFound(err))
        };

        let index = IndexBuilder::default()
            .id(id.to_string())
            .name(id.to_string())
            .path(id.to_string())
            .build()
            .unwrap();

        Ok(index)
    }
}

#[async_trait::async_trait]
impl DocumentManager for QdrantClient {
    async fn store_document(&self, index: &str, doc: &Document) -> StorageResult<String> {
        let id = uuid::Uuid::new_v4().to_string();

        let value = qdrant_client::qdrant::Value::from(serde_json::to_string(doc).unwrap());
        let metadata = HashMap::from([
            ("_source".to_string(), value),
        ]);


        let payload = Payload::from(metadata);

        // let embeddings = doc
        //     .embeddings()
        //     .as_ref()
        //     .unwrap()
        //     .into_iter()
        //     .enumerate()
        //     .map(|(index, it)| {
        //         (index.to_string(), it.knn.to_vec())
        //     })
        //     .collect::<HashMap<String, Vec<f64>>>();
        //
        // let vectors = NamedVectors {
        //     vectors: embeddings.into(),
        // };

        let embeddings = Vectors::default();

        let point_struct = PointStruct::new(id.clone(), embeddings, payload);
        let points = vec![point_struct];

        let upsert_points = UpsertPointsBuilder::new(self.options.collection(), points).build();
        let _ = self
            .client
            .upsert_points(upsert_points)
            .await?;

        Ok(id)
    }

    async fn store_documents(&self, index: &str, docs: &[Document]) -> StorageResult<Vec<StoredDocument>> {
        todo!()
    }

    async fn get_document(&self, index: &str, id: &str) -> StorageResult<Document> {
        todo!()
    }

    async fn delete_document(&self, index: &str, id: &str) -> StorageResult<()> {
        todo!()
    }

    async fn update_document(&self, index: &str, id: &str, doc: &Document) -> StorageResult<()> {
        todo!()
    }
}

#[async_trait::async_trait]
impl DocumentSearcher for QdrantClient {
    async fn retrieve(&self, ids: &str, params: &RetrieveDocumentParams) -> PaginateResult<FoundedDocument> {
        todo!()
    }

    async fn fulltext(&self, params: &FullTextSearchParams) -> PaginateResult<FoundedDocument> {
        let search_result = self.client
            .search_points(SearchPoints {
                collection_name: params.indexes().clone(),
                vector: vec![], // Empty vector for fulltext-only search
                filter: None,
                limit: params.result().size() as u64,
                with_payload: Some(true.into()),
                ..Default::default()
            })
            .await?;

        let founded = search_result
            .result
            .into_iter()
            .map(|it| it.payload)
            .map(|it| {
                let document = SourceDocument::from(it);
                let founded: FoundedDocument = document.into();
                founded
            })
            .collect::<Vec<FoundedDocument>>();

        let paginated = PaginatedBuilder::default()
            .founded(founded)
            .scroll_id(None)
            .build()
            .unwrap();

        Ok(paginated)
    }

    async fn hybrid(&self, params: &HybridSearchParams) -> PaginateResult<FoundedDocument> {
        // let query_embedding = self.generate_embedding(query).await?;
        let query_embedding = Vec::default();

        let search_result = self.client
            .search_points(SearchPoints {
                collection_name: params.indexes().clone(),
                vector: query_embedding,
                filter: None,
                limit: params.result().size() as u64,
                with_payload: Some(true.into()),
                ..Default::default()
            })
            .await?;

        let alpha = 0.5;
        let founded = search_result
            .result
            .into_iter()
            .map(|it| {
                let score = it.score;
                let payload = it.payload;
                let mut document = SourceDocument::from(payload);
                let score = alpha * it.score + (1.0 - alpha);
                document.set__score(Some(score as f64));
                let mut founded: FoundedDocument = document.into();
                founded
            })
            .collect::<Vec<FoundedDocument>>();

        // TODO: Impled scoring
        // founded.sort_by(|a, b| b.score().partial_cmp(a.score()).unwrap());

        let paginated = PaginatedBuilder::default()
            .founded(founded)
            .scroll_id(None)
            .build()
            .unwrap();

        Ok(paginated)
    }

    async fn semantic(&self, params: &SemanticSearchParams) -> PaginateResult<FoundedDocument> {
        // let query_embedding = self.generate_embedding(query).await?;
        let query_embedding = Vec::default();

        let search_result = self.client
            .search_points(SearchPoints {
                collection_name: params.indexes().clone(),
                vector: query_embedding,
                filter: None,
                limit: params.result().size() as u64,
                with_payload: Some(true.into()),
                ..Default::default()
            })
            .await?;

        let founded = search_result
            .result
            .into_iter()
            .map(|it| it.payload)
            .map(|it| {
                let document = SourceDocument::from(it);
                let founded: FoundedDocument = document.into();
                founded
            })
            .collect::<Vec<FoundedDocument>>();

        let paginated = PaginatedBuilder::default()
            .founded(founded)
            .scroll_id(None)
            .build()
            .unwrap();

        Ok(paginated)
    }
}

#[async_trait::async_trait]
impl PaginateManager for QdrantClient {
    async fn delete_session(&self, session_id: &str) -> StorageResult<()> {
        todo!()
    }

    async fn paginate(&self, params: &PaginateParams) -> PaginateResult<FoundedDocument> {
        todo!()
    }
}

impl QdrantClient {
    pub async fn create_index(&self) -> anyhow::Result<()> {
        let collection_name = self.options.collection();

        let vector_params = VectorParams {
            size: self.options.dimension(),
            distance: Distance::Cosine.into(),
            on_disk: Some(true),
            datatype: Some(Datatype::Float32.into()),
            ..Default::default()
            // hnsw_config: Some(HnswConfigDiff::default()),
            // quantization_config: Some(QuantizationConfig::default()),
            // multivector_config: ::core::option::Option<MultiVectorConfig>,
        };

        let vectors_config = VectorsConfig {
            config: Some(Config::Params(vector_params)),
        };

        let create_collection_options = CreateCollection {
            collection_name: collection_name.clone(),
            vectors_config: Some(vectors_config),
            ..Default::default()
        };

        let _ = self
            .client
            .create_collection(create_collection_options)
            .await?;

        Ok(())
    }

    pub async fn delete_index(&self) -> anyhow::Result<()> {
        let _ = self.client.delete_collection(self.options.collection()).await?;
        Ok(())
    }
}
