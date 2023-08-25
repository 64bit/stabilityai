use reqwest::header::HeaderMap;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    error::{map_deserialization_error, ApiError, StabilityAIError},
    generate::Generate,
    user::User,
    Engines,
};

#[derive(Debug, Clone)]
/// Client is a container of configurations to make API calls.
pub struct Client {
    http_client: reqwest::Client,
    api_key: String,
    api_base: String,
    organization: String,
    client_id: Option<String>,
    client_version: Option<String>,
    backoff: backoff::ExponentialBackoff,
}

/// Default v1 API base url
pub const API_BASE: &str = "https://api.stability.ai/v1";
/// Name for organization header
pub const ORGANIZATION_HEADER: &str = "Organization";
/// Name for client id header
pub const CLIENT_ID_HEADER: &str = "Stability-Client-ID";
/// Name for client version header
pub const CLIENT_VERSION_HEADER: &str = "Stability-Client-Version";

impl Default for Client {
    /// Create client with default [API_BASE] url and default API key from STABILITY_API_KEY env var
    fn default() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            api_base: API_BASE.to_string(),
            api_key: std::env::var("STABILITY_API_KEY").unwrap_or_else(|_| "".to_string()),
            organization: Default::default(),
            backoff: Default::default(),
            client_id: None,
            client_version: None,
        }
    }
}

impl Client {
    /// Create client with default [API_BASE] url and default API key from STABILITY_API_KEY env var
    pub fn new() -> Self {
        Default::default()
    }

    /// Provide your own [client] to make HTTP requests with.
    ///
    /// [client]: reqwest::Client
    pub fn with_http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = http_client;
        self
    }

    /// To use a different API key different from default STABILITY_API_KEY env var
    pub fn with_api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = api_key.into();
        self
    }

    /// To use a different organization id other than default
    pub fn with_organization<S: Into<String>>(mut self, organization: S) -> Self {
        self.organization = organization.into();
        self
    }

    /// To use a API base url different from default [API_BASE]
    pub fn with_api_base<S: Into<String>>(mut self, api_base: S) -> Self {
        self.api_base = api_base.into();
        self
    }

    /// Exponential backoff for retrying rate limited requests.
    pub fn with_backoff(mut self, backoff: backoff::ExponentialBackoff) -> Self {
        self.backoff = backoff;
        self
    }

    pub fn api_base(&self) -> &str {
        &self.api_base
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if !self.organization.is_empty() {
            headers.insert(
                ORGANIZATION_HEADER,
                self.organization.as_str().parse().unwrap(),
            );
        }

        if let Some(ref client_id) = self.client_id {
            if !client_id.is_empty() {
                headers.insert(CLIENT_ID_HEADER, client_id.as_str().parse().unwrap());
            }
        }

        if let Some(ref client_version) = self.client_version {
            if !client_version.is_empty() {
                headers.insert(
                    CLIENT_VERSION_HEADER,
                    client_version.as_str().parse().unwrap(),
                );
            }
        }

        headers
    }

    // API groups

    /// To call [User] group related APIs using this client.
    pub fn user(&self) -> User {
        User::new(self)
    }

    /// To call [Engines] group related APIs using this client.
    pub fn engines(&self) -> Engines {
        Engines::new(self)
    }

    /// To call [Generate] group related APIs using this client.
    pub fn generate<S: Into<String>>(&self, engine_id: S) -> Generate<S>
    where
        S: std::fmt::Display,
    {
        Generate::new(self, engine_id)
    }

    /// Make a GET request to {path} and deserialize the response body
    pub(crate) async fn get<O>(&self, path: &str) -> Result<O, StabilityAIError>
    where
        O: DeserializeOwned,
    {
        let request_maker = || async {
            Ok(self
                .http_client
                .get(format!("{}{path}", self.api_base()))
                .bearer_auth(self.api_key())
                .headers(self.headers())
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// Make a POST request to {path} and deserialize the response body
    pub(crate) async fn post<I, O>(&self, path: &str, request: I) -> Result<O, StabilityAIError>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let request_maker = || async {
            Ok(self
                .http_client
                .post(format!("{}{path}", self.api_base()))
                .bearer_auth(self.api_key())
                .headers(self.headers())
                .json(&request)
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// POST a form at {path} and deserialize the response body
    pub(crate) async fn post_form<O, F>(&self, path: &str, form: F) -> Result<O, StabilityAIError>
    where
        O: DeserializeOwned,
        reqwest::multipart::Form: async_convert::TryFrom<F, Error = StabilityAIError>,
        F: Clone,
    {
        let request_maker = || async {
            Ok(self
                .http_client
                .post(format!("{}{path}", self.api_base()))
                .bearer_auth(self.api_key())
                .headers(self.headers())
                .multipart(async_convert::TryInto::try_into(form.clone()).await?)
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// Execute a HTTP request and retry on rate limit
    ///
    /// request_maker serves one purpose: to be able to create request again
    /// to retry API call after getting rate limited. request_maker is async because
    /// reqwest::multipart::Form is created by async calls to read files for uploads.
    async fn execute<O, M, Fut>(&self, request_maker: M) -> Result<O, StabilityAIError>
    where
        O: DeserializeOwned,
        M: Fn() -> Fut,
        Fut: core::future::Future<Output = Result<reqwest::Request, StabilityAIError>>,
    {
        let client = self.http_client.clone();

        backoff::future::retry(self.backoff.clone(), || async {
            let request = request_maker().await.map_err(backoff::Error::Permanent)?;
            let response = client
                .execute(request)
                .await
                .map_err(StabilityAIError::Reqwest)
                .map_err(backoff::Error::Permanent)?;

            let status = response.status();
            let bytes = response
                .bytes()
                .await
                .map_err(StabilityAIError::Reqwest)
                .map_err(backoff::Error::Permanent)?;

            // Deserialize response body from either error object or actual response object
            if !status.is_success() {
                let api_error: ApiError = serde_json::from_slice(bytes.as_ref())
                    .map_err(|e| map_deserialization_error(e, bytes.as_ref()))
                    .map_err(backoff::Error::Permanent)?;

                if status.as_u16() == 429 {
                    // Rate limited retry...
                    tracing::warn!("Rate limited: {}", api_error.message);
                    return Err(backoff::Error::Transient {
                        err: StabilityAIError::ApiError(api_error),
                        retry_after: None,
                    });
                } else {
                    return Err(backoff::Error::Permanent(StabilityAIError::ApiError(
                        api_error,
                    )));
                }
            }

            let response: O = serde_json::from_slice(bytes.as_ref())
                .map_err(|e| map_deserialization_error(e, bytes.as_ref()))
                .map_err(backoff::Error::Permanent)?;
            Ok(response)
        })
        .await
    }
}
