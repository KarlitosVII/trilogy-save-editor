use anyhow::Result;
use serde::{de, Serialize};

use super::{player::Name, List};
use crate::{
    save_data::{
        shared::{appearance::LinearColor, Rotator, Vector},
        Dummy,
    },
    save_data::{RcCell, RcRef},
    unreal,
};

#[derive(Serialize, Clone)]
pub struct Data {
    _osef: Dummy<4>,
    pub properties: List<RcRef<Property>>,
}

impl Data {
    pub fn visit_seq<'de, A>(names: &[Name], seq: &mut A) -> Result<Self, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let _osef = seq.next_element()?.unwrap();
        let properties = List::<RcRef<Property>>::visit_seq(names, seq)?;
        Ok(Self { _osef, properties })
    }

    pub fn size(&self) -> Result<usize> {
        let mut size = 4;
        for property in self.properties.iter() {
            size += property.borrow().size()?
        }
        Ok(size)
    }
}

fn get_name(names: &[Name], id: u32) -> String {
    names[id as usize].string.borrow().clone()
}

impl List<RcRef<Property>> {
    pub fn visit_seq<'de, A>(names: &[Name], seq: &mut A) -> Result<Self, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut properties = Vec::new();

        let mut finished = false;
        while !finished {
            let property = Property::visit_seq(names, seq)?;

            // Ça se termine toujours par un None donc on break ici
            if let Property::None { .. } = property {
                finished = true;
            }
            properties.push(property.into());
        }

        Ok(properties.into())
    }
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
        value: RcCell<bool>,
    },
    Byte {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        value: RcCell<u8>,
    },
    Float {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        value: RcCell<f32>,
    },
    Int {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        value: RcCell<i32>,
    },
    Name {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        value_name_id: RcCell<u32>,
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
        string: RcRef<String>,
    },
    StringRef {
        name_id: u32,
        _osef1: Dummy<4>,
        type_id: u32,
        _osef2: Dummy<4>,
        size: u32,
        _osef3: Dummy<4>,
        value: RcCell<i32>,
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
        struct_type: StructType,
    },
    None {
        name_id: u32,
        _osef: Dummy<4>,
    },
}

impl Property {
    pub fn visit_seq<'de, A>(names: &[Name], seq: &mut A) -> Result<Self, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        // Name
        let name_id = seq.next_element()?.unwrap();
        let _osef1 = seq.next_element()?.unwrap();

        let name = get_name(names, name_id);
        if name == "None" {
            return Ok(Property::None { name_id, _osef: _osef1 });
        }

        // Type
        let type_id = seq.next_element()?.unwrap();
        let _osef2 = seq.next_element()?.unwrap();
        // Size
        let size = seq.next_element()?.unwrap();
        let _osef3 = seq.next_element()?.unwrap();

        let type_name = get_name(names, type_id);
        let property = match type_name.as_str() {
            "ArrayProperty" => {
                let len: u32 = seq.next_element()?.unwrap();
                let mut array = Vec::new();
                // Hardcodé sinon je dois chercher dans toutes les classes du jeu...
                match name.as_str() {
                    "m_PrereqTalentIDArray" | "m_PrereqTalentRankArray" => {
                        for _ in 0..len {
                            let array_int = ArrayType::Int(seq.next_element()?.unwrap());
                            array.push(array_int);
                        }
                    }
                    "m_aItem"
                    | "m_aXMod"
                    | "m_aEquipped"
                    | "m_QuickSlotArray"
                    | "m_savedBuybackItems" => {
                        for _ in 0..len {
                            let array_object = ArrayType::Object(seq.next_element()?.unwrap());
                            array.push(array_object);
                        }
                    }
                    "m_vPosition" => {
                        for _ in 0..len {
                            let array_vector = ArrayType::Vector(seq.next_element()?.unwrap());
                            array.push(array_vector);
                        }
                    }
                    "m_DependentPackages" => {
                        for _ in 0..len {
                            let array_string = ArrayType::String(seq.next_element()?.unwrap());
                            array.push(array_string);
                        }
                    }
                    _ => {
                        for _ in 0..len {
                            let array_properties = ArrayType::Properties(
                                List::<RcRef<Property>>::visit_seq(names, seq)?,
                            );
                            array.push(array_properties);
                        }
                    }
                }
                Property::Array { name_id, _osef1, type_id, _osef2, size, _osef3, array }
            }
            "BoolProperty" => {
                let value = seq.next_element()?.unwrap();
                Property::Bool { name_id, _osef1, type_id, _osef2, size, _osef3, value }
            }
            "ByteProperty" => {
                if size == 1 {
                    let value = seq.next_element()?.unwrap();
                    Property::Byte { name_id, _osef1, type_id, _osef2, size, _osef3, value }
                } else {
                    let value_name_id = seq.next_element()?.unwrap();
                    let _osef4 = seq.next_element()?.unwrap();
                    Property::Name {
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
                let value = seq.next_element()?.unwrap();
                Property::Float { name_id, _osef1, type_id, _osef2, size, _osef3, value }
            }
            "IntProperty" => {
                let value = seq.next_element()?.unwrap();
                Property::Int { name_id, _osef1, type_id, _osef2, size, _osef3, value }
            }
            "NameProperty" => {
                let value_name_id = seq.next_element()?.unwrap();
                let _osef4 = seq.next_element()?.unwrap();
                Property::Name {
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
                let object_id = seq.next_element()?.unwrap();
                Property::Object { name_id, _osef1, type_id, _osef2, size, _osef3, object_id }
            }
            "StrProperty" => {
                let string = seq.next_element()?.unwrap();
                Property::Str { name_id, _osef1, type_id, _osef2, size, _osef3, string }
            }
            "StringRefProperty" => {
                let value = seq.next_element()?.unwrap();
                Property::StringRef { name_id, _osef1, type_id, _osef2, size, _osef3, value }
            }
            "StructProperty" => {
                let struct_name_id = seq.next_element()?.unwrap();
                let _osef4 = seq.next_element()?.unwrap();

                let struct_name = get_name(names, struct_name_id);
                let struct_type = match struct_name.as_str() {
                    "LinearColor" => StructType::LinearColor(seq.next_element()?.unwrap()),
                    "Vector" => StructType::Vector(seq.next_element()?.unwrap()),
                    "Rotator" => StructType::Rotator(seq.next_element()?.unwrap()),
                    _ => StructType::Properties(List::<RcRef<Property>>::visit_seq(names, seq)?),
                };
                Property::Struct {
                    name_id,
                    _osef1,
                    type_id,
                    _osef2,
                    size,
                    _osef3,
                    struct_name_id,
                    _osef4,
                    struct_type,
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
                let bytes = unreal::Serializer::to_vec(string)?;
                size + bytes.len()
            }
            Property::StringRef { .. } => size + 4,
            Property::Struct { struct_type, .. } => size + struct_type.size()? + 8,
            Property::None { .. } => 8,
        })
    }
}

#[derive(Serialize, Clone)]
pub enum ArrayType {
    Int(RcCell<i32>),
    Object(i32),
    Vector(RcRef<Vector>),
    String(RcRef<String>),
    Properties(List<RcRef<Property>>),
}

impl ArrayType {
    fn size(&self) -> Result<usize> {
        Ok(match self {
            ArrayType::Int(_) => 4,
            ArrayType::Object(_) => 4,
            ArrayType::Vector(_) => 12,
            ArrayType::String(string) => {
                let bytes = unreal::Serializer::to_vec(string)?;
                bytes.len()
            }
            ArrayType::Properties(properties) => {
                let mut size = 0;
                for property in properties.iter() {
                    size += property.borrow().size()?
                }
                size
            }
        })
    }
}

#[derive(Serialize, Clone)]
pub enum StructType {
    LinearColor(RcRef<LinearColor>),
    Vector(RcRef<Vector>),
    Rotator(RcRef<Rotator>),
    Properties(List<RcRef<Property>>),
}

impl StructType {
    fn size(&self) -> Result<usize> {
        Ok(match self {
            StructType::LinearColor(_) => 16,
            StructType::Vector(_) => 12,
            StructType::Rotator(_) => 12,
            StructType::Properties(properties) => {
                let mut size = 0;
                for property in properties.iter() {
                    size += property.borrow().size()?
                }
                size
            }
        })
    }
}
