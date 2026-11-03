use std::fmt;

/// Error describing why a search path (file/path filter passed alongside a
/// bucket index) is invalid.
///
/// The path is used verbatim inside the OpenSearch `match` query against the
/// `file_path` field, so a relative, traversal-free path must be enforced
/// before it reaches the search layer.
#[derive(Debug, PartialEq, Eq)]
pub enum PathError {
    /// Path is empty or resolves to the root ("", ".", "./").
    Empty,
    /// Path starts with "/" (absolute path).
    Absolute,
    /// Path contains a ".." traversal segment.
    Traversal,
    /// Path ends with "/" (a folder marker, not a file path).
    TrailingSlash,
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            PathError::Empty => "path is empty or resolves to the root",
            PathError::Absolute => "path must not start with '/'",
            PathError::Traversal => "path must not contain '..'",
            PathError::TrailingSlash => "path must not end with '/'",
        };
        write!(f, "invalid search path: {msg}")
    }
}

impl std::error::Error for PathError {}

/// Validates a search path (object key or folder prefix) before it is used as
/// a filter in the OpenSearch query. The rules mirror the object-key contract
/// used by the document storages:
///
/// - the path must not be empty or resolve to the root ("", ".", "./");
/// - the path must not be absolute (leading "/");
/// - the path must not contain a ".." traversal segment — it would escape the
///   user folder inside the shared bucket index;
/// - the path must not end with "/" — a folder marker is not a document path.
///
/// Relative dot segments ("./a", "a/./b") and multiple slashes are allowed.
pub fn validate_path(path: &str) -> Result<(), PathError> {
    if path.is_empty() || path == "." || path == "./" {
        return Err(PathError::Empty);
    }

    if path.starts_with('/') {
        return Err(PathError::Absolute);
    }

    if path.ends_with('/') {
        return Err(PathError::TrailingSlash);
    }

    if path.split('/').any(|segment| segment == "..") {
        return Err(PathError::Traversal);
    }

    Ok(())
}
