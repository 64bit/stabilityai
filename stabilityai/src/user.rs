use crate::{
    error::StabilityAIError,
    types::{AccountResponseBody, BalanceResponseBody},
    Client,
};

/// Manage your Stability.ai account, and view account/organization balances
pub struct User<'c> {
    client: &'c Client,
}

impl<'c> User<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Get information about the account associated with the provided API key
    pub async fn account(&self) -> Result<AccountResponseBody, StabilityAIError> {
        self.client.get("/user/account").await
    }

    /// The balance of the account/organization associated with the API key
    pub async fn balance(&self) -> Result<BalanceResponseBody, StabilityAIError> {
        self.client.get("/user/balance").await
    }
}
