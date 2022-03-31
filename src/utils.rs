use std::fmt;
use serde::{de, Deserializer};
use serde::de::Unexpected;

// https://stackoverflow.com/questions/37870428/convert-two-types-into-a-single-type-with-serde

struct DeserializeIdVisitor;

impl<'de> de::Visitor<'de> for DeserializeIdVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer or a string")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == -1_i64 {
            Ok(v.to_string())
        } else {
            Err(E::invalid_value(Unexpected::Signed(v), &self))
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }
}

pub(crate) fn deserialize_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeIdVisitor)
}