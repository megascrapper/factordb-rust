use std::fmt::{Display, Formatter};
use std::str::FromStr;

use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

const ENDPOINT: &str = "http://factordb.com/api";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Number {
    id: String,
    status: NumberStatus,
    factors: Vec<Factor>,
}

impl Number {
    pub fn id(&self) -> BigUint {
        BigUint::from_str(&self.id).unwrap()
    }

    pub fn status(&self) -> &NumberStatus {
        &self.status
    }
    pub fn factors(&self) -> &Vec<Factor> {
        &self.factors
    }

    pub fn is_prime(&self) -> bool {
        self.status == NumberStatus::DefinitelyPrime || self.status == NumberStatus::ProbablyPrime
    }

    pub fn is_definitely_prime(&self) -> bool {
        self.status == NumberStatus::DefinitelyPrime
    }

    pub fn factor_list(&self) -> Vec<BigUint> {
        let mut out = vec![];
        for f in &self.factors {
            for _ in 0..f.1 {
                out.push(f.base())
            }
        }
        out
    }

    pub fn get<T: Display>(number: T) -> Result<Self, FactorDbError> {
        Self::with_client(number, reqwest::blocking::Client::new())
    }

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


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Factor(String, u32);

impl Factor {
    pub fn base(&self) -> BigUint { BigUint::from_str(&self.0).unwrap() }

    pub fn exponent(&self) -> BigUint { BigUint::from(self.1) }
}

impl Display for Factor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}^{}", self.base(), self.exponent())
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum NumberStatus {
    #[serde(rename = "C")]
    NoFactorsKnown,
    #[serde(rename = "CF")]
    FactorsKnown,
    #[serde(rename = "FF")]
    FullyFactored,
    #[serde(rename = "P")]
    DefinitelyPrime,
    #[serde(rename = "Prp")]
    ProbablyPrime,
    #[serde(rename = "U")]
    Unknown,
    #[serde(rename = "Unit")]
    Unit,
    #[serde(rename = "N")]
    NotInDatabase,
}

#[derive(thiserror::Error, Debug)]
pub enum FactorDbError {
    #[error("Invalid number")]
    InvalidNumber(#[from] reqwest::Error),
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
