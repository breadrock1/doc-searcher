use crate::domain::searcher::path::{PathError, validate_path};

#[test]
fn accepts_plain_relative_path() {
    assert_eq!(Ok(()), validate_path("user-123/doc.txt"));
}

#[test]
fn accepts_folder_prefix() {
    assert_eq!(Ok(()), validate_path("user-123"));
}

#[test]
fn accepts_nested_prefix() {
    assert_eq!(Ok(()), validate_path("user-123/inbox/2024"));
}

#[test]
fn accepts_leading_dot_segment() {
    assert_eq!(Ok(()), validate_path("./test-document.docx"));
}

#[test]
fn accepts_inline_dot_segment() {
    assert_eq!(Ok(()), validate_path("user-123/./doc.txt"));
}

#[test]
fn accepts_multiple_slashes() {
    assert_eq!(Ok(()), validate_path("user-123//doc.txt"));
}

#[test]
fn accepts_path_with_spaces() {
    assert_eq!(Ok(()), validate_path("user 123/my file.docx"));
}

#[test]
fn rejects_empty_path() {
    assert_eq!(Err(PathError::Empty), validate_path(""));
}

#[test]
fn rejects_root_as_single_dot() {
    assert_eq!(Err(PathError::Empty), validate_path("."));
}

#[test]
fn rejects_root_as_dot_slash() {
    assert_eq!(Err(PathError::Empty), validate_path("./"));
}

#[test]
fn rejects_leading_slash() {
    assert_eq!(Err(PathError::Absolute), validate_path("/etc/passwd"));
}

#[test]
fn rejects_leading_traversal() {
    assert_eq!(Err(PathError::Traversal), validate_path("../secret.txt"));
}

#[test]
fn rejects_traversal_escaping_prefix() {
    assert_eq!(
        Err(PathError::Traversal),
        validate_path("user-123/../../other/secret.txt")
    );
}

#[test]
fn rejects_traversal_in_middle() {
    assert_eq!(
        Err(PathError::Traversal),
        validate_path("user-123/../other/file.txt")
    );
}

#[test]
fn rejects_trailing_slash() {
    assert_eq!(Err(PathError::TrailingSlash), validate_path("user-123/"));
}
