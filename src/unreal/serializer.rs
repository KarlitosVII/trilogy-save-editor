use std::iter::once;

use encoding_rs::WINDOWS_1252;
use serde::ser::{self, Error};
use serde::Serialize;

use super::Result;

pub struct Serializer {
    output: Vec<u8>,
    is_le: bool,
}

impl Serializer {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize,
    {
        let mut serializer = Serializer { output: Vec::new(), is_le: true };
        value.serialize(&mut serializer)?;
        Ok(serializer.output)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_be_vec<T>(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize,
    {
        let mut serializer = Serializer { output: Vec::new(), is_le: false };
        value.serialize(&mut serializer)?;
        Ok(serializer.output)
    }
}

macro_rules! unimpl_serialize {
    ($ser_method:ident($type:ty)) => {
        fn $ser_method(self, _: $type) -> Result<()> {
            unimplemented!()
        }
    };
}

macro_rules! impl_serialize {
    ($ser_method:ident($type:ty)) => {
        fn $ser_method(self, value: $type) -> Result<()> {
            let bytes =
                if self.is_le { <$type>::to_le_bytes(value) } else { <$type>::to_be_bytes(value) };
            self.output.extend(&bytes);
            Ok(())
        }
    };
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = super::Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, value: bool) -> Result<()> {
        self.serialize_u32(value as u32)
    }

    // Signed ints
    unimpl_serialize!(serialize_i8(i8));
    unimpl_serialize!(serialize_i16(i16));

    impl_serialize!(serialize_i32(i32)); // Impl

    unimpl_serialize!(serialize_i64(i64));

    // Unsigned ints
    impl_serialize!(serialize_u8(u8)); // Impl
    impl_serialize!(serialize_u16(u16)); // Impl
    impl_serialize!(serialize_u32(u32)); // Impl
    impl_serialize!(serialize_u64(u64)); // Impl

    // Floats
    impl_serialize!(serialize_f32(f32)); // Impl

    unimpl_serialize!(serialize_f64(f64));

    // Char
    unimpl_serialize!(serialize_char(char));

    fn serialize_str(self, string: &str) -> Result<()> {
        if string.is_empty() {
            return self.serialize_u32(0);
        }

        let (bytes, len) = if string.chars().any(|c| c as u32 > 0xff) {
            // Unicode
            let encoded: Vec<u8> = string
                .encode_utf16()
                .chain(once(0))
                .flat_map(|c| if self.is_le { u16::to_le_bytes(c) } else { u16::to_be_bytes(c) })
                .collect();

            let len = encoded.len() as i32;
            (encoded, -(len / 2))
        } else {
            // Ascii
            let (encoded, _, had_errors) = WINDOWS_1252.encode(string);
            if had_errors {
                return Err(Error::custom("WINDOWS_1252 encoding error"));
            }

            let mut encoded = encoded.into_owned();
            encoded.push(0);

            let len = encoded.len() as i32;
            (encoded, len)
        };

        self.serialize_i32(len)?;
        self.output.extend(bytes);
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        self.output.extend(value);
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_bool(false)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.serialize_bool(true)?;
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self, _: &'static str, variant_index: u32, _: &'static str,
    ) -> Result<()> {
        self.serialize_u8(variant_index as u8)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self, _: &'static str, _: u32, _: &'static str, value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        if let Some(len) = len {
            self.serialize_u32(len as u32)?;
        }
        Ok(self)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self, _: &'static str, _: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self, _: &'static str, _: u32, _: &'static str, _: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        if let Some(len) = len {
            self.serialize_u32(len as u32)?;
        }
        Ok(self)
    }

    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self, _: &'static str, _: u32, _: &'static str, _: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(self)
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = super::Error;

    fn serialize_field<T: ?Sized>(&mut self, _: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = super::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = super::Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = super::Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = super::Error;

    fn serialize_field<T: ?Sized>(&mut self, _: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = super::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = super::Error;

    fn serialize_field<T: ?Sized>(&mut self, _: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}
