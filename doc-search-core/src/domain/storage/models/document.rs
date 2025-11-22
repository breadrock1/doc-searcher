use anyhow::{Context, anyhow};
use character_text_splitter::CharacterTextSplitter;
use derive_builder::Builder;

use crate::domain::storage::{StorageError, StorageResult};

const FIRST_DOCUMENT_PART_ID: usize = 1;

pub type LargeDocumentId = String;
pub type DocumentPartId = String;

pub type AllDocumentParts = Vec<DocumentPart>;

#[derive(Debug, Builder)]
pub struct LargeDocument {
    pub file_name: String,
    pub file_path: String,
    pub file_size: u32,
    pub created_at: i64,
    pub modified_at: i64,
    pub content: String,
}

#[derive(Clone, Debug, Builder)]
pub struct DocumentPart {
    pub large_doc_id: String,
    pub doc_part_id: usize,
    pub file_name: String,
    pub file_path: String,
    pub file_size: u32,
    pub created_at: i64,
    pub modified_at: i64,
    pub content: String,
}

impl LargeDocument {
    pub fn divide_large_document_on_parts(
        self,
        part_size: usize,
    ) -> StorageResult<Vec<DocumentPart>> {
        if self.content.is_empty() {
            let err = anyhow!("document content is empty.");
            return Err(StorageError::CantSplitLargeDocuments(err));
        }

        let large_doc_id = uuid::Uuid::new_v4().to_string();
        let document_part = DocumentPartBuilder::default()
            .large_doc_id(large_doc_id)
            .doc_part_id(FIRST_DOCUMENT_PART_ID)
            .file_name(self.file_name)
            .file_path(self.file_path)
            .file_size(self.file_size)
            .created_at(self.created_at)
            .modified_at(self.modified_at)
            .content(String::default())
            .build()
            .context("failed to build document part")
            .map_err(StorageError::CantSplitLargeDocuments)?;

        let document_parts = CharacterTextSplitter::new()
            .with_chunk_size(part_size)
            .split_text(&self.content)
            .into_iter()
            .enumerate()
            .map(|(part_id, part_content)| {
                let mut doc_part_cln = document_part.clone();
                doc_part_cln.doc_part_id = part_id + 1;
                doc_part_cln.content = part_content;
                doc_part_cln
            })
            .collect::<Vec<DocumentPart>>();

        Ok(document_parts)
    }
}

#[derive(Debug, Builder)]
pub struct StoredDocumentPartsInfo {
    pub large_doc_id: String,
    pub first_part_id: String,
    pub doc_parts_amount: usize,
}
