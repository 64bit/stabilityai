//! Errors originating from API calls, parsing responses, and reading-or-writing to the file system.
use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum StabilityAIError {
    /// Underlying error from reqwest library after an API call was made
    #[error("http error: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// OpenAI returns error object with details of API call failure
    #[error("id: {}, name: {}, message: {}", .0.id, .0.name, .0.message)]
    ApiError(ApiError),
    /// Error when a response cannot be deserialized into a Rust type
    #[error("failed to deserialize api response: {0}")]
    JSONDeserialize(serde_json::Error),
    /// Error on the client side when saving file to file system
    #[error("failed to save file: {0}")]
    FileSaveError(String),
    /// Error on the client side when reading file from file system
    #[error("failed to read file: {0}")]
    FileReadError(String),
    /// Error from client side validation
    /// or when builder fails to build request before making API call
    #[error("invalid args: {0}")]
    InvalidArgument(String),
}

/// OpenAI API returns error object on failure
#[derive(Debug, Deserialize)]
pub struct ApiError {
    /// A unique identifier for this particular occurrence of the problem.
    pub id: String,
    /// The short-name of this class of errors e.g. `bad_request`.
    pub name: String,
    /// A human-readable explanation specific to this occurrence of the problem.
    pub message: String,
}

pub(crate) fn map_deserialization_error(e: serde_json::Error, bytes: &[u8]) -> StabilityAIError {
    tracing::error!(
        "failed deserialization of: {}",
        String::from_utf8_lossy(bytes)
    );
    StabilityAIError::JSONDeserialize(e)
}
