//! Rust wrapper for [FactorDB](http://factordb.com/) API.
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```
//! use std::error::Error;
//! use factordb::FactorDbClient;
//! use num_bigint::BigInt; // All numeric values in the result object are of this type
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     // Initialise the client
//!     let client = FactorDbClient::new();
//!
//!     // Make requests
//!     let forty_two = client.get(42).await?;
//!     let expect_factors: Vec<BigInt> = vec![2, 3, 7].into_iter().map(|n| BigInt::from(n)).collect();
//!     assert_eq!(forty_two.into_factors_flattened(), expect_factors);
//!
//!     Ok(())
//!  }
//! ```
//!
//! # Crate features
//! - **blocking** - Enables [`FactorDbBlockingClient`] which is a blocking alternative to [`FactorDbClient`] and does not require async runtime.

#![warn(missing_docs)]

mod utils;

use std::fmt::Display;

use log::debug;
use reqwest::{Client, Response};

pub mod factor;
pub mod number;

pub use factor::Factor;
pub use number::Number;
pub use number::NumberStatus;

const ENDPOINT: &str = "http://factordb.com/api";

/// Asynchronous API client for factorDB API.
///
/// If you need a blocking client, use [`FactorDbBlockingClient`] instead.
///
/// If you're making multiple requests, it's probably a good idea to reuse the client to take advantage of keep-alive
/// connection pooling. ([Learn more](https://docs.rs/reqwest/latest/reqwest/index.html#making-a-get-request))
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// use factordb::FactorDbClient;
/// use num_bigint::BigInt; // All numeric values in the result object are of this type
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn Error>> {
/// // Initialise the client
/// let client = FactorDbClient::new();
///
/// // Make requests
/// let forty_two = client.get(42).await?;
/// let expect_factors: Vec<BigInt> = vec![2, 3, 7].into_iter().map(|n| BigInt::from(n)).collect();
/// assert_eq!(forty_two.into_factors_flattened(), expect_factors);
/// #
/// #   Ok(())
/// # }
#[derive(Debug, Clone)]
pub struct FactorDbClient {
    client: Client,
}

impl FactorDbClient {
    /// Creates a new instance of [`FactorDbClient`] with a default HTTP client.
    pub fn new() -> Self {
        Self::with_client(Client::new())
    }

    /// Creates a new instance of [`FactorDbClient`] with a supplied [`reqwest::Client`].
    pub fn with_client(client: Client) -> Self {
        debug!("Creating async HTTP client");
        Self { client }
    }

    /// Sends a GET request to the FactorDB API for a given number. Returns an instance of [`Factor`].
    ///
    /// # Errors
    /// Returns a [`FactorDbError`] if either the API request responded with an error or there is an error in the
    /// request or parsing of the response.
    pub async fn get<T: Display>(&self, number: T) -> Result<Number, FactorDbError> {
        let response = self.fetch_response(number).await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await.expect("Invalid JSON response"))
        } else {
            Err(FactorDbError::InvalidNumber)
        }
    }

    /// Sends a GET request to the FactorDB API for a given number and returns its JSON response.
    ///
    /// # Errors
    /// Returns a [`FactorDbError`] if either the API request responded with an error or there is an error in the
    /// request or parsing of the response.
    pub async fn get_json<T: Display>(&self, number: T) -> Result<String, FactorDbError> {
        let response = self.fetch_response(number).await?;
        let status = response.status();
        if status.is_success() {
            Ok(response
                .text()
                .await
                .expect("Unable to decode response body"))
        } else {
            Err(FactorDbError::InvalidNumber)
        }
    }

    /// Make the actual web request/// # #[tokio::main]
    async fn fetch_response<T: Display>(&self, number: T) -> reqwest::Result<Response> {
        let url = format!("{}?query={}", ENDPOINT, number);
        debug!("Fetching API response from {}", url);
        self.client.get(url).send().await
    }
}

impl Default for FactorDbClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Blocking API client for FactorDB API.
///
/// If you're making multiple requests, it's probably a good idea to reuse the client to take advantage of keep-alive
/// connection pooling. ([Learn more](https://docs.rs/reqwest/latest/reqwest/index.html#making-a-get-request))
///
/// As per [`reqwest::blocking`] restriction, this client must not be used in an async runtime. Please use
/// [`FactorDbClient`] for that.
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// use factordb::FactorDbBlockingClient;
/// use num_bigint::BigInt; // All numeric values in the result object are of this type
///
/// fn main() -> Result<(), Box<dyn Error>> {
/// // Initialise the client
/// let client = FactorDbBlockingClient::new();
///
/// // Make requests
/// let forty_two = client.get(42)?;
/// let expect_factors: Vec<BigInt> = vec![2, 3, 7].into_iter().map(|n| BigInt::from(n)).collect();
/// assert_eq!(forty_two.into_factors_flattened(), expect_factors);
/// #
/// #   Ok(())
/// # }
#[cfg(feature = "blocking")]
#[derive(Debug, Clone)]
pub struct FactorDbBlockingClient {
    client: reqwest::blocking::Client,
}

#[cfg(feature = "blocking")]
impl FactorDbBlockingClient {
    /// Creates a new instance of [`FactorDbBlockingClient`] with a default HTTP client.
    pub fn new() -> Self {
        Self::with_client(reqwest::blocking::Client::new())
    }

    /// Creates a new instance of [`FactorDbBlockingClient`] with a supplied [`reqwest::Client`].
    pub fn with_client(client: reqwest::blocking::Client) -> Self {
        debug!("Creating blocking HTTP client");
        Self { client }
    }

    /// Sends a GET request to the FactorDB API for a given number. Returns an instance of [`Factor`].
    ///
    /// # Errors
    /// Returns a [`FactorDbError`] if either the API request responded with an error or there is an error in the
    /// request or parsing of the response.
    pub fn get<T: Display>(&self, number: T) -> Result<Number, FactorDbError> {
        let response = self.fetch_response(number)?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().expect("Invalid JSON response"))
        } else {
            Err(FactorDbError::InvalidNumber)
        }
    }

    /// Sends a GET request to the FactorDB API for a given number and returns its JSON response.
    ///
    /// # Errors
    /// Returns a [`FactorDbError`] if either the API request responded with an error or there is an error in the
    /// request or parsing of the response.
    pub fn get_json<T: Display>(&self, number: T) -> Result<String, FactorDbError> {
        let response = self.fetch_response(number)?;
        let status = response.status();
        if status.is_success() {
            Ok(response.text().expect("Unable to decode response body"))
        } else {
            Err(FactorDbError::InvalidNumber)
        }
    }

    /// Make the actual web request
    fn fetch_response<T: Display>(
        &self,
        number: T,
    ) -> reqwest::Result<reqwest::blocking::Response> {
        let url = format!("{}?query={}", ENDPOINT, number);
        debug!("Fetching API response from {}", url);
        self.client.get(url).send()
    }
}

#[cfg(feature = "blocking")]
impl Default for FactorDbBlockingClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Error type in this crate.
#[derive(thiserror::Error, Debug)]
pub enum FactorDbError {
    /// Request error
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    /// Invalid number
    #[error("Invalid number")]
    InvalidNumber,
}

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;

    use super::*;

    #[tokio::test]
    async fn test_two_factors() {
        let client = FactorDbClient::new();
        let result = client.get(15).await.unwrap();
        assert_eq!(
            vec![BigInt::from(3), BigInt::from(5)],
            result.into_factors_flattened()
        )
    }

    #[tokio::test]
    async fn test_repeating_factors() {
        let client = FactorDbClient::new();
        let result = client.get(100).await.unwrap();
        assert_eq!(
            vec![
                BigInt::from(2),
                BigInt::from(2),
                BigInt::from(5),
                BigInt::from(5)
            ],
            result.clone().into_factors_flattened()
        );
        assert_eq!(
            vec![BigInt::from(2), BigInt::from(5)],
            result.into_unique_factors()
        );
    }

    #[tokio::test]
    async fn test_prime() {
        let client = FactorDbClient::new();
        let result = client.get(17).await.unwrap();
        let flatenned = result.clone().into_factors_flattened();
        let unique = result.into_unique_factors();
        assert_eq!(vec![BigInt::from(17)], flatenned);
        assert_eq!(flatenned, unique);
    }

    #[tokio::test]
    async fn test_invalid() {
        let client = FactorDbClient::new();
        let result = client.get("AAAAA").await;
        assert!(result.is_err());
    }

    // blocking tests
    #[test]
    fn test_two_factors_blocking() {
        let client = FactorDbBlockingClient::new();
        let result = client.get(15).unwrap();
        assert_eq!(
            vec![BigInt::from(3), BigInt::from(5)],
            result.into_factors_flattened()
        )
    }

    #[test]
    fn test_repeating_factors_blocking() {
        let client = FactorDbBlockingClient::new();
        let result = client.get(100).unwrap();
        assert_eq!(
            vec![
                BigInt::from(2),
                BigInt::from(2),
                BigInt::from(5),
                BigInt::from(5)
            ],
            result.clone().into_factors_flattened()
        );
        assert_eq!(
            vec![BigInt::from(2), BigInt::from(5)],
            result.into_unique_factors()
        );
    }

    #[test]
    fn test_prime_blocking() {
        let client = FactorDbBlockingClient::new();
        let result = client.get(17).unwrap();
        let flatenned = result.clone().into_factors_flattened();
        let unique = result.into_unique_factors();
        assert_eq!(vec![BigInt::from(17)], flatenned);
        assert_eq!(flatenned, unique);
    }

    #[test]
    fn test_invalid_blocking() {
        let client = FactorDbBlockingClient::new();
        let result = client.get("AAAAA");
        assert!(result.is_err());
    }
}
