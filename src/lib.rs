//! Rust wrapper for [FactorDB](http://factordb.com/) API.
//!
//! # Examples
//! TODO: redo examples
//!
//! # Crate features
//! - **blocking** - Enables blocking alternative to [`Number::get()`] and [`Number::with_client()`]
//! which does not require async runtime

#![warn(missing_docs)]

mod utils;

use std::collections::HashSet;
use std::fmt::{Display, Formatter};

use num_bigint::BigInt;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::utils::{deserialize_id, deserialize_string_to_bigint, deserialize_u64_to_bigint};

const ENDPOINT: &str = "http://factordb.com/api";


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

    pub async fn get<T: Display>(&self, number: T) -> Result<Number, FactorDbError> {
        let url = format!("{}?query={}", ENDPOINT, number);
        let response = self.client.get(url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await.expect("Invalid JSON response"))
        } else {
            Err(FactorDbError::InvalidNumber)
        }
    }
}

impl Default for FactorDbClient {
    fn default() -> Self {
        Self::new()
    }
}


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

    pub fn get<T: Display>(&self, number: T) -> Result<Number, FactorDbError> {
        let url = format!("{}?query={}", ENDPOINT, number);
        let response = self.client.get(url).send()?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().expect("Invalid JSON response"))
        } else {
            Err(FactorDbError::InvalidNumber)
        }
    }
}

#[cfg(feature = "blocking")]
impl Default for FactorDbBlockingClient {
    fn default() -> Self {
        Self::new()
    }
}

/// A number entry in FactorDB. Contains the number itself, its status in the database as well as its
/// factors.
#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Number {
    #[serde(deserialize_with = "deserialize_id")]
    id: BigInt,
    status: NumberStatus,
    factors: Vec<Factor>,
}

impl Number {
    /// Returns the FactorDB ID as a [`BigInt`].
    pub fn id(&self) -> &BigInt {
        &self.id
    }

    /// Returns the number's status in FactorDB.
    ///
    /// See [`NumberStatus`] for possible values.
    pub fn status(&self) -> &NumberStatus {
        &self.status
    }

    /// Returns a vector of [`Factor`].
    pub fn factors(&self) -> &Vec<Factor> {
        &self.factors
    }

    /// Returns `true` if the number may be prime.
    ///
    /// Use [`Self::is_definitely_prime()`] to check if the number have been confirmed to be prime.
    pub fn is_prime(&self) -> bool {
        self.status == NumberStatus::DefinitelyPrime || self.status == NumberStatus::ProbablyPrime
    }

    /// Returns `true` if the number is prime.
    ///
    /// This only includes that have been confirmed to be prime. Use [`Self::is_prime()`] to include
    /// numbers that may have been prime, but haven't been proven to be one.
    pub fn is_definitely_prime(&self) -> bool {
        self.status == NumberStatus::DefinitelyPrime
    }

    /// Returns a vector [`BigInt`] containing the number's factors, with its exponents expanded.
    pub fn factor_list(&self) -> Vec<&BigInt> {
        let mut out = vec![];
        for f in &self.factors {
            let mut e = BigInt::from(0);
            while &e < f.exponent() {
                out.push(f.base());
                e += 1;
            }
        }
        out
    }

    /// Returns a [`HashSet`] of unique factors of this number.
    pub fn unique_factors(&self) -> HashSet<&BigInt> {
        let mut out = HashSet::new();
        for f in &self.factors {
            out.insert(f.base());
        }
        out
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let factor_strings: Vec<String> = self.factors.iter().map(|f| f.to_string()).collect();
        write!(f, "{}", factor_strings.join(" + "))
    }
}

/// A struct representing a factor with a unique base, along with the exponent (i.e. how many times
/// the factor is repeated).
#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Factor(
    #[serde(deserialize_with = "deserialize_string_to_bigint")] BigInt,
    #[serde(deserialize_with = "deserialize_u64_to_bigint")] BigInt,
);

impl Factor {
    /// Returns the base as a [`BigInt`].
    pub fn base(&self) -> &BigInt {
        &self.0
    }

    /// Returns the exponent as a [`BigInt`].
    pub fn exponent(&self) -> &BigInt {
        &self.1
    }
}

impl Display for Factor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}^{}", self.base(), self.exponent())
    }
}

/// The status of a number in FactorDB.
#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NumberStatus {
    /// Composite, no factors known (C)
    #[serde(rename = "C")]
    NoFactorsKnown,
    /// Composite, factors known (CF)
    #[serde(rename = "CF")]
    FactorsKnown,
    /// Composite, fully factored (FF)
    #[serde(rename = "FF")]
    FullyFactored,
    /// Definitely prime (P)
    #[serde(rename = "P")]
    DefinitelyPrime,
    /// Probably prime (Prp)
    #[serde(rename = "Prp")]
    #[serde(alias = "PRP")]
    ProbablyPrime,
    /// Unknown (U)
    #[serde(rename = "U")]
    Unknown,
    /// Just for "1" (Unit)
    Unit,
    /// Just for "0"
    Zero,
    /// This number is not in database (N)
    #[serde(rename = "N")]
    NotInDatabase,
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
    use super::*;

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
