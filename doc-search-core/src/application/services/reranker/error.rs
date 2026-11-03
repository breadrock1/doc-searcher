use thiserror::Error;

pub type RerankerResult<T> = Result<T, RerankerError>;

#[derive(Debug, Error)]
pub enum RerankerError {
    #[error("reranker request failed: {0}")]
    Request(anyhow::Error),
    #[error("invalid reranker response: {0}")]
    InvalidResponse(anyhow::Error),
    #[error("returned empty vectors")]
    EmptyVectors,
    #[error("reranker service returned unexpected status {status}: {body}")]
    UnexpectedStatus { status: u16, body: String },
}

impl RerankerError {
    pub fn is_retriable(&self) -> bool {
        match self {
            RerankerError::Request(_) => true,
            RerankerError::UnexpectedStatus { status, .. } => {
                (500..=599).contains(status) || *status == 429
            }
            RerankerError::InvalidResponse(_) => false,
            RerankerError::EmptyVectors => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    #[test]
    fn test_error_display_and_retriability() {
        let request = RerankerError::Request(anyhow!("connection refused"));
        assert!(request.is_retriable());
        assert!(request.to_string().contains("request failed"));

        let timeout = RerankerError::UnexpectedStatus {
            status: 504,
            body: "gateway".to_string(),
        };
        assert!(timeout.is_retriable());

        let client_error = RerankerError::UnexpectedStatus {
            status: 400,
            body: "bad".to_string(),
        };
        assert!(!client_error.is_retriable());

        let too_many = RerankerError::UnexpectedStatus {
            status: 429,
            body: "slow".to_string(),
        };
        assert!(too_many.is_retriable());

        let invalid = RerankerError::InvalidResponse(anyhow!("bad json"));
        assert!(!invalid.is_retriable());
    }
}
