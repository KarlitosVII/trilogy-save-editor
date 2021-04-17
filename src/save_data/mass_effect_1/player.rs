use anyhow::*;
use async_trait::async_trait;
use imgui::{ImStr, ImString};
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

use crate::{gui::Gui, save_data::Dummy};

use super::{data::Data, SaveCursor, SaveData};

#[derive(Clone)]
pub struct Player {
    _begin: Dummy<8>,
    header_offset: u32,
    _no_mans_land1: Vec<u8>,
    header: Header,
    names: Vec<Name>,
    classes: Vec<Class>,
    pub objects: Vec<Object>,
    _no_mans_land2: Vec<u8>,
    datas: Vec<RefCell<Data>>,
}

impl Player {
    pub fn get_name(&self, id: u32) -> &ImStr {
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

#[async_trait(?Send)]
impl SaveData for Player {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let _begin = SaveData::deserialize(cursor)?;
        let header_offset = SaveData::deserialize(cursor)?;
        let _no_mans_land1 = cursor.read((header_offset - 12) as usize)?.to_owned();
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

        // Metadatas
        let mut objects: Vec<Object> = Vec::new();
        for _ in 0..header.objects_len {
            objects.push(SaveData::deserialize(cursor)?);
        }

        let _no_mans_land2 =
            cursor.read((header.data_offset - header.no_mans_land_offset) as usize)?.to_owned();

        // Data
        let mut datas = Vec::new();
        for object in objects.iter() {
            let data_bytes = cursor.read((object.data_size) as usize)?.to_owned();
            let mut cursor = SaveCursor::new(data_bytes);
            datas.push(RefCell::new(Data::new(&names, &mut cursor)?));
        }

        Ok(Self {
            _begin,
            header_offset,
            _no_mans_land1,
            header,
            names,
            classes,
            objects,
            _no_mans_land2,
            datas,
        })
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
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
            header.classes_offset += name.size()? as u32;
        }

        header.objects_offset = header.classes_offset + (classes.len() * 28) as u32;
        header.no_mans_land_offset = header.objects_offset + (objects.len() * 72) as u32;
        header.data_offset = header.no_mans_land_offset + _no_mans_land2.len() as u32;

        let mut objects = objects.clone();
        {
            let mut current_offset = header.data_offset;
            for (i, object) in objects.iter_mut().enumerate() {
                object.data_offset = current_offset;
                current_offset += datas[i].borrow().size()? as u32;
            }
        }

        // Serialize
        _begin.serialize(output)?;
        header_offset.serialize(output)?;
        output.extend(_no_mans_land1);
        header.serialize(output)?;

        for name in names.iter() {
            name.serialize(output)?;
        }

        for class in classes {
            class.serialize(output)?;
        }

        for object in objects {
            object.serialize(output)?;
        }

        output.extend(_no_mans_land2);

        for data in datas {
            data.borrow().serialize(output)?;
        }

        Ok(())
    }

    async fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
}

#[derive(SaveData, Clone)]
struct Header {
    _magic: u32,
    _version: Dummy<4>, // low_version: u16, high_version: u16
    data_offset: u32,
    _upk_name: ImString,
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

#[derive(SaveData, Clone)]
pub struct Name {
    string: ImString,
    _osef: Dummy<8>,
}

impl Name {
    fn size(&self) -> Result<usize> {
        let mut bytes = Vec::new();
        self.string.serialize(&mut bytes)?;
        Ok(bytes.len() + 8)
    }
}

impl Deref for Name {
    type Target = ImString;

    fn deref(&self) -> &ImString {
        &self.string
    }
}

impl DerefMut for Name {
    fn deref_mut(&mut self) -> &mut ImString {
        &mut self.string
    }
}

#[derive(SaveData, Clone)]
pub struct Class {
    package_id: u32,
    _osef1: Dummy<4>,
    base_name_id: u32,
    _osef2: Dummy<4>,
    link_id: u32,
    pub class_name_id: u32,
    _osef3: Dummy<4>,
}

#[derive(SaveData, Clone)]
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
    use anyhow::*;
    use std::{
        fs::File,
        io::{Cursor, Read},
    };
    use zip::ZipArchive;

    use crate::save_data::*;

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
        let mut output = Vec::new();
        Player::serialize(&player, &mut output)?;

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
