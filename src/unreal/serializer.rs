use encoding_rs::WINDOWS_1252;
use serde::{
    ser::{self, Error},
    Serialize,
};

use super::Result;

pub struct Serializer {
    output: Vec<u8>,
}

impl Serializer {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
    where
        T: Serialize,
    {
        let mut serializer = Serializer { output: Vec::new() };
        value.serialize(&mut serializer)?;
        Ok(serializer.output)
    }
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

    fn serialize_i8(self, _: i8) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i16(self, _: i16) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i32(self, value: i32) -> Result<()> {
        let bytes = i32::to_le_bytes(value);
        self.output.extend(&bytes);
        Ok(())
    }

    fn serialize_i64(self, _: i64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u8(self, value: u8) -> Result<()> {
        let bytes = u8::to_le_bytes(value);
        self.output.extend(&bytes);
        Ok(())
    }

    fn serialize_u16(self, _: u16) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u32(self, value: u32) -> Result<()> {
        let bytes = u32::to_le_bytes(value);
        self.output.extend(&bytes);
        Ok(())
    }

    fn serialize_u64(self, value: u64) -> Result<()> {
        let bytes = u64::to_le_bytes(value);
        self.output.extend(&bytes);
        Ok(())
    }

    fn serialize_f32(self, value: f32) -> Result<()> {
        let bytes = f32::to_le_bytes(value);
        self.output.extend(&bytes);
        Ok(())
    }

    fn serialize_f64(self, _: f64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_char(self, _: char) -> Result<()> {
        unimplemented!()
    }

    fn serialize_str(self, string: &str) -> Result<()> {
        if string.is_empty() {
            return self.serialize_u32(0);
        }

        let (bytes, len) = if string.chars().any(|c| c as u32 > 0xff) {
            // Unicode
            let mut encoded: Vec<u16> = string.encode_utf16().collect();
            encoded.push(0);

            let mut bytes = Vec::new();
            for doublebyte in encoded.drain(..) {
                bytes.extend(&u16::to_le_bytes(doublebyte));
            }

            let len = bytes.len() as i32;
            (bytes, -(len / 2))
        } else {
            // Ascii
            let (encoded, _, had_errors) = WINDOWS_1252.encode(&string);
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
        unimplemented!()
    }

    fn serialize_some<T: ?Sized>(self, _: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<()> {
        unimplemented!()
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
        unimplemented!()
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
        match len {
            Some(len) => self.serialize_u32(len as u32)?,
            None => self.serialize_u32(0)?,
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

    fn serialize_element<T: ?Sized>(&mut self, _: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
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
