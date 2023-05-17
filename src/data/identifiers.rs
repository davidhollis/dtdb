use std::marker::PhantomData;

use rand::{thread_rng, distributions::{Alphanumeric, DistString}};
use serde::{de, Serialize, Deserialize, Deserializer, de::Visitor};

// re-export the identifier_prefix(...) annotation
pub use identifier_prefix::identifier_prefix;

pub trait IdentifierPrefix {
    fn identifier_prefix() -> &'static str;
}

#[derive(Debug)]
pub struct Identifier<T> {
    pub token: String,
    _t: PhantomData<T>
}

impl<T: IdentifierPrefix> Default for Identifier<T> {
    fn default() -> Self {
        let prefix = <T as IdentifierPrefix>::identifier_prefix();
        let random_part = Alphanumeric.sample_string(&mut thread_rng(), 24);
        Identifier {
            token: format!("{prefix}_{random_part}"),
            _t: PhantomData::default()
        }
    }
}

impl<T> Serialize for Identifier<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(&self.token)
    }
}

struct PrefixedStringVisitor<T> {
    _t: PhantomData<T>
}

impl<T: IdentifierPrefix> PrefixedStringVisitor<T> {
    fn new() -> PrefixedStringVisitor<T> {
        PrefixedStringVisitor { _t: PhantomData::default() }
    }
}

impl<'de, T: IdentifierPrefix> Visitor<'de> for PrefixedStringVisitor<T> {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string prefixed with {}", <T as IdentifierPrefix>::identifier_prefix())
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        let expected_prefix = <T as IdentifierPrefix>::identifier_prefix();
        if v.starts_with(expected_prefix) {
            Ok(v.to_owned())
        } else {
            Err(E::custom(format!("expected a prefix of {}", expected_prefix)))
        }
    }
}

impl<'de, T: IdentifierPrefix> Deserialize<'de> for Identifier<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(
            Identifier {
                token: deserializer.deserialize_string(PrefixedStringVisitor::<T>::new())?,
                _t: PhantomData::default()
            }
        )
    }
}

impl<T> Clone for Identifier<T> {
    fn clone(&self) -> Self {
        Identifier {
            token: self.token.clone(),
            _t: PhantomData::default()
        }
    }
}