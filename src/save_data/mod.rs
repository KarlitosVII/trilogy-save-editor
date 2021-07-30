use anyhow::Result;
use derive_more::{Deref, DerefMut, From, Display};
use serde::{de, ser::{self, SerializeSeq}, Serialize};
use uuid::Uuid;
use std::fmt;

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

#[derive(Clone, Display)]
#[display(fmt = "{}-{}-{}-{}-{}", part1, part2, part3, part4, part5)]
pub struct Guid {
    pub part1: String,
    pub part2: String,
    pub part3: String,
    pub part4: String,
    pub part5: String,
}

impl Default for Guid {
    fn default() -> Self {
        let mut part1 = String::with_capacity(8);
        part1.push_str("00000000");
        let mut part2 = String::with_capacity(4);
        part2.push_str("0000");
        let mut part3 = String::with_capacity(4);
        part3.push_str("0000");
        let mut part4 = String::with_capacity(4);
        part4.push_str("0000");
        let mut part5 = String::with_capacity(12);
        part5.push_str("000000000000");

        Guid { part1, part2, part3, part4, part5 }
    }
}

impl<'de> serde::Deserialize<'de> for Guid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct GuidVisitor;
        impl<'de> de::Visitor<'de> for GuidVisitor {
            type Value = Guid;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a Guid")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut result = [0u8; 16];
                let mut i = 0;
                while let Some(element) = seq.next_element()? {
                    result[i] = element;
                    i += 1;
                }

                let guid =
                    Uuid::from_slice(&result).map_err(de::Error::custom)?.to_simple().to_string();

                let mut part1 = String::with_capacity(8);
                part1.push_str(&guid[0..8]);
                let mut part2 = String::with_capacity(4);
                part2.push_str(&guid[8..12]);
                let mut part3 = String::with_capacity(4);
                part3.push_str(&guid[12..16]);
                let mut part4 = String::with_capacity(4);
                part4.push_str(&guid[16..20]);
                let mut part5 = String::with_capacity(12);
                part5.push_str(&guid[20..32]);

                Ok(Guid { part1, part2, part3, part4, part5 })
            }
        }
        deserializer.deserialize_tuple_struct("Guid", 16, GuidVisitor)
    }
}

impl serde::Serialize for Guid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let guid_str =
            self.part1.to_owned() + &self.part2 + &self.part3 + &self.part4 + &self.part5;
        let guid = List(
            Uuid::parse_str(&guid_str)
                .map_err(|err| ser::Error::custom(format!("GUID: {}", err)))?
                .as_bytes()
                .to_vec(),
        );
        serde::Serialize::serialize(&guid, serializer)
    }
}
