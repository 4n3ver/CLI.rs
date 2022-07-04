use std::{
    collections::HashMap,
    fmt,
    time::{Duration, Instant},
};

use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(bound(deserialize = "T: de::DeserializeOwned"))]
pub struct TryParse<T> {
    #[serde(flatten)]
    pub value: TryParseResult<T>,

    #[serde(flatten)]
    pub raw: HashMap<String, Value>,
}

#[derive(Debug)]
pub enum TryParseResult<T> {
    Parsed(T),
    Unparsed(Value),
    NotPresent,
}

impl<'de, T: de::DeserializeOwned> Deserialize<'de> for TryParseResult<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match Option::<Value>::deserialize(deserializer)? {
            None => Ok(TryParseResult::NotPresent),
            Some(value) => match T::deserialize(&value) {
                Ok(t) => Ok(TryParseResult::Parsed(t)),
                Err(_) => Ok(TryParseResult::Unparsed(value)),
            },
        }
    }
}

pub fn deserialize_yes_no<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    if "YES".eq_ignore_ascii_case(s) || "Y".eq_ignore_ascii_case(s) {
        Ok(true)
    } else if "NO".eq_ignore_ascii_case(s) || "N".eq_ignore_ascii_case(s) {
        Ok(false)
    } else {
        Err(de::Error::unknown_variant(s, &["YES", "NO", "Y", "N"]))
    }
}

pub fn deserialize_secs_from_now<'de, D>(deserializer: D) -> Result<Instant, D::Error>
where
    D: de::Deserializer<'de>,
{
    fn secs_from_now(secs: i32) -> Instant {
        if secs >= 0 {
            Instant::now() + Duration::from_secs(secs as u64)
        } else {
            Instant::now() - Duration::from_secs(secs.abs() as u64)
        }
    }

    struct SecsVisitor;
    impl<'de> de::Visitor<'de> for SecsVisitor {
        type Value = Instant;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("Secs as a number or string")
        }

        fn visit_i32<E>(self, secs: i32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(secs_from_now(secs))
        }

        fn visit_str<E>(self, secs: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            secs.parse().map(secs_from_now).map_err(de::Error::custom)
        }
    }

    deserializer.deserialize_any(SecsVisitor)
}
