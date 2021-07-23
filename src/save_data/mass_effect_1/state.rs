use anyhow::Result;
use serde::{de, Serialize};
use std::fmt;

use crate::{
    gui::Gui,
    save_data::{shared::plot::Me1PlotTable, Dummy, String, List, RawUi},
};

#[derive(Serialize, Clone)]
pub struct State {
    _begin: Dummy<12>,
    base_level_name: String,
    _osef1: Dummy<24>,
    pub plot: Me1PlotTable,
    _osef2: List<u8>,
}

impl RawUi for State {
    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        self.plot.draw_raw_ui(gui, ident);
    }
}

impl<'de> serde::Deserialize<'de> for State {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StateVisitor;
        impl<'de> de::Visitor<'de> for StateVisitor {
            type Value = State;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a State")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let _begin = seq.next_element()?.unwrap();
                let base_level_name = seq.next_element()?.unwrap();
                let _osef1 = seq.next_element()?.unwrap();
                let plot = seq.next_element()?.unwrap();
                let _osef2 = seq.next_element()?.unwrap();
                Ok(State { _begin, base_level_name, _osef1, plot, _osef2 })
            }
        }
        deserializer.deserialize_tuple_struct("State", 5, StateVisitor)
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use std::{
        convert::TryInto,
        fs,
        io::{Cursor, Read},
    };
    use zip::ZipArchive;

    use crate::unreal;

    use super::*;

    #[test]
    fn deserialize_serialize() -> Result<()> {
        let input = fs::read("test/Clare00_AutoSave.MassEffectSave")?;

        let state_data = {
            let zip_offset = <u32>::from_le_bytes((&input[8..12]).try_into()?);
            let mut zip = ZipArchive::new(Cursor::new(&input[zip_offset as usize..]))?;

            let mut bytes = Vec::new();
            zip.by_name("state.sav")?.read_to_end(&mut bytes)?;
            bytes
        };

        // Deserialize
        let state: State = unreal::Deserializer::from_bytes(&state_data.clone())?;

        // Serialize
        let output = unreal::Serializer::to_byte_buf(&state)?;

        // // Check serialized = state_data
        // let cmp = state_data.chunks(4).zip(output.chunks(4));
        // for (i, (a, b)) in cmp.enumerate() {
        //     if a != b {
        //         panic!("0x{:02x?} : {:02x?} != {:02x?}", i * 4, a, b);
        //     }
        // }

        assert_eq!(state_data, output);

        Ok(())
    }
}
