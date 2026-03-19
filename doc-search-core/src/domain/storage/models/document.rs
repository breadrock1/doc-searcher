use anyhow::{Context, anyhow};
use character_text_splitter::CharacterTextSplitter;
use derive_builder::Builder;
use std::fmt::{Debug, Formatter};

use crate::domain::storage::{StorageError, StorageResult};
use crate::shared::kernel::metadata::DocumentMetadata;
use crate::shared::kernel::{DocumentPartId, LargeDocumentId};

/// The ID of the first document part in a sequence.
///
/// Document parts are numbered starting from 1 to maintain
/// consistent ordering when splitting large documents.
pub const FIRST_DOCUMENT_PART_ID: usize = 1;

/// Type alias for a collection of all document parts belonging to a large document.
///
/// Represents the complete set of parts that make up a single large document
/// after splitting.
pub type AllDocumentParts = Vec<DocumentPart>;

/// Represents a complete large document before splitting into parts.
///
/// This structure contains the full content and metadata of a document
/// that will be processed and split into smaller chunks for efficient
/// storage and search operations.
///
/// # Fields
/// * `file_name` - Original name of the file
/// * `file_path` - Storage path where the file is located
/// * `file_size` - Size of the file in bytes
/// * `created_at` - Unix timestamp of file creation
/// * `modified_at` - Unix timestamp of last modification
/// * `content` - Complete text content of the document
/// * `metadata` - Additional document metadata (optional)
///
/// # Example
/// ```
/// let large_doc = LargeDocument {
///     file_name: "the_great_gatsby.txt".to_string(),
///     file_path: "/uploads/novels/the_great_gatsby.txt".to_string(),
///     file_size: 512000,
///     created_at: 1634567890,
///     modified_at: 1634567890,
///     content: "Chapter 1 ...".to_string(),
///     metadata: Some(DocumentMetadata {
///         author: "F. Scott Fitzgerald".to_string(),
///         language: "en".to_string(),
///         // ... other metadata fields
///     }),
/// };
/// ```
#[derive(Builder)]
pub struct LargeDocument {
    pub file_name: String,
    pub file_path: String,
    pub file_size: u32,
    pub created_at: i64,
    pub modified_at: i64,
    pub content: String,
    pub metadata: Option<DocumentMetadata>,
}

impl Debug for LargeDocument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "file_path: {}, create_at: {}",
            &self.file_path, &self.created_at
        )
    }
}

/// Represents a single part of a split large document.
///
/// This structure contains a chunk of text from a larger document,
/// along with its position in the sequence and reference to the parent document.
/// Document parts are the fundamental units stored in the search index.
///
/// Common LargeDocument must be divided on document parts (> 1 parts)!!!
/// Any way if there is document with empty content data will be returned error.
///
/// # Fields
/// * `large_doc_id` - Reference to the parent large document
/// * `doc_part_id` - Sequential position of this part (starting from 1)
/// * `file_name` - Original file name (inherited from parent)
/// * `file_path` - Original file path (inherited from parent)
/// * `file_size` - Size of this part in bytes
/// * `created_at` - Creation timestamp (inherited from parent)
/// * `modified_at` - Modification timestamp (inherited from parent)
/// * `content` - Text content of this specific part
/// * `metadata` - Document metadata (inherited and possibly extended)
///
/// # Example
/// ```
/// let doc_part = DocumentPart {
///     large_doc_id: "doc_123".to_string(),
///     doc_part_id: 3,
///     file_name: "the_great_gatsby.txt".to_string(),
///     file_path: "/uploads/novels/the_great_gatsby.txt".to_string(),
///     file_size: 5120,
///     created_at: 1634567890,
///     modified_at: 1634567890,
///     content: "Chapter 3, Part 1...".to_string(),
///     metadata: None,
/// };
/// ```
#[derive(Clone, Builder)]
pub struct DocumentPart {
    pub large_doc_id: LargeDocumentId,
    pub doc_part_id: usize,
    pub file_name: String,
    pub file_path: String,
    pub file_size: u32,
    pub created_at: i64,
    pub modified_at: i64,
    pub content: String,
    pub metadata: Option<DocumentMetadata>,
}

impl Debug for DocumentPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "large_doc_id: {:?}, doc_part_id: {}",
            &self.large_doc_id, &self.doc_part_id
        )
    }
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
            .large_doc_id(LargeDocumentId(large_doc_id))
            .doc_part_id(FIRST_DOCUMENT_PART_ID)
            .file_name(self.file_name)
            .file_path(self.file_path)
            .file_size(self.file_size)
            .created_at(self.created_at)
            .modified_at(self.modified_at)
            .content(String::default())
            .metadata(self.metadata)
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

/// Information about stored document parts for a large document.
///
/// This structure provides metadata about how a large document was split
/// and stored, useful for retrieval and management operations.
///
/// # Fields
/// * `large_doc_id` - Identifier of the parent large document
/// * `first_part_id` - ID of the first document part (for pagination/retrieval)
/// * `doc_parts_amount` - Total number of parts this document was split into
///
/// # Example
/// ```
/// let stored_info = StoredDocumentPartsInfo {
///     large_doc_id: "doc_123".to_string(),
///     first_part_id: "doc_123_part_1".to_string(),
///     doc_parts_amount: 15,
/// };
/// ```
#[derive(Debug, Builder)]
pub struct StoredDocumentPartsInfo {
    pub large_doc_id: LargeDocumentId,
    pub first_part_id: DocumentPartId,
    pub doc_parts_amount: usize,
}
