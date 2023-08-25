//! Types used in API requests and responses.
//! These types are created from component schemas in the [OpenAPI spec](https://platform.stability.ai/docs/api-reference)
mod impls;
mod spec_types;
use derive_builder::UninitializedFieldError;
pub use spec_types::*;

use crate::error::StabilityAIError;

impl From<UninitializedFieldError> for StabilityAIError {
    fn from(value: UninitializedFieldError) -> Self {
        StabilityAIError::InvalidArgument(value.to_string())
    }
}
