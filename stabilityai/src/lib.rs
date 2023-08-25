//! Rust library for stability.ai based on OpenAPI spec.
//!
//! ## Creating client
//!
//! ```
//! use stabilityai::Client;
//!
//! // Create a client with api key from env var STABILITY_API_KEY and default base url.
//! let client = Client::new();
//!
//! // OR use API key from different source and a non default organization
//! let api_key = "sk-..."; // This secret could be from a file, or environment variable.
//! let client = Client::new()
//!     .with_api_key(api_key)
//!     .with_organization("the-continental");
//!
//! // Use custom reqwest client
//! let http_client = reqwest::ClientBuilder::new()
//!     .user_agent("Rust/stabilityai")
//!     .build().unwrap();
//!
//! let client = Client::new()
//!     .with_http_client(http_client);
//! ```
//!
//! ## Making requests
//!
//!```
//!# tokio_test::block_on(async {
//! use stabilityai::Client;
//!
//! // Create client
//! let client = Client::new();
//!
//! // Call API
//! let response = client
//!     .user()
//!     .account()
//!     .await
//!     .unwrap();
//!
//! println!("{:#?}", response);
//! # });
//!```
//!
//! ## Examples
//! For full working examples see [examples](https://github.com/64bit/stabilityai/tree/main/examples) directory in the repository.
//!

mod client;
mod download;
mod engine;
pub mod error;
mod generate;
pub mod types;
mod user;
mod util;

pub use client::Client;
pub use engine::Engines;
pub use generate::Generate;
pub use user::User;

pub use client::API_BASE;
pub use client::CLIENT_ID_HEADER;
pub use client::CLIENT_VERSION_HEADER;
pub use client::ORGANIZATION_HEADER;
