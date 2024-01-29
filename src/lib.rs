//! Rust wrapper for [FactorDB](http://factordb.com/) API.
//!
//! # Examples
//! TODO: redo examples
//!
//! # Crate features
//! - **blocking** - Enables [`FactorDbBlockingClient`] which is a blocking alternative to [`FactorDbClient`] and does not require async runtime.

#![warn(missing_docs)]

mod utils;

use std::fmt::Display;

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
/// TODO
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

    /// Make the actual web request
    async fn fetch_response<T: Display>(&self, number: T) -> reqwest::Result<Response> {
        let url = format!("{}?query={}", ENDPOINT, number);
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

    // #[test]
    // fn get_42() {
    //     let num = Number::get_blocking(42).unwrap();
    //     println!("{}", num);
    // }

    // fn test_exponents() {
    //     let num = Number::get_blocking(36).unwrap();
    //     let mut product = BigInt::from(1);
    //     for f in num.factor_list().iter() {
    //         product *= f;
    //     }
    //     assert_eq!(num.id(), product);
    // }
}
