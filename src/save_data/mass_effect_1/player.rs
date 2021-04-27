use anyhow::Result;
use serde::{ser::SerializeStruct, Serialize};
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

use crate::{
    gui::Gui,
    save_data::{Dummy, ImguiString, List},
    unreal,
};

use super::{data::Data, SaveCursor, SaveData};

#[derive(Clone)]
pub struct Player {
    _begin: Dummy<8>,
    header_offset: u32,
    _no_mans_land1: List<u8>,
    header: Header,
    names: List<Name>,
    classes: List<Class>,
    pub objects: List<Object>,
    _no_mans_land2: List<u8>,
    datas: List<RefCell<Data>>,
}

impl Player {
    pub fn get_name(&self, id: u32) -> &ImguiString {
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

impl SaveData for Player {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let _begin = SaveData::deserialize(cursor)?;
        let header_offset = SaveData::deserialize(cursor)?;
        let _no_mans_land1 = cursor.read((header_offset - 12) as usize)?.into();
        let header: Header = SaveData::deserialize(cursor)?;

        // Names
        let mut names = Vec::new();
        for _ in 0..header.name_len {
            names.push(SaveData::deserialize(cursor)?);
        }

        // Imports
        let mut classes = Vec::new();
        for _ in 0..header.classes_len {
            classes.push(SaveData::deserialize(cursor)?);
        }

        // Objects
        let mut objects: Vec<Object> = Vec::new();
        for _ in 0..header.objects_len {
            objects.push(SaveData::deserialize(cursor)?);
        }

        let _no_mans_land2 =
            cursor.read((header.data_offset - header.no_mans_land_offset) as usize)?.into();

        // Data
        let mut datas = Vec::new();
        for object in objects.iter() {
            let data_bytes = cursor.read((object.data_size) as usize)?.into();
            let mut cursor = SaveCursor::new(data_bytes);
            datas.push(RefCell::new(Data::new(&names, &mut cursor)?));
        }

        Ok(Self {
            _begin,
            header_offset,
            _no_mans_land1,
            header,
            names: names.into(),
            classes: classes.into(),
            objects: objects.into(),
            _no_mans_land2,
            datas: datas.into(),
        })
    }

    fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
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
        } = self;

        // Calculs d'offsets
        let mut header = header.clone();

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

#[derive(Serialize, SaveData, Clone)]
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

#[derive(Serialize, SaveData, Clone)]
pub struct Name {
    string: ImguiString,
    _osef: Dummy<8>,
}

impl Name {
    fn size(&self) -> Result<usize> {
        let bytes = unreal::Serializer::to_byte_buf(&self.string)?;
        Ok(bytes.len() + 8)
    }
}

impl Deref for Name {
    type Target = ImguiString;

    fn deref(&self) -> &ImguiString {
        &self.string
    }
}

impl DerefMut for Name {
    fn deref_mut(&mut self) -> &mut ImguiString {
        &mut self.string
    }
}

#[derive(Serialize, SaveData, Clone)]
pub struct Class {
    package_id: u32,
    _osef1: Dummy<4>,
    base_name_id: u32,
    _osef2: Dummy<4>,
    link_id: u32,
    pub class_name_id: u32,
    _osef3: Dummy<4>,
}

#[derive(Serialize, SaveData, Clone)]
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
        fs::File,
        io::{Cursor, Read},
    };
    use zip::ZipArchive;

    use crate::{save_data::*, unreal};

    use super::*;

    #[test]
    fn deserialize_serialize() -> Result<()> {
        let mut input = Vec::new();
        {
            let mut file = File::open("test/Clare00_AutoSave.MassEffectSave")?;
            file.read_to_end(&mut input)?;
        }

        let player_data = {
            let mut cursor = SaveCursor::new(input);
            let _: Dummy<8> = SaveData::deserialize(&mut cursor)?;
            let zip_offset: u32 = SaveData::deserialize(&mut cursor)?;
            let _ = cursor.read(zip_offset as usize - 12)?.to_owned();

            let zip_data = cursor.read_to_end()?;
            let mut zip = ZipArchive::new(Cursor::new(zip_data))?;

            let mut bytes = Vec::new();
            zip.by_name("player.sav")?.read_to_end(&mut bytes)?;
            bytes
        };

        // Deserialize
        let mut cursor = SaveCursor::new(player_data.clone());
        let player = Player::deserialize(&mut cursor)?;

        // Serialize
        let output = unreal::Serializer::to_byte_buf(&player)?;

        // Check serialized = player_data
        let cmp = player_data.chunks(4).zip(output.chunks(4));
        for (i, (a, b)) in cmp.enumerate() {
            if a != b {
                panic!("0x{:02x?} : {:02x?} != {:02x?}", i * 4, a, b);
            }
        }

        Ok(())
    }
}
