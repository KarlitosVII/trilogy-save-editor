use serde::{Deserialize, Serialize};

use crate::save_data::{shared::plot::PlotTable, Dummy, List};

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUi)]
pub struct State {
    _begin: Dummy<12>,
    base_level_name: String,
    _osef1: Dummy<24>,
    pub plot: PlotTable,
    _osef2: List<u8>,
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

        let state_data = {
            let mut offset_bytes = [0; 4];
            offset_bytes.copy_from_slice(&input[8..12]);
            let zip_offset = <u32>::from_le_bytes(offset_bytes);
            let mut zip = ZipArchive::new(Cursor::new(&input[zip_offset as usize..]))?;

            let mut bytes = Vec::new();
            zip.by_name("state.sav")?.read_to_end(&mut bytes)?;
            bytes
        };

        // Deserialize
        let state: State = unreal::Deserializer::from_bytes(&state_data)?;

        // Serialize
        let output = unreal::Serializer::to_vec(&state)?;

        // // Check serialized = state_data
        // let cmp = state_data.chunks(4).zip(output.chunks(4));
        // for (i, (a, b)) in cmp.enumerate() {
        //     if a != b {
        //         panic!("0x{:02x?} : {:02x?} != {:02x?}", i * 4, a, b);
        //     }
        // }

        assert!(state_data == output);

        Ok(())
    }
}
