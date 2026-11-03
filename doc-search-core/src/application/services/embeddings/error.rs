use thiserror::Error;

pub type TokenizerResult<T> = Result<T, TokenizerError>;

#[derive(Debug, Error)]
pub enum TokenizerError {
    #[error("vllm request failed: {0}")]
    Request(anyhow::Error),
    #[error("invalid vllm response: {0}")]
    InvalidResponse(anyhow::Error),
    #[error("returned empty vectors")]
    EmptyVectors,
    #[error("vllm service returned unexpected status {status}: {body}")]
    UnexpectedStatus { status: u16, body: String },
}

impl TokenizerError {
    pub fn is_retriable(&self) -> bool {
        match self {
            TokenizerError::Request(_) => true,
            TokenizerError::UnexpectedStatus { status, .. } => {
                (500..=599).contains(status) || *status == 429
            }
            TokenizerError::InvalidResponse(_) => false,
            TokenizerError::EmptyVectors => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    #[test]
    fn test_error_display_and_retriability() {
        let request = TokenizerError::Request(anyhow!("connection refused"));
        assert!(request.is_retriable());
        assert!(request.to_string().contains("request failed"));

        let timeout = TokenizerError::UnexpectedStatus {
            status: 504,
            body: "gateway".to_string(),
        };
        assert!(timeout.is_retriable());

        let client_error = TokenizerError::UnexpectedStatus {
            status: 400,
            body: "bad".to_string(),
        };
        assert!(!client_error.is_retriable());

        let too_many = TokenizerError::UnexpectedStatus {
            status: 429,
            body: "slow".to_string(),
        };
        assert!(too_many.is_retriable());

        let invalid = TokenizerError::InvalidResponse(anyhow!("bad json"));
        assert!(!invalid.is_retriable());
    }
}
