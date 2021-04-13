use anyhow::Result;
use async_trait::async_trait;
use imgui::ImString;
use std::mem::size_of;

use crate::{gui::Gui, save_data::Dummy};

use super::{
    export::{Data, Export},
    SaveCursor, SaveData,
};

#[derive(Clone)]
pub struct Player {
    _begin: Dummy<8>,
    header_offset: u32,
    _no_mans_land1: Vec<u8>,
    header: Header,
    names: Vec<Name>,
    imports: Vec<Import>,
    exports: Vec<Export>,
    _no_mans_land2: Vec<u8>,
    datas: Vec<Data>,
}

#[async_trait(?Send)]
impl SaveData for Player {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let _begin: Dummy<8> = SaveData::deserialize(cursor)?;
        let header_offset: u32 = SaveData::deserialize(cursor)?;
        let _no_mans_land1 = cursor.read((header_offset - 12) as usize)?.to_owned();
        let header: Header = SaveData::deserialize(cursor)?;

        // Names
        let mut names = Vec::new();
        for _ in 0..header.name_len {
            names.push(SaveData::deserialize(cursor)?);
        }

        // Imports
        let mut imports = Vec::new();
        for _ in 0..header.imports_len {
            imports.push(SaveData::deserialize(cursor)?);
        }

        // Exports
        let mut exports: Vec<Export> = Vec::new();
        for _ in 0..header.exports_len {
            exports.push(SaveData::deserialize(cursor)?);
        }

        let _no_mans_land2 =
            cursor.read((header.data_offset - header.no_mans_land_offset) as usize)?.to_owned();

        // Data
        let mut datas = Vec::new();
        for export in exports.iter() {
            let data_bytes = cursor.read((export.data_size) as usize)?.to_owned();
            let mut cursor = SaveCursor::new(data_bytes);
            datas.push(Data::new(&names, &mut cursor)?);
        }

        Ok(Self {
            _begin,
            header_offset,
            _no_mans_land1,
            header,
            names,
            imports,
            exports,
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
            imports,
            exports,
            _no_mans_land2,
            datas,
        } = self;

        // Calculs d'offsets
        let mut header = header.clone();

        header.imports_offset = header.name_offset;
        for name in names.iter() {
            header.imports_offset += name.size()? as u32;
        }

        header.exports_offset =
            header.imports_offset + (imports.len() * size_of::<Import>()) as u32;
        header.no_mans_land_offset =
            header.exports_offset + (exports.len() * size_of::<Export>()) as u32;
        header.data_offset = header.no_mans_land_offset + _no_mans_land2.len() as u32;

        let mut exports = exports.clone();
        {
            let mut current_offset = header.data_offset;
            for (i, export) in exports.iter_mut().enumerate() {
                export.data_offset = current_offset;
                current_offset += datas[i].size()? as u32;
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

        for import in imports {
            import.serialize(output)?;
        }

        for export in exports {
            export.serialize(output)?;
        }

        output.extend(_no_mans_land2);

        for data in datas {
            data.serialize(output)?;
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
    exports_len: u32,
    exports_offset: u32,
    imports_len: u32,
    imports_offset: u32,
    no_mans_land_offset: u32,
    _osef1: Dummy<68>,
    _compression: u32,
    _osef2: Dummy<12>,
}

#[derive(SaveData, Clone)]
pub struct Name {
    pub name: ImString,
    osef: Dummy<8>,
}

impl Name {
    fn size(&self) -> Result<usize> {
        let mut bytes = Vec::new();
        self.name.serialize(&mut bytes)?;
        Ok(bytes.len() + 8)
    }
}

#[derive(SaveData, Clone)]
struct Import {
    package_index: u32,
    _osef1: Dummy<4>,
    class_index: u32,
    _osef2: Dummy<4>,
    link_index: u32,
    object_index: u32,
    _osef3: Dummy<4>,
}

#[cfg(test)]
mod test {
    use anyhow::Result;
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
