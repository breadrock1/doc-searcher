use derive_builder::Builder;
use getset::{CopyGetters, Getters};
use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[allow(unused_imports)]
use serde_json::json;

#[derive(Builder, Clone, Getters, CopyGetters, Serialize, Deserialize, ToSchema)]
pub struct Document {
    #[schema(example = "98ac9896be35f47fb8442580cd9839b4")]
    #[getset(get = "pub")]
    id: String,
    #[schema(example = "test-document.docx")]
    #[getset(get = "pub")]
    file_name: String,
    #[schema(example = "./test-document.docx")]
    #[getset(get = "pub")]
    file_path: String,
    #[schema(example = 1024)]
    #[getset(get_copy = "pub")]
    file_size: u32,
    #[schema(example = "There is some content data")]
    #[getset(get = "pub")]
    content: String,
    #[schema(example = 1750957115)]
    #[getset(get = "pub")]
    created_at: i64,
    #[schema(example = 1750957115)]
    #[getset(get = "pub")]
    modified_at: i64,
    #[getset(get = "pub")]
    embeddings: Vec<EmbeddingChunk>,
}

#[derive(Builder, Clone, CopyGetters, Getters, Serialize, Deserialize, ToSchema)]
pub struct EmbeddingChunk {
    #[schema(example = 0)]
    #[getset(get_copy = "pub")]
    chunk_id: u32,
    #[schema(example = "There is some")]
    #[getset(get = "pub")]
    chunk_text: String,
    #[schema(example = json!(vec![0.0345456, -0.4353242]))]
    #[getset(get = "pub")]
    tokens: Vec<f64>,
}

impl TryFrom<crate::domain::structures::Document> for Document {
    type Error = DocumentBuilderError;

    fn try_from(value: crate::domain::structures::Document) -> Result<Self, Self::Error> {
        let embeddings = value
            .embeddings()
            .into_iter()
            .filter_map(|it| match EmbeddingChunk::try_from(it) {
                Ok(maped) => Some(maped),
                Err(err) => {
                    tracing::warn!(
                        chunk=it.chunk_id(),
                        err=?err,
                        "failed to map embeddings to domain object",
                    );
                    None
                }
            })
            .collect::<Vec<EmbeddingChunk>>();

        let document = DocumentBuilder::default()
            .id(value.id().to_owned())
            .file_name(value.file_name().to_owned())
            .file_path(value.file_path().to_owned())
            .file_size(value.file_size())
            .content(value.content().to_owned())
            .created_at(value.created_at().to_owned())
            .modified_at(value.modified_at().to_owned())
            .embeddings(embeddings)
            .build()?;

        Ok(document)
    }
}

impl TryFrom<&crate::domain::structures::EmbeddingChunk> for EmbeddingChunk {
    type Error = EmbeddingChunkBuilderError;

    fn try_from(value: &crate::domain::structures::EmbeddingChunk) -> Result<Self, Self::Error> {
        let chunk = EmbeddingChunkBuilder::default()
            .chunk_id(value.chunk_id())
            .chunk_text(value.chunk_text().to_owned())
            .tokens(value.tokens().to_owned())
            .build()?;
        Ok(chunk)
    }
}
