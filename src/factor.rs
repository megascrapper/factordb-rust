//! Contains struct containing a factor and its iterators.

use std::fmt::{Display, Formatter};

use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

use crate::utils::{deserialize_string_to_bigint, deserialize_u64_to_bigint};

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

    /// Iterate over the base by the exponent value.
    pub fn iter(&self) -> Iter {
        Iter {
            base: &self.0,
            remaining_exp: self.1.clone(),
        }
    }
}

impl Display for Factor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl IntoIterator for Factor {
    type Item = BigInt;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            base: self.0,
            remaining_exp: self.1,
        }
    }
}

/// Iterator which repeats the base of a [`Factor`] by the number of exponent as instances of `&BigInt`.
///
/// See also: [`IntoIter`]
pub struct Iter<'f> {
    base: &'f BigInt,
    remaining_exp: BigInt,
}

impl<'f> Iterator for Iter<'f> {
    type Item = &'f BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_exp > BigInt::from(0) {
            self.remaining_exp -= 1;
            Some(self.base)
        } else {
            None
        }
    }
}

/// Iterator which repeats the base of a [`Factor`] by the number of exponent as instances of `BigInt`.
///
/// See also: [`Iter`]
pub struct IntoIter {
    base: BigInt,
    remaining_exp: BigInt,
}

impl Iterator for IntoIter {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_exp > BigInt::from(0) {
            self.remaining_exp -= 1;
            Some(self.base.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factor_iter() {
        let a_million = Factor(BigInt::from(10), BigInt::from(6));
        assert_eq!(
            a_million.iter().product::<BigInt>(),
            BigInt::from(1_000_000)
        );
        assert_eq!(
            a_million.iter().cloned().collect::<Vec<_>>(),
            vec![
                BigInt::from(10),
                BigInt::from(10),
                BigInt::from(10),
                BigInt::from(10),
                BigInt::from(10),
                BigInt::from(10)
            ]
        )
    }

    #[test]
    fn test_factor_into_iter() {
        let a_million = Factor(BigInt::from(10), BigInt::from(6));
        assert_eq!(
            a_million.clone().into_iter().product::<BigInt>(),
            BigInt::from(1_000_000)
        );
        assert_eq!(
            a_million.into_iter().collect::<Vec<_>>(),
            vec![
                BigInt::from(10),
                BigInt::from(10),
                BigInt::from(10),
                BigInt::from(10),
                BigInt::from(10),
                BigInt::from(10)
            ]
        )
    }
}
