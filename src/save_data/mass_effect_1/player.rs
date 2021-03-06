use std::fmt;

use anyhow::Result;
use serde::de;
use serde::ser::SerializeTupleStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{data::Data, List};
use crate::save_data::Dummy;
use crate::save_data::RcRef;
use crate::unreal;

#[derive(Clone)]
pub struct Player {
    _begin: Dummy<8>,
    _header_offset: u32,
    _no_mans_land1: List<u8>,
    header: Header,
    pub names: RcRef<List<Name>>,
    classes: List<Class>,
    pub objects: List<Object>,
    _no_mans_land2: List<u8>,
    datas: List<Data>,
}

impl Player {
    pub fn get_name(&self, id: u32) -> String {
        self.names.borrow()[id as usize].string.borrow().clone()
    }

    pub fn get_class(&self, id: i32) -> &Class {
        &self.classes[id.abs() as usize - 1]
    }

    pub fn get_object(&self, id: i32) -> &Object {
        &self.objects[id as usize - 1]
    }

    pub fn get_data(&self, i: i32) -> &Data {
        &self.datas[i as usize - 1]
    }
}

impl<'de> Deserialize<'de> for Player {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PlayerVisitor;
        impl<'de> de::Visitor<'de> for PlayerVisitor {
            type Value = Player;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a Player")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let begin = seq.next_element()?.unwrap();
                let header_offset = seq.next_element()?.unwrap();

                // No man's land 1
                let mut no_mans_land1 = Vec::new();
                for _ in 0..(header_offset - 12) {
                    no_mans_land1.push(seq.next_element()?.unwrap());
                }

                let header: Header = seq.next_element()?.unwrap();

                // Names
                let mut names = Vec::new();
                for _ in 0..header.name_len {
                    names.push(seq.next_element()?.unwrap());
                }

                // Imports
                let mut classes = Vec::new();
                for _ in 0..header.classes_len {
                    classes.push(seq.next_element()?.unwrap());
                }

                // Objects
                let mut objects = Vec::new();
                for _ in 0..header.objects_len {
                    objects.push(seq.next_element()?.unwrap());
                }

                // No man's land 2
                let mut no_mans_land2 = Vec::new();
                for _ in 0..(header.data_offset - header.no_mans_land_offset) {
                    no_mans_land2.push(seq.next_element()?.unwrap());
                }

                // Data
                let mut datas = Vec::new();
                for _ in objects.iter() {
                    let data = Data::visit_seq(&names, &mut seq)?;
                    datas.push(data);
                }

                Ok(Player {
                    _begin: begin,
                    _header_offset: header_offset,
                    _no_mans_land1: no_mans_land1.into(),
                    header,
                    names: RcRef::new(names.into()),
                    classes: classes.into(),
                    objects: objects.into(),
                    _no_mans_land2: no_mans_land2.into(),
                    datas: datas.into(),
                })
            }
        }
        deserializer.deserialize_tuple_struct("Player", usize::MAX, PlayerVisitor)
    }
}

impl serde::Serialize for Player {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;
        let Player {
            _begin,
            _header_offset,
            _no_mans_land1,
            header,
            names,
            classes,
            objects,
            _no_mans_land2,
            datas,
        } = self;
        let names = names.borrow();

        // Calculs d'offsets
        let mut header = header.clone();

        header.name_len = names.len() as u32;
        header.classes_offset = header.name_offset;
        for name in names.iter() {
            header.classes_offset += name.size().map_err(Error::custom)? as u32;
        }

        header.objects_offset = header.classes_offset + (classes.len() * 28) as u32;
        header.no_mans_land_offset = header.objects_offset + (objects.len() * 72) as u32;
        header.data_offset = header.no_mans_land_offset + _no_mans_land2.0.len() as u32;

        let mut objects = objects.clone();
        {
            let mut current_offset = header.data_offset;
            for (i, object) in objects.iter_mut().enumerate() {
                object.data_offset = current_offset;
                let data_size = datas[i].size().map_err(Error::custom)? as u32;
                object.data_size = data_size;
                current_offset += data_size;
            }
        }

        // Serialize
        let mut s = serializer.serialize_tuple_struct("Player", 9)?;
        s.serialize_field(_begin)?;
        s.serialize_field(_header_offset)?;
        s.serialize_field(_no_mans_land1)?;
        s.serialize_field(&header)?;
        s.serialize_field(&*names)?;
        s.serialize_field(classes)?;
        s.serialize_field(&objects)?;
        s.serialize_field(_no_mans_land2)?;
        s.serialize_field(datas)?;
        s.end()
    }
}

#[derive(Deserialize, Serialize, Clone)]
struct Header {
    _magic: u32,
    _version: Dummy<4>, // low_version: u16, high_version: u16
    data_offset: u32,
    _upk_name: String,
    _flags: u32,
    name_len: u32,
    name_offset: u32,
    objects_len: u32,
    objects_offset: u32,
    classes_len: u32,
    classes_offset: u32,
    no_mans_land_offset: u32,
    _osef1: Dummy<68>,
    _compression: u32,
    _osef2: Dummy<12>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Name {
    pub string: RcRef<String>,
    _osef: Dummy<8>,
    #[serde(skip)]
    pub is_duplicate: bool, // Special
}

impl Name {
    fn size(&self) -> Result<usize> {
        let bytes = unreal::Serializer::to_vec(&self.string)?;
        Ok(bytes.len() + 8)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Class {
    package_id: u32,
    _osef1: Dummy<4>,
    base_name_id: u32,
    _osef2: Dummy<4>,
    link_id: u32,
    pub class_name_id: u32,
    _osef3: Dummy<4>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Object {
    pub class_id: i32,
    class_parent_id: u32,
    link_id: u32,
    pub object_name_id: u32,
    value_id: u32,
    archtype_id: u32,
    flag: u64,
    pub data_size: u32,
    pub data_offset: u32,
    _osef: Dummy<32>,
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::io::{Cursor, Read};

    use anyhow::Result;
    use zip::ZipArchive;

    use super::*;
    use crate::unreal;

    #[test]
    fn deserialize_serialize() -> Result<()> {
        let input = fs::read("test/ME1Save.MassEffectSave")?;

        let player_data = {
            let mut offset_bytes = [0; 4];
            offset_bytes.copy_from_slice(&input[8..12]);
            let zip_offset = <u32>::from_le_bytes(offset_bytes);
            let mut zip = ZipArchive::new(Cursor::new(&input[zip_offset as usize..]))?;

            let mut bytes = Vec::new();
            zip.by_name("player.sav")?.read_to_end(&mut bytes)?;
            bytes
        };

        // Deserialize
        let player: Player = unreal::Deserializer::from_bytes(&player_data)?;

        // Serialize
        let output = unreal::Serializer::to_vec(&player)?;

        // // Check serialized = player_data
        // let cmp = player_data.chunks(4).zip(output.chunks(4));
        // for (i, (a, b)) in cmp.enumerate() {
        //     if a != b {
        //         panic!("0x{:02x?} : {:02x?} != {:02x?}", i * 4, a, b);
        //     }
        // }

        // Check serialized = player_data
        assert!(player_data == output);

        Ok(())
    }
}
