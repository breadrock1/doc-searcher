#[cfg(feature = "docx")]
use crate::parser::docx_file_parser;
#[cfg(feature = "excel")]
use crate::parser::excel_file_parser;
#[cfg(feature = "pdf")]
use crate::parser::pdf_file_parser;
use crate::parser::text_file_parser;

use std::path::Path;

pub enum FileKind {
    Audio,
    Docx,
    Excel,
    Pdf,
    Picture,
    Text,
    Video,
}

impl FileKind {
    pub fn what_kind(extension: &str) -> FileKind {
        match extension {
            "pdf" => FileKind::Pdf,
            "txt" | "text" => FileKind::Text,
            "docx" | "doc" => FileKind::Docx,
            "xlsx" | "xlsm" | "xltx" | "xls" => FileKind::Excel,
            "jpg" | "jpeg" | "webp" => FileKind::Picture,
            "wav" | "mp3" => FileKind::Audio,
            "mpeg" => FileKind::Video,
            _ => FileKind::Text,
        }
    }
    
    pub fn document_type(kind: &FileKind) -> String {
        match kind {
            FileKind::Audio => "audio",
            FileKind::Docx => "document",
            FileKind::Excel => "document",
            FileKind::Pdf => "document",
            FileKind::Picture => "image",
            FileKind::Text => "document",
            FileKind::Video => "video",
        }.to_string()
    }

    pub fn parse_file(file_path: &Path, kind: &FileKind) -> String {
        match kind {
            #[cfg(feature = "docx")]
            FileKind::Docx => docx_file_parser::parse(file_path),
            #[cfg(feature = "excel")]
            FileKind::Excel => excel_file_parser::parse(file_path),
            FileKind::Text => text_file_parser::parse(file_path),
            #[cfg(feature = "pdf")]
            FileKind::Pdf => pdf_file_parser::parse(file_path),
            #[cfg(feature = "video")]
            FileKind::Video => Ok("There is video file.".to_string()),
            #[cfg(feature = "audio")]
            FileKind::Audio => Ok("There is audio file.".to_string()),
            #[cfg(feature = "image")]
            FileKind::Picture => Ok("There is picture file.".to_string()),
            _ => text_file_parser::parse(file_path),
        }
        .unwrap()
    }
}
