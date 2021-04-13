use std::mem::size_of;

use anyhow::Result;
use async_trait::async_trait;
use imgui::ImString;

use crate::{
    gui::Gui,
    save_data::{
        common::{appearance::LinearColor, Rotator, Vector},
        Dummy,
    },
};

use super::{player::Name, SaveCursor, SaveData};

#[derive(SaveData, Clone)]
pub struct Export {
    class_id: u32,
    class_parent_id: u32,
    link_id: u32,
    object_id: u32,
    value_id: u32,
    archtype_id: u32,
    flag: u64,
    pub data_size: u32,
    pub data_offset: u32,
    _osef: Dummy<32>,
}

#[derive(Clone)]
pub struct Data {
    _osef: Dummy<4>,
    properties: Vec<Property>,
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

        Ok(Self { _osef, properties })
    }

    pub fn size(&self) -> Result<usize> {
        let mut size = 4;
        for property in &self.properties {
            size += property.size()?
        }
        Ok(size)
    }
}

#[async_trait(?Send)]
impl SaveData for Data {
    fn deserialize(_: &mut SaveCursor) -> Result<Self> {
        unreachable!()
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        self._osef.serialize(output)?;
        for property in &self.properties {
            property.serialize(output)?;
        }
        Ok(())
    }

    async fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
}

macro_rules! serialize {
    ($output:ident, $($vars:ident),+) => {{
        $(
            $vars.serialize($output)?;
        )*
    }};
}

#[derive(Clone)]
enum Property {
    ArrayProperty {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        array: Vec<ArrayType>,
    },
    BoolProperty {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        value: bool,
    },
    ByteProperty {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        value: u8,
    },
    FloatProperty {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        value: f32,
    },
    IntProperty {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        value: i32,
    },
    NameProperty {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        value_name_id: u32,
        _osef4: Dummy<4>,
    },
    ObjectProperty {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        object_id: i32,
    },
    StrProperty {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        string: ImString,
    },
    StringRefProperty {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        value: i32,
    },
    StructProperty {
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

        let name = Self::get_name(names, name_id);
        if name == "None" {
            return Ok(Self::None { name_id, _osef: _osef1 });
        }

        // Type
        let type_id = SaveData::deserialize(cursor)?;
        let _osef2 = SaveData::deserialize(cursor)?;
        // Size
        let size = SaveData::deserialize(cursor)?;
        let _osef3 = SaveData::deserialize(cursor)?;

        let type_name = Self::get_name(names, type_id);
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
                Self::ArrayProperty { name_id, _osef1, type_id, _osef2, size, _osef3, array }
            }
            "BoolProperty" => {
                let value = SaveData::deserialize(cursor)?;
                Self::BoolProperty { name_id, _osef1, type_id, _osef2, size, _osef3, value }
            }
            "ByteProperty" => {
                if size == 1 {
                    let value = SaveData::deserialize(cursor)?;
                    Self::ByteProperty { name_id, _osef1, type_id, _osef2, size, _osef3, value }
                } else {
                    let value_name_id = SaveData::deserialize(cursor)?;
                    let _osef4 = SaveData::deserialize(cursor)?;
                    Self::NameProperty {
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
                Self::FloatProperty { name_id, _osef1, type_id, _osef2, size, _osef3, value }
            }
            "IntProperty" => {
                let value = SaveData::deserialize(cursor)?;
                Self::IntProperty { name_id, _osef1, type_id, _osef2, size, _osef3, value }
            }
            "NameProperty" => {
                let value_name_id = SaveData::deserialize(cursor)?;
                let _osef4 = SaveData::deserialize(cursor)?;
                Self::NameProperty {
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
            "ObjectProperty" => {
                let object_id = SaveData::deserialize(cursor)?;
                Self::ObjectProperty { name_id, _osef1, type_id, _osef2, size, _osef3, object_id }
            }
            "StrProperty" => {
                let string = SaveData::deserialize(cursor)?;
                Self::StrProperty { name_id, _osef1, type_id, _osef2, size, _osef3, string }
            }
            "StringRefProperty" => {
                let value = SaveData::deserialize(cursor)?;
                Self::StringRefProperty { name_id, _osef1, type_id, _osef2, size, _osef3, value }
            }
            "StructProperty" => {
                let struct_name_id = SaveData::deserialize(cursor)?;
                let _osef4 = SaveData::deserialize(cursor)?;

                let struct_name = Self::get_name(names, struct_name_id);
                let properties = match struct_name.as_str() {
                    "LinearColor" => StructType::linear_color(cursor)?,
                    "Vector" => StructType::vector(cursor)?,
                    "Rotator" => StructType::rotator(cursor)?,
                    _ => StructType::properties(names, cursor)?,
                };
                Self::StructProperty {
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

    fn get_name(names: &[Name], id: u32) -> String {
        names[id as usize].name.to_string()
    }

    pub fn size(&self) -> Result<usize> {
        let mut size = 24;
        Ok(match self {
            Property::ArrayProperty { array, .. } => {
                size += 4;
                for item in array {
                    size += item.size()?
                }
                size
            }
            Property::BoolProperty { .. } => size + 4,
            Property::ByteProperty { .. } => size + 1,
            Property::FloatProperty { .. } => size + 4,
            Property::IntProperty { .. } => size + 4,
            Property::NameProperty { .. } => size + 8,
            Property::ObjectProperty { .. } => size + 4,
            Property::StrProperty { string, .. } => {
                let mut bytes = Vec::new();
                string.serialize(&mut bytes)?;
                size + bytes.len()
            }
            Property::StringRefProperty { .. } => size + 4,
            Property::StructProperty { properties, .. } => size + properties.size()? + 8,
            Property::None { .. } => 8,
        })
    }
}

#[async_trait(?Send)]
impl SaveData for Property {
    fn deserialize(_: &mut SaveCursor) -> Result<Self> {
        unreachable!()
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        match self {
            Property::ArrayProperty { name_id, _osef1, type_id, _osef2, size, _osef3, array } => {
                serialize!(output, name_id, _osef1, type_id, _osef2, size, _osef3);

                let len = array.len() as u32;
                len.serialize(output)?;

                for item in array.iter() {
                    SaveData::serialize(item, output)?;
                }
            }
            Property::BoolProperty { name_id, _osef1, type_id, _osef2, size, _osef3, value } => {
                serialize!(output, name_id, _osef1, type_id, _osef2, size, _osef3, value)
            }
            Property::ByteProperty { name_id, _osef1, type_id, _osef2, size, _osef3, value } => {
                serialize!(output, name_id, _osef1, type_id, _osef2, size, _osef3, value)
            }
            Property::FloatProperty { name_id, _osef1, type_id, _osef2, size, _osef3, value } => {
                serialize!(output, name_id, _osef1, type_id, _osef2, size, _osef3, value)
            }
            Property::IntProperty { name_id, _osef1, type_id, _osef2, size, _osef3, value } => {
                serialize!(output, name_id, _osef1, type_id, _osef2, size, _osef3, value)
            }
            Property::NameProperty {
                name_id,
                _osef1,
                type_id,
                _osef2,
                size,
                _osef3,
                value_name_id,
                _osef4,
            } => serialize!(
                output,
                name_id,
                _osef1,
                type_id,
                _osef2,
                size,
                _osef3,
                value_name_id,
                _osef4
            ),
            Property::ObjectProperty {
                name_id,
                _osef1,
                type_id,
                _osef2,
                size,
                _osef3,
                object_id,
            } => {
                serialize!(output, name_id, _osef1, type_id, _osef2, size, _osef3, object_id)
            }
            Property::StrProperty { name_id, _osef1, type_id, _osef2, size, _osef3, string } => {
                serialize!(output, name_id, _osef1, type_id, _osef2, size, _osef3, string)
            }
            Property::StringRefProperty {
                name_id,
                _osef1,
                type_id,
                _osef2,
                size,
                _osef3,
                value,
            } => {
                serialize!(output, name_id, _osef1, type_id, _osef2, size, _osef3, value)
            }
            Property::StructProperty {
                name_id,
                _osef1,
                type_id,
                _osef2,
                size,
                _osef3,
                struct_name_id,
                _osef4,
                properties,
            } => serialize!(
                output,
                name_id,
                _osef1,
                type_id,
                _osef2,
                size,
                _osef3,
                struct_name_id,
                _osef4,
                properties
            ),
            Property::None { name_id, _osef } => serialize!(output, name_id, _osef),
        };
        Ok(())
    }

    async fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
}

#[derive(Clone)]
enum ArrayType {
    Int(i32),
    Object(u32),
    Vector(Vector),
    String(ImString),
    Properties(Vec<Property>),
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

        Ok(Self::Properties(properties))
    }

    fn size(&self) -> Result<usize> {
        Ok(match self {
            ArrayType::Int(_) => 4,
            ArrayType::Object(_) => 4,
            ArrayType::Vector(_) => size_of::<Vector>(),
            ArrayType::String(string) => {
                let mut bytes = Vec::new();
                string.serialize(&mut bytes)?;
                bytes.len()
            }
            ArrayType::Properties(properties) => {
                let mut size = 0;
                for property in properties {
                    size += property.size()?
                }
                size
            }
        })
    }
}

#[async_trait(?Send)]
impl SaveData for ArrayType {
    fn deserialize(_: &mut SaveCursor) -> Result<Self> {
        unreachable!()
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        match self {
            ArrayType::Int(value) => value.serialize(output)?,
            ArrayType::Object(export_id) => export_id.serialize(output)?,
            ArrayType::Vector(vector) => vector.serialize(output)?,
            ArrayType::String(string) => string.serialize(output)?,
            ArrayType::Properties(properties) => {
                for property in properties {
                    property.serialize(output)?;
                }
            }
        }
        Ok(())
    }

    async fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
}

#[derive(Clone)]
enum StructType {
    LinearColor(LinearColor),
    Vector(Vector),
    Rotator(Rotator),
    Properties(Vec<Property>),
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

        Ok(Self::Properties(properties))
    }

    fn size(&self) -> Result<usize> {
        Ok(match self {
            StructType::LinearColor(_) => size_of::<LinearColor>(),
            StructType::Vector(_) => size_of::<Vector>(),
            StructType::Rotator(_) => size_of::<Rotator>(),
            StructType::Properties(properties) => {
                let mut size = 0;
                for property in properties {
                    size += property.size()?
                }
                size
            }
        })
    }
}

#[async_trait(?Send)]
impl SaveData for StructType {
    fn deserialize(_: &mut SaveCursor) -> Result<Self> {
        unreachable!()
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        match self {
            StructType::LinearColor(linear_color) => linear_color.serialize(output)?,
            StructType::Vector(vector) => vector.serialize(output)?,
            StructType::Rotator(rotator) => rotator.serialize(output)?,
            StructType::Properties(properties) => {
                for property in properties {
                    property.serialize(output)?;
                }
            }
        }
        Ok(())
    }

    async fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
}
