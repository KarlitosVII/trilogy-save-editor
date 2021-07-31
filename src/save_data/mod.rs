use anyhow::Result;
use derive_more::{Deref, DerefMut, Display, From};
use serde::{
    de,
    ser::{self, SerializeSeq},
    Serialize,
};
use std::fmt;
use uuid::Uuid;

// pub mod mass_effect_1;
// pub mod mass_effect_1_le;
pub mod mass_effect_2;
// pub mod mass_effect_3;
pub mod shared;

// Raw Ui
// Implémentation des dummy
#[derive(Clone)]
pub struct Dummy<const LEN: usize>([u8; LEN]);

impl<const LEN: usize> Default for Dummy<LEN> {
    fn default() -> Self {
        Self([0; LEN])
    }
}

impl<'de, const LEN: usize> serde::Deserialize<'de> for Dummy<LEN> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DummyVisitor<const LEN: usize>;
        impl<'de, const LEN: usize> de::Visitor<'de> for DummyVisitor<LEN> {
            type Value = Dummy<LEN>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a Dummy<LEN>")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut result = [0u8; LEN];
                let mut i = 0;
                while let Some(element) = seq.next_element()? {
                    result[i] = element;
                    i += 1;
                }
                Ok(Dummy(result))
            }
        }
        deserializer.deserialize_tuple_struct("Dummy<LEN>", LEN, DummyVisitor)
    }
}

impl<const LEN: usize> serde::Serialize for Dummy<LEN> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

// List<T> : Vec<T> qui se (dé)sérialise sans précision de longueur
#[derive(Deref, DerefMut, From, Clone)]
pub struct List<T>(Vec<T>)
where
    T: Serialize + Clone;

impl<T> From<&[T]> for List<T>
where
    T: Serialize + Clone,
{
    fn from(from: &[T]) -> List<T> {
        List(from.to_vec())
    }
}

impl<'de> serde::Deserialize<'de> for List<u8> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ByteListVisitor;
        impl<'de> de::Visitor<'de> for ByteListVisitor {
            type Value = List<u8>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a byte buf")
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(List(v))
            }
        }
        deserializer.deserialize_byte_buf(ByteListVisitor)
    }
}

impl<T> serde::Serialize for List<T>
where
    T: Serialize + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_seq(None)?;
        for element in &self.0 {
            s.serialize_element(element)?;
        }
        s.end()
    }
}

#[derive(Clone, From, Display)]
pub struct Guid(String);

impl Default for Guid {
    fn default() -> Self {
        Guid(Uuid::default().to_hyphenated().to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Guid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: [u8; 16] = serde::Deserialize::deserialize(deserializer)?;
        let guid = Uuid::from_bytes(bytes);

        Ok(Guid(guid.to_hyphenated().to_string()))
    }
}

impl serde::Serialize for Guid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let guid =
            Uuid::parse_str(&self.0).map_err(|err| ser::Error::custom(format!("GUID: {}", err)))?;

        serde::Serialize::serialize(guid.as_bytes(), serializer)
    }
}
