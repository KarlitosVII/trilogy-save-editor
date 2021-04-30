use anyhow::Result;
use derive_more::{Deref, DerefMut};
use serde::{de, ser::SerializeStruct, Deserialize, Serialize};
use std::{cell::RefCell, fmt};

use crate::{
    save_data::{Dummy, ImguiString},
    unreal,
};

use super::{data::Data, List};

#[derive(Clone)]
pub struct Player {
    _begin: Dummy<8>,
    header_offset: u32,
    _no_mans_land1: List<u8>,
    header: Header,
    pub names: List<RefCell<Name>>,
    classes: List<Class>,
    pub objects: List<Object>,
    _no_mans_land2: List<u8>,
    datas: List<RefCell<Data>>,
    pub duplicate: RefCell<Option<Name>>, // Spécial n'est pas (dé)sérialisé
}

impl Player {
    pub fn get_name(&self, id: u32) -> &RefCell<Name> {
        &self.names[id as usize]
    }

    pub fn get_class(&self, id: i32) -> &Class {
        &self.classes[id.abs() as usize - 1]
    }

    pub fn get_object(&self, id: i32) -> &Object {
        &self.objects[id as usize - 1]
    }

    pub fn get_data(&self, i: i32) -> &RefCell<Data> {
        &self.datas[i as usize - 1]
    }
}

impl<'de> serde::Deserialize<'de> for Player {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PlayerVisitor;
        impl<'de> de::Visitor<'de> for PlayerVisitor {
            type Value = Player;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a seq")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let _begin = seq.next_element()?.unwrap();
                let header_offset = seq.next_element()?.unwrap();

                // No man's land 1
                let mut _no_mans_land1 = Vec::new();
                for _ in 0..(header_offset - 12) {
                    _no_mans_land1.push(seq.next_element()?.unwrap());
                }

                let header: Header = seq.next_element()?.unwrap();

                // Names
                let mut names = Vec::new();
                for _ in 0..header.name_len {
                    names.push(RefCell::new(seq.next_element()?.unwrap()));
                }

                // Imports
                let mut classes = Vec::new();
                for _ in 0..header.classes_len {
                    classes.push(seq.next_element()?.unwrap());
                }

                // Objects
                let mut objects: Vec<Object> = Vec::new();
                for _ in 0..header.objects_len {
                    objects.push(seq.next_element()?.unwrap());
                }

                // No man's land 2
                let mut _no_mans_land2 = Vec::new();
                for _ in 0..(header.data_offset - header.no_mans_land_offset) {
                    _no_mans_land2.push(seq.next_element()?.unwrap());
                }

                // Data
                let mut datas = Vec::new();
                for _ in objects.iter() {
                    let data = Data::visit_seq(&names, &mut seq)?;
                    datas.push(RefCell::new(data));
                }

                Ok(Player {
                    _begin,
                    header_offset,
                    _no_mans_land1: _no_mans_land1.into(),
                    header,
                    names: names.into(),
                    classes: classes.into(),
                    objects: objects.into(),
                    _no_mans_land2: _no_mans_land2.into(),
                    datas: datas.into(),
                    duplicate: RefCell::new(None),
                })
            }
        }
        deserializer.deserialize_tuple_struct("Player", usize::MAX, PlayerVisitor)
    }
}

impl serde::Serialize for Player {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::Error;
        let Player {
            _begin,
            header_offset,
            _no_mans_land1,
            header,
            names,
            classes,
            objects,
            _no_mans_land2,
            datas,
            duplicate: _,
        } = self;

        // Calculs d'offsets
        let mut header = header.clone();

        header.name_len = names.len() as u32;
        header.classes_offset = header.name_offset;
        for name in names.iter() {
            header.classes_offset += name.borrow().size().map_err(Error::custom)? as u32;
        }

        header.objects_offset = header.classes_offset + (classes.len() * 28) as u32;
        header.no_mans_land_offset = header.objects_offset + (objects.len() * 72) as u32;
        header.data_offset = header.no_mans_land_offset + _no_mans_land2.0.len() as u32;

        let mut objects = objects.clone();
        {
            let mut current_offset = header.data_offset;
            for (i, object) in objects.iter_mut().enumerate() {
                object.data_offset = current_offset;
                let data_size = datas[i].borrow().size().map_err(Error::custom)? as u32;
                object.data_size = data_size;
                current_offset += data_size;
            }
        }

        // Serialize
        let mut s = serializer.serialize_struct("Player", 9)?;
        s.serialize_field("_begin", _begin)?;
        s.serialize_field("header_offset", header_offset)?;
        s.serialize_field("_no_mans_land1", _no_mans_land1)?;
        s.serialize_field("header", &header)?;
        s.serialize_field("name", names)?;
        s.serialize_field("classes", classes)?;
        s.serialize_field("objects", &objects)?;
        s.serialize_field("_no_mans_land2", _no_mans_land2)?;
        s.serialize_field("objects", datas)?;
        s.end()
    }
}

#[derive(Deserialize, Serialize, Clone)]
struct Header {
    _magic: u32,
    _version: Dummy<4>, // low_version: u16, high_version: u16
    data_offset: u32,
    _upk_name: ImguiString,
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

#[derive(Deserialize, Serialize, RawUi, Deref, DerefMut, Clone)]
pub struct Name {
    #[deref]
    #[deref_mut]
    string: ImguiString,
    _osef: Dummy<8>,
    #[serde(skip)]
    pub is_duplicate: bool, // Spécial
}

impl Name {
    fn size(&self) -> Result<usize> {
        let bytes = unreal::Serializer::to_byte_buf(&self.string)?;
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
    use anyhow::Result;
    use std::{
        convert::TryInto,
        fs::File,
        io::{Cursor, Read},
    };
    use zip::ZipArchive;

    use crate::unreal;

    use super::*;

    #[test]
    fn deserialize_serialize() -> Result<()> {
        let mut input = Vec::new();
        {
            let mut file = File::open("test/Clare00_AutoSave.MassEffectSave")?;
            file.read_to_end(&mut input)?;
        }

        let player_data = {
            let zip_offset = <u32>::from_le_bytes((&input[8..12]).try_into()?);
            let mut zip = ZipArchive::new(Cursor::new(&input[zip_offset as usize..]))?;

            let mut bytes = Vec::new();
            zip.by_name("player.sav")?.read_to_end(&mut bytes)?;
            bytes
        };

        // Deserialize
        let player: Player = unreal::Deserializer::from_bytes(&player_data.clone())?;

        // Serialize
        let output = unreal::Serializer::to_byte_buf(&player)?;

        // // Check serialized = player_data
        // let cmp = player_data.chunks(4).zip(output.chunks(4));
        // for (i, (a, b)) in cmp.enumerate() {
        //     if a != b {
        //         panic!("0x{:02x?} : {:02x?} != {:02x?}", i * 4, a, b);
        //     }
        // }

        // Check serialized = player_data
        assert_eq!(player_data, output);

        Ok(())
    }
}
