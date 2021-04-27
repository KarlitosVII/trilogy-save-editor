use anyhow::Result;
use serde::Serialize;

use crate::{
    gui::Gui,
    save_data::{common::plot::Me1PlotTable, Dummy, ImguiString, List},
};

use super::{SaveCursor, SaveData};

#[derive(Serialize, Clone)]
pub struct State {
    _begin: Dummy<12>,
    base_level_name: ImguiString,
    _osef1: Dummy<24>,
    pub plot: Me1PlotTable,
    _osef2: List<u8>,
}

impl SaveData for State {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let _begin = SaveData::deserialize(cursor)?;
        let base_level_name = SaveData::deserialize(cursor)?;
        let _osef1 = SaveData::deserialize(cursor)?;
        let plot = SaveData::deserialize(cursor)?;
        let _osef2 = cursor.read_to_end()?.into();

        Ok(Self { _begin, base_level_name, _osef1, plot, _osef2 })
    }

    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        self.plot.draw_raw_ui(gui, ident);
    }
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

        let state_data = {
            let mut cursor = SaveCursor::new(input);
            let _: Dummy<8> = SaveData::deserialize(&mut cursor)?;
            let zip_offset: u32 = SaveData::deserialize(&mut cursor)?;
            let _ = cursor.read(zip_offset as usize - 12)?.to_owned();

            let zip_data = cursor.read_to_end()?;
            let mut zip = ZipArchive::new(Cursor::new(zip_data))?;

            let mut bytes = Vec::new();
            zip.by_name("state.sav")?.read_to_end(&mut bytes)?;
            bytes
        };

        // Deserialize
        let mut cursor = SaveCursor::new(state_data.clone());
        let state = State::deserialize(&mut cursor)?;

        // Serialize
        let output = unreal::Serializer::to_byte_buf(&state)?;

        // Check serialized = state_data
        let cmp = state_data.chunks(4).zip(output.chunks(4));
        for (i, (a, b)) in cmp.enumerate() {
            if a != b {
                panic!("0x{:02x?} : {:02x?} != {:02x?}", i * 4, a, b);
            }
        }

        Ok(())
    }
}
