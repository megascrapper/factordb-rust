//! Contains response struct for FactorDB API requests, excluding the factors (see [`crate::factor`] for that).
//!
//! Both [`Number`] and [`NumberStatus`] are re-exported so importing this module directly isn't necessary.

use std::fmt::{Display, Formatter};

use num_bigint::BigInt;

use serde::{Deserialize, Serialize};

use crate::utils::deserialize_id;
use crate::Factor;

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

    /// Returns a vector of unique factors of this number.
    pub fn unique_factors(&self) -> Vec<&BigInt> {
        self.factors.iter().map(|f| f.base()).collect()
    }

    /// Converts `self` to a vector of unique factors of this number in ascending order.
    pub fn into_unique_factors(self) -> Vec<BigInt> {
        let mut factors: Vec<BigInt> = self.factors
            .into_iter()
            .map(|f| f.base().to_owned())
            .collect();
        factors.sort_unstable();
        factors
    }

    /// Converts `self` to a vector of [`BigInt`] containing the number's factors, with its exponents expanded in ascending order.
    pub fn into_factors_flattened(self) -> Vec<BigInt> {
        let mut factors: Vec<BigInt> = self.factors.clone().into_iter().flatten().collect();
        factors.sort_unstable();
        factors
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let factor_strings: Vec<String> = self
            .clone()
            .into_factors_flattened()
            .iter()
            .map(|n| n.to_string())
            .collect();
        write!(f, "{}", factor_strings.join(" "))
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
