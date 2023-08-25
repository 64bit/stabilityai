use crate::{error::StabilityAIError, types::Engine, Client};

/// Enumerate available engines
pub struct Engines<'c> {
    client: &'c Client,
}

impl<'c> Engines<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// List all engines available to your organization/user
    pub async fn list(&self) -> Result<Vec<Engine>, StabilityAIError> {
        self.client.get("/engines/list").await
    }
}
