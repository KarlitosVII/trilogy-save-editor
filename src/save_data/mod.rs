pub mod mass_effect_1;
pub mod mass_effect_1_le;
pub mod mass_effect_2;
pub mod mass_effect_3;
pub mod shared;

use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::{self, Display},
    rc::Rc,
};

use anyhow::Result;
use serde::{de, ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

// RcRef
#[derive(Clone, Default)]
pub struct RcRef<T>(Rc<RefCell<T>>);

impl<T> RcRef<T> {
    pub fn new(inner: T) -> Self {
        RcRef(Rc::new(RefCell::new(inner)))
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        RefCell::borrow(&self.0)
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        RefCell::borrow_mut(&self.0)
    }
}

impl<T> From<T> for RcRef<T> {
    fn from(from: T) -> Self {
        Self::new(from)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for RcRef<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner: T = Deserialize::deserialize(deserializer)?;
        Ok(inner.into())
    }
}

impl<T: Serialize> serde::Serialize for RcRef<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.borrow().serialize(serializer)
    }
}

impl<T> PartialEq for RcRef<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: Display> Display for RcRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.borrow().fmt(f)
    }
}

// Implémentation des dummy
#[derive(Clone)]
pub struct Dummy<const LEN: usize>([u8; LEN]);

impl<const LEN: usize> Default for Dummy<LEN> {
    fn default() -> Self {
        Self([0; LEN])
    }
}

impl<'de, const LEN: usize> Deserialize<'de> for Dummy<LEN> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
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
        S: Serializer,
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

impl<'de> Deserialize<'de> for List<u8> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
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
        S: Serializer,
    {
        let mut s = serializer.serialize_seq(None)?;
        for element in &self.0 {
            s.serialize_element(element)?;
        }
        s.end()
    }
}

#[derive(Clone, From, Display, Default)]
#[display(fmt = "")]
pub struct Guid(Uuid);

impl Guid {
    pub fn hyphenated(&self) -> String {
        self.0.to_hyphenated().to_string()
    }
}

impl<'de> Deserialize<'de> for Guid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (d1, d2, d3, d4): (u32, u16, u16, [u8; 8]) = Deserialize::deserialize(deserializer)?;
        let guid = Uuid::from_fields(d1, d2, d3, &d4).map_err(de::Error::custom)?;
        Ok(Guid(guid))
    }
}

impl serde::Serialize for Guid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serde::Serialize::serialize(&self.0.as_fields(), serializer)
    }
}
