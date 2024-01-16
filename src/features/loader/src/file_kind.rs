use crate::parser::docx_file_parser;
use crate::parser::excel_file_parser;
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
            ".pdf" => FileKind::Pdf,
            ".txt" | ".text" => FileKind::Text,
            ".docx" | ".doc" => FileKind::Docx,
            ".xlsx" | ".xlsm" | ".xltx" => FileKind::Excel,
            ".jpg" | ".jpeg" | ".webp" => FileKind::Picture,
            ".wav" | ".mp3" => FileKind::Audio,
            ".mpeg" => FileKind::Video,
            _ => FileKind::Text,
        }
    }

    pub fn parse_file(file_path: &Path, kind: &FileKind) -> String {
        match kind {
            FileKind::Docx => docx_file_parser::parse(file_path),
            FileKind::Excel => excel_file_parser::parse(file_path),
            FileKind::Text => text_file_parser::parse(file_path),
            FileKind::Pdf => pdf_file_parser::parse(file_path),
            FileKind::Video => Ok("There is video file.".to_string()),
            FileKind::Audio => Ok("There is audio file.".to_string()),
            FileKind::Picture => Ok("There is picture file.".to_string()),
        }
        .unwrap()
    }
}
