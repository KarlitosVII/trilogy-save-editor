use encoding_rs::{UTF_16LE, WINDOWS_1252};
use serde::{
    de::{
        self, DeserializeSeed, EnumAccess, Error, IntoDeserializer, MapAccess, SeqAccess,
        VariantAccess, Visitor,
    },
    Deserialize,
};
use std::{convert::TryInto, mem::size_of};

use super::Result;

pub struct Deserializer<'de> {
    input: &'de [u8],
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes<T: Deserialize<'de>>(input: &'de [u8]) -> Result<T> {
        let mut deserializer = Deserializer { input };
        T::deserialize(&mut deserializer)
    }

    fn read(&mut self, num_bytes: usize) -> Result<&[u8]> {
        if num_bytes > self.input.len() {
            return Err(super::Error::Eof);
        }

        let slice = &self.input[..num_bytes];
        self.input = &self.input[num_bytes..];

        Ok(slice)
    }

    fn read_to_end(&mut self) -> Result<&[u8]> {
        self.read(self.input.len())
    }
}

macro_rules! unimpl_deserialize {
    ($de_method:ident()) => {
        fn $de_method<V>(self, _: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }
    };
}

macro_rules! impl_deserialize {
    ($de_method:ident($type:ty) = $visit_type:ident()) => {
        fn $de_method<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            const SIZE: usize = size_of::<$type>();
            let bytes = self.read(SIZE)?;
            let value = <$type>::from_le_bytes(bytes.try_into().map_err(Error::custom)?);
            visitor.$visit_type(value)
        }
    };
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = super::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Only used by save_data::mass_effect_1_le::NoExport so far
        if self.input.is_empty() {
            return visitor.visit_none();
        }
        visitor.visit_some(self)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value: u32 = de::Deserialize::deserialize(&mut *self)?;
        match value {
            0 | 1 => visitor.visit_bool(value != 0),
            v => Err(Error::custom(format!("expected 1 or 0, found: {}", v))),
        }
    }

    // Signed ints
    unimpl_deserialize!(deserialize_i8());
    unimpl_deserialize!(deserialize_i16());

    impl_deserialize!(deserialize_i32(i32) = visit_i32()); // Impl

    unimpl_deserialize!(deserialize_i64());

    // Unsigned ints
    impl_deserialize!(deserialize_u8(u8) = visit_u8()); // Impl

    unimpl_deserialize!(deserialize_u16());

    impl_deserialize!(deserialize_u32(u32) = visit_u32()); // Impl
    impl_deserialize!(deserialize_u64(u64) = visit_u64()); // Impl

    // Floats
    impl_deserialize!(deserialize_f32(f32) = visit_f32()); // Impl

    unimpl_deserialize!(deserialize_f64());

    // Char
    unimpl_deserialize!(deserialize_char());

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len: i32 = de::Deserialize::deserialize(&mut *self)?;
        if len == 0 {
            return visitor.visit_borrowed_str("");
        }

        let string = if len < 0 {
            // Unicode
            let string_len = (len.abs() * 2) as usize;
            let bytes = self.read(string_len)?.to_owned();

            let (decoded, _, had_errors) = UTF_16LE.decode(&bytes);
            if had_errors {
                return Err(Error::custom("UTF_16LE decoding error"));
            }

            decoded.into_owned()
        } else {
            // Ascii
            let string_len = len as usize;
            let bytes = self.read(string_len)?.to_owned();

            let (decoded, _, had_errors) = WINDOWS_1252.decode(&bytes);
            if had_errors {
                return Err(Error::custom("WINDOWS_1252 decoding error"));
            }

            decoded.into_owned()
        };

        visitor.visit_string(string)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bytes(self.read_to_end()?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_byte_buf(self.read_to_end()?.to_owned())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value: bool = de::Deserialize::deserialize(&mut *self)?;
        match value {
            true => visitor.visit_some(&mut *self),
            false => visitor.visit_none(),
        }
    }

    fn deserialize_unit<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(super::Error::Eof)
    }

    fn deserialize_unit_struct<V>(self, _: &'static str, _: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len: u32 = de::Deserialize::deserialize(&mut *self)?;
        visitor.visit_seq(SizedSeqMap::new(&mut self, len as usize))
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len: u32 = de::Deserialize::deserialize(&mut *self)?;
        visitor.visit_map(SizedSeqMap::new(&mut self, len as usize))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SizedSeqMap::new(self, len))
    }

    fn deserialize_tuple_struct<V>(
        self, _: &'static str, len: usize, visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_struct<V>(
        self, name: &'static str, fields: &'static [&'static str], visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple_struct(name, fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self, _: &'static str, _: &'static [&'static str], visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        impl<'de, 'a> EnumAccess<'de> for &'a mut Deserializer<'de> {
            type Error = super::Error;
            type Variant = Self;

            fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
            where
                V: DeserializeSeed<'de>,
            {
                let idx: u8 = de::Deserialize::deserialize(&mut *self)?;
                let val = seed.deserialize(idx.into_deserializer())?;
                Ok((val, self))
            }
        }

        visitor.visit_enum(self)
    }

    unimpl_deserialize!(deserialize_identifier());
    unimpl_deserialize!(deserialize_ignored_any());

    fn is_human_readable(&self) -> bool {
        false
    }
}

struct SizedSeqMap<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    len: usize,
}

impl<'a, 'de> SizedSeqMap<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, len: usize) -> Self {
        SizedSeqMap { de, len }
    }
}

impl<'de, 'a> SeqAccess<'de> for SizedSeqMap<'a, 'de> {
    type Error = super::Error;

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.len == 0 {
            return Ok(None);
        }
        self.len -= 1;

        seed.deserialize(&mut *self.de).map(Some)
    }
}

impl<'de, 'a> MapAccess<'de> for SizedSeqMap<'a, 'de> {
    type Error = super::Error;

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.len == 0 {
            return Ok(None);
        }
        self.len -= 1;

        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

impl<'de, 'a> VariantAccess<'de> for &'a mut Deserializer<'de> {
    type Error = super::Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }

    fn tuple_variant<V>(self, _: usize, _: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        use serde::de::Deserializer;
        self.deserialize_tuple(fields.len(), visitor)
    }
}
