use num_bigint::BigInt;
use serde::de::Unexpected;
use serde::{de, Deserializer};
use std::fmt;
use std::str::FromStr;

// https://stackoverflow.com/questions/37870428/convert-two-types-into-a-single-type-with-serde

struct DeserializeToBigIntVisitor;

impl<'de> de::Visitor<'de> for DeserializeToBigIntVisitor {
    type Value = BigInt;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer or a string")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(BigInt::from(v))
    }

    // added this method because apparently serde didn't think that `1` is a valid i64
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(BigInt::from(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        BigInt::from_str(v).map_err(|e| E::invalid_value(Unexpected::Str(&e.to_string()), &self))
    }
}

// Deserialize either an integer or string
pub(crate) fn deserialize_id<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeToBigIntVisitor)
}

// deserialize String to bigint ONLY
pub(crate) fn deserialize_string_to_bigint<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(DeserializeToBigIntVisitor)
}

// deserialize u64 to bigint ONLY
pub(crate) fn deserialize_u64_to_bigint<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_u64(DeserializeToBigIntVisitor)
}
