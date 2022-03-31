//! Rust wrapper for [FactorDB](http://factordb.com/) API.
//!
//! # Examples
//! ```
//! use factordb::Number;
//! # fn main() {
//!   let num = Number::get(36).unwrap();
//!   println!("36 = {}", num);
//!
//!   let mut product = BigUint::from(1u32);
//!   for f in num.factor_list().iter() {
//!       product *= f;
//!   }
//!  assert_eq!(num.id(), product);
//! # }
//! ```

#![warn(missing_docs)]

use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

const ENDPOINT: &str = "http://factordb.com/api";

/// A number entry in FactorDB. Contains the number itself, its status in the database as well as its
/// factors.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Number {
    id: String,
    status: NumberStatus,
    factors: Vec<Factor>,
}

impl Number {
    /// Returns the queried number as a [`BigUint`].
    pub fn id(&self) -> BigUint {
        BigUint::from_str(&self.id).unwrap()
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

    /// Returns a vector [`BigUint`] containing the number's factors, with its exponents expanded.
    pub fn factor_list(&self) -> Vec<BigUint> {
        let mut out = vec![];
        for f in &self.factors {
            for _ in 0..f.1 {
                out.push(f.base())
            }
        }
        out
    }

    /// Sends a GET request to the FactorDB API to query the given number.
    ///
    /// If you are planning on making multiple requests, it is best to use [`Self::with_client()`]
    /// instead and reuse the client, taking advantage of keep-alive connection pooling.
    /// ([Learn more](https://docs.rs/reqwest/0.11.10/reqwest/blocking/index.html#making-a-get-request))
    ///
    /// # Errors
    /// Returns an [`FactorDbError`] if the number is invalid or there is something wrong in the
    /// request.
    ///
    /// # Panics
    /// This function cannot be executed in an async runtime, as per [`reqwest::blocking`] restriction.
    pub fn get<T: Display>(number: T) -> Result<Self, FactorDbError> {
        Self::with_client(number, reqwest::blocking::Client::new())
    }

    /// Sends a GET request to the FactorDB API to query the given number, using a supplied [`reqwest::blocking::Client`].
    ///
    /// # Errors
    /// Returns an [`FactorDbError`] if the number is invalid or there is something wrong in the
    /// request.
    ///
    /// # Panics
    /// This function cannot be executed in an async runtime, as per [`reqwest::blocking`] restriction.
    pub fn with_client<T: Display>(number: T, client: reqwest::blocking::Client) -> Result<Self, FactorDbError> {
        let url = format!("{}?query={}", ENDPOINT, number);
        match client.get(url).send()?.json() {
            Ok(n) => Ok(n),
            Err(e) => Err(e.into())
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let factor_strings: Vec<String> = self.factors.iter().map(|f| format!("{}", f)).collect();
        write!(f, "{}", factor_strings.join(" + "))
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Number {}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id().cmp(&other.id())
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A struct representing a factor with a unique base, along with the exponent (i.e. how many times
/// the factor is repeated).
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Factor(String, u32);

impl Factor {
    /// Returns the base as a [`BigUint`].
    pub fn base(&self) -> BigUint { BigUint::from_str(&self.0).unwrap() }

    /// Returns the exponent as a [`BigUint`].
    pub fn exponent(&self) -> BigUint { BigUint::from(self.1) }
}

impl Display for Factor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}^{}", self.base(), self.exponent())
    }
}

/// The status of a number in FactorDB.
#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Hash)]
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
    #[serde(rename = "Unit")]
    Unit,
    /// This number is not in database (N)
    #[serde(rename = "N")]
    NotInDatabase,
}

/// Error type in this crate.
#[derive(thiserror::Error, Debug)]
pub enum FactorDbError {
    /// Invalid number or request error
    #[error("Invalid number or request error")]
    RequestError(#[from] reqwest::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_42() {
        let num = Number::get(42).unwrap();
        println!("{}", num);
    }

    fn test_exponents() {
        let num = Number::get(36).unwrap();
        let mut product = BigUint::from(1u32);
        for f in num.factor_list().iter() {
            product *= f;
        }
        assert_eq!(num.id(), product);
    }
}
