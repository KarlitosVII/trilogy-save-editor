use anyhow::*;
use serde::Serialize;
use std::ops::{Deref, DerefMut};

use crate::{
    gui::Gui,
    save_data::{
        common::{appearance::LinearColor, Rotator, Vector},
        Dummy, ImguiString, List,
    },
    unreal,
};

use super::{player::Name, SaveCursor, SaveData};

#[derive(Serialize, Clone)]
pub struct Data {
    _osef: Dummy<4>,
    pub properties: List<Property>,
}

impl Data {
    pub fn new(names: &[Name], cursor: &mut SaveCursor) -> Result<Self> {
        let _osef: Dummy<4> = SaveData::deserialize(cursor)?;
        let mut properties = Vec::new();

        let mut finished = false;
        while !finished {
            let property = Property::new(&names, cursor)?;

            // Ça se termine toujours par un None donc on break ici
            if let Property::None { .. } = property {
                finished = true;
            }
            properties.push(property);
        }

        Ok(Self { _osef, properties: properties.into() })
    }

    pub fn size(&self) -> Result<usize> {
        let mut size = 4;
        for property in self.properties.iter() {
            size += property.size()?
        }
        Ok(size)
    }
}

impl Deref for Data {
    type Target = Vec<Property>;

    fn deref(&self) -> &Vec<Property> {
        &self.properties
    }
}

impl DerefMut for Data {
    fn deref_mut(&mut self) -> &mut Vec<Property> {
        &mut self.properties
    }
}

impl SaveData for Data {
    fn deserialize(_: &mut SaveCursor) -> Result<Self> {
        unreachable!()
    }

    fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
}

fn get_name(names: &[Name], id: u32) -> String {
    names[id as usize].to_string()
}

#[derive(Serialize, Clone)]
pub enum Property {
    Array {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        array: Vec<ArrayType>,
    },
    Bool {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        value: bool,
    },
    Byte {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        value: u8,
    },
    Float {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        value: f32,
    },
    Int {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        value: i32,
    },
    Name {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        value_name_id: u32,
        _osef4: Dummy<4>,
    },
    Object {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        object_id: i32,
    },
    Str {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        string: ImguiString,
    },
    StringRef {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        value: i32,
    },
    Struct {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        struct_name_id: u32,
        _osef4: Dummy<4>,
        properties: StructType,
    },
    None {
        name_id: u32,
        _osef: Dummy<4>,
    },
}

impl Property {
    pub fn new(names: &[Name], cursor: &mut SaveCursor) -> Result<Self> {
        // Name
        let name_id = SaveData::deserialize(cursor)?;
        let _osef1 = SaveData::deserialize(cursor)?;

        let name = get_name(names, name_id);
        if name == "None" {
            return Ok(Self::None { name_id, _osef: _osef1 });
        }

        // Type
        let type_id = SaveData::deserialize(cursor)?;
        let _osef2 = SaveData::deserialize(cursor)?;
        // Size
        let size = SaveData::deserialize(cursor)?;
        let _osef3 = SaveData::deserialize(cursor)?;

        let type_name = get_name(names, type_id);
        let property = match type_name.as_str() {
            "ArrayProperty" => {
                let len: u32 = SaveData::deserialize(cursor)?;
                let mut array = Vec::new();
                // Hardcodé sinon je dois chercher dans toutes les classes du jeu...
                match name.as_str() {
                    "m_PrereqTalentIDArray" | "m_PrereqTalentRankArray" => {
                        for _ in 0..len {
                            array.push(ArrayType::int(cursor)?);
                        }
                    }
                    "m_aItem"
                    | "m_aXMod"
                    | "m_aEquipped"
                    | "m_QuickSlotArray"
                    | "m_savedBuybackItems" => {
                        for _ in 0..len {
                            array.push(ArrayType::object(cursor)?);
                        }
                    }
                    "m_vPosition" => {
                        for _ in 0..len {
                            array.push(ArrayType::vector(cursor)?);
                        }
                    }
                    "m_DependentPackages" => {
                        for _ in 0..len {
                            array.push(ArrayType::string(cursor)?);
                        }
                    }
                    _ => {
                        for _ in 0..len {
                            array.push(ArrayType::properties(names, cursor)?);
                        }
                    }
                }
                Self::Array { name_id, _osef1, type_id, _osef2, size, _osef3, array }
            }
            "BoolProperty" => {
                let value = SaveData::deserialize(cursor)?;
                Self::Bool { name_id, _osef1, type_id, _osef2, size, _osef3, value }
            }
            "ByteProperty" => {
                if size == 1 {
                    let value = SaveData::deserialize(cursor)?;
                    Self::Byte { name_id, _osef1, type_id, _osef2, size, _osef3, value }
                } else {
                    let value_name_id = SaveData::deserialize(cursor)?;
                    let _osef4 = SaveData::deserialize(cursor)?;
                    Self::Name {
                        name_id,
                        _osef1,
                        type_id,
                        _osef2,
                        size,
                        _osef3,
                        value_name_id,
                        _osef4,
                    }
                }
            }
            "FloatProperty" => {
                let value = SaveData::deserialize(cursor)?;
                Self::Float { name_id, _osef1, type_id, _osef2, size, _osef3, value }
            }
            "IntProperty" => {
                let value = SaveData::deserialize(cursor)?;
                Self::Int { name_id, _osef1, type_id, _osef2, size, _osef3, value }
            }
            "NameProperty" => {
                let value_name_id = SaveData::deserialize(cursor)?;
                let _osef4 = SaveData::deserialize(cursor)?;
                Self::Name { name_id, _osef1, type_id, _osef2, size, _osef3, value_name_id, _osef4 }
            }
            "ObjectProperty" => {
                let object_id = SaveData::deserialize(cursor)?;
                Self::Object { name_id, _osef1, type_id, _osef2, size, _osef3, object_id }
            }
            "StrProperty" => {
                let string = SaveData::deserialize(cursor)?;
                Self::Str { name_id, _osef1, type_id, _osef2, size, _osef3, string }
            }
            "StringRefProperty" => {
                let value = SaveData::deserialize(cursor)?;
                Self::StringRef { name_id, _osef1, type_id, _osef2, size, _osef3, value }
            }
            "StructProperty" => {
                let struct_name_id = SaveData::deserialize(cursor)?;
                let _osef4 = SaveData::deserialize(cursor)?;

                let struct_name = get_name(names, struct_name_id);
                let properties = match struct_name.as_str() {
                    "LinearColor" => StructType::linear_color(cursor)?,
                    "Vector" => StructType::vector(cursor)?,
                    "Rotator" => StructType::rotator(cursor)?,
                    _ => StructType::properties(names, cursor)?,
                };
                Self::Struct {
                    name_id,
                    _osef1,
                    type_id,
                    _osef2,
                    size,
                    _osef3,
                    struct_name_id,
                    _osef4,
                    properties,
                }
            }
            _ => unimplemented!(),
        };
        Ok(property)
    }

    pub fn size(&self) -> Result<usize> {
        let mut size = 24;
        Ok(match self {
            Property::Array { array, .. } => {
                size += 4;
                for item in array {
                    size += item.size()?
                }
                size
            }
            Property::Bool { .. } => size + 4,
            Property::Byte { .. } => size + 1,
            Property::Float { .. } => size + 4,
            Property::Int { .. } => size + 4,
            Property::Name { .. } => size + 8,
            Property::Object { .. } => size + 4,
            Property::Str { string, .. } => {
                let bytes = unreal::Serializer::to_bytes(string)?;
                size + bytes.len()
            }
            Property::StringRef { .. } => size + 4,
            Property::Struct { properties, .. } => size + properties.size()? + 8,
            Property::None { .. } => 8,
        })
    }
}

impl SaveData for Property {
    fn deserialize(_: &mut SaveCursor) -> Result<Self> {
        unreachable!()
    }

    fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
}

#[derive(Serialize, Clone)]
pub enum ArrayType {
    Int(i32),
    Object(i32),
    Vector(Vector),
    String(ImguiString),
    Properties(List<Property>),
}

impl ArrayType {
    pub fn int(cursor: &mut SaveCursor) -> Result<Self> {
        Ok(Self::Int(SaveData::deserialize(cursor)?))
    }

    pub fn object(cursor: &mut SaveCursor) -> Result<Self> {
        Ok(Self::Object(SaveData::deserialize(cursor)?))
    }

    pub fn vector(cursor: &mut SaveCursor) -> Result<Self> {
        Ok(Self::Vector(SaveData::deserialize(cursor)?))
    }

    pub fn string(cursor: &mut SaveCursor) -> Result<Self> {
        Ok(Self::String(SaveData::deserialize(cursor)?))
    }

    pub fn properties(names: &[Name], cursor: &mut SaveCursor) -> Result<Self> {
        let mut properties = Vec::new();

        let mut finished = false;
        while !finished {
            let property = Property::new(&names, cursor)?;

            // Ça se termine toujours par un None donc on break ici
            if let Property::None { .. } = property {
                finished = true;
            }
            properties.push(property);
        }

        Ok(Self::Properties(properties.into()))
    }

    fn size(&self) -> Result<usize> {
        Ok(match self {
            ArrayType::Int(_) => 4,
            ArrayType::Object(_) => 4,
            ArrayType::Vector(_) => 12,
            ArrayType::String(string) => {
                let bytes = unreal::Serializer::to_bytes(string)?;
                bytes.len()
            }
            ArrayType::Properties(properties) => {
                let mut size = 0;
                for property in properties.iter() {
                    size += property.size()?
                }
                size
            }
        })
    }
}

impl SaveData for ArrayType {
    fn deserialize(_: &mut SaveCursor) -> Result<Self> {
        unreachable!()
    }

    fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
}

#[derive(Serialize, Clone)]
pub enum StructType {
    LinearColor(LinearColor),
    Vector(Vector),
    Rotator(Rotator),
    Properties(List<Property>),
}

impl StructType {
    pub fn linear_color(cursor: &mut SaveCursor) -> Result<Self> {
        Ok(Self::LinearColor(SaveData::deserialize(cursor)?))
    }

    pub fn vector(cursor: &mut SaveCursor) -> Result<Self> {
        Ok(Self::Vector(SaveData::deserialize(cursor)?))
    }

    pub fn rotator(cursor: &mut SaveCursor) -> Result<Self> {
        Ok(Self::Rotator(SaveData::deserialize(cursor)?))
    }

    pub fn properties(names: &[Name], cursor: &mut SaveCursor) -> Result<Self> {
        let mut properties = Vec::new();

        let mut finished = false;
        while !finished {
            let property = Property::new(&names, cursor)?;

            // Ça se termine toujours par un None donc on break ici
            if let Property::None { .. } = property {
                finished = true;
            }
            properties.push(property);
        }

        Ok(Self::Properties(properties.into()))
    }

    fn size(&self) -> Result<usize> {
        Ok(match self {
            StructType::LinearColor(_) => 16,
            StructType::Vector(_) => 12,
            StructType::Rotator(_) => 12,
            StructType::Properties(properties) => {
                let mut size = 0;
                for property in properties.iter() {
                    size += property.size()?
                }
                size
            }
        })
    }
}

impl SaveData for StructType {
    fn deserialize(_: &mut SaveCursor) -> Result<Self> {
        unreachable!()
    }

    fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
}
