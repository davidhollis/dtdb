use std::{marker::PhantomData, error::Error, fmt::{Debug, Display}};

use diesel::{deserialize::FromSql, sql_types::Text, backend::RawValue, pg::Pg, FromSqlRow, serialize::ToSql, AsExpression};
use rand::{thread_rng, distributions::{Alphanumeric, DistString}};
use serde::{de, Serialize, Deserialize, Deserializer, de::Visitor};

// re-export the identifier_prefix(...) annotation
pub use identifier_prefix::identifier_prefix;

pub trait IdentifierPrefix {
    fn identifier_prefix() -> &'static str;
}

#[derive(Debug, FromSqlRow, AsExpression)]
#[diesel(sql_type = Text)]
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

impl<T> Display for Identifier<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token)
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

#[derive(Debug)]
pub struct InvalidPrefixError {
    expected_prefix: String
}

impl Display for InvalidPrefixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "expected database identifier to have prefix {}", self.expected_prefix)
    }
}

impl Error for InvalidPrefixError {}

impl<T: IdentifierPrefix> TryFrom<String> for Identifier<T> {
    type Error = InvalidPrefixError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let expected_prefix = <T as IdentifierPrefix>::identifier_prefix();
        if value.starts_with(expected_prefix) {
            Ok(
                Identifier {
                    token: value,
                    _t: PhantomData::default()
                }
            )
        } else {
            Err(InvalidPrefixError {expected_prefix: expected_prefix.to_owned()})
        }
    }
}

impl<T> Serialize for Identifier<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
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
    type Value = Identifier<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string prefixed with {}", <T as IdentifierPrefix>::identifier_prefix())
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        v.to_owned().try_into().map_err(|err: InvalidPrefixError| E::custom(err.to_string()))
    }
}

impl<'de, T: IdentifierPrefix> Deserialize<'de> for Identifier<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(deserializer.deserialize_string(PrefixedStringVisitor::<T>::new())?)
    }
}

impl<T: IdentifierPrefix> FromSql<Text, Pg> for Identifier<T> {
    fn from_sql(bytes: RawValue<'_, Pg>) -> diesel::deserialize::Result<Self> {
        String::from_sql(bytes)?.try_into().map_err(|err: InvalidPrefixError| err.to_string().into())
    }
}

impl<T: Debug> ToSql<Text, Pg> for Identifier<T> {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        <String as ToSql<Text, Pg>>::to_sql(&self.token, out)
    }
}