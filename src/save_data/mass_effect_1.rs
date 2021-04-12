use anyhow::Result;
use async_trait::async_trait;
use std::io::{Cursor, Read, Write};
use zip::{write::FileOptions, CompressionMethod, ZipArchive, ZipWriter};

use crate::{gui::Gui, save_data::Dummy};

use super::{SaveCursor, SaveData};

mod player;
use self::player::*;

#[derive(Clone)]
pub struct Me1SaveGame {
    begin: Dummy<8>,
    zip_offset: u32,
    no_mans_land: Vec<u8>,
    player: Player,
    state: State,
    world_save_package: WorldSavePackage,
}

#[async_trait(?Send)]
impl SaveData for Me1SaveGame {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let begin: Dummy<8> = SaveData::deserialize(cursor)?;
        let zip_offset: u32 = SaveData::deserialize(cursor)?;
        let no_mans_land = cursor.read(zip_offset as usize - 12)?.to_owned();

        let zip_data = cursor.read_to_end()?;
        let mut zip = ZipArchive::new(Cursor::new(zip_data))?;

        let player: Player = {
            let mut bytes = Vec::new();
            zip.by_name("player.sav")?.read_to_end(&mut bytes)?;
            let mut cursor = SaveCursor::new(bytes);
            SaveData::deserialize(&mut cursor)?
        };

        let state: State = {
            let mut bytes = Vec::new();
            zip.by_name("state.sav")?.read_to_end(&mut bytes)?;
            let mut cursor = SaveCursor::new(bytes);
            SaveData::deserialize(&mut cursor)?
        };

        let world_save_package: WorldSavePackage = {
            let mut bytes = Vec::new();
            zip.by_name("WorldSavePackage.sav")?.read_to_end(&mut bytes)?;
            let mut cursor = SaveCursor::new(bytes);
            SaveData::deserialize(&mut cursor)?
        };

        Ok(Self { begin, zip_offset, no_mans_land, player, state, world_save_package })
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        let Me1SaveGame { begin, zip_offset, no_mans_land, player, state, world_save_package } =
            &self;

        begin.serialize(output)?;
        zip_offset.serialize(output)?;
        output.extend(no_mans_land);

        let mut zip = Vec::new();
        {
            let mut zipper = ZipWriter::new(Cursor::new(&mut zip));
            let options = FileOptions::default().compression_method(CompressionMethod::DEFLATE);

            // Player
            {
                let mut player_data = Vec::new();
                player.serialize(&mut player_data)?;
                zipper.start_file("player.sav", options)?;
                zipper.write_all(&player_data)?;
            }

            // State
            {
                let mut state_data = Vec::new();
                state.serialize(&mut state_data)?;
                zipper.start_file("state.sav", options)?;
                zipper.write_all(&state_data)?;
            }

            // WorldSavePackage
            {
                let mut world_save_package_data = Vec::new();
                world_save_package.serialize(&mut world_save_package_data)?;
                zipper.start_file("WorldSavePackage.sav", options)?;
                zipper.write_all(&world_save_package_data)?;
            }
        }
        output.extend(&zip);

        Ok(())
    }

    async fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
}

macro_rules! zip_file_save_data {
    ($type:ident) => {
        #[derive(Clone)]
        pub(super) struct $type {
            data: Vec<u8>,
        }

        #[async_trait(?Send)]
        impl SaveData for $type {
            fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
                Ok(Self { data: cursor.read_to_end()?.to_owned() })
            }

            fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
                Ok(output.extend(&self.data))
            }

            async fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
        }
    };
}
zip_file_save_data!(State);
zip_file_save_data!(WorldSavePackage);

#[cfg(test)]
mod test {
    use anyhow::Result;
    use tokio::{fs::File, io::AsyncReadExt};

    use crate::save_data::*;

    use super::*;

    #[tokio::test]
    async fn test_deserialize_serialize() -> Result<()> {
        let mut input = Vec::new();
        {
            let mut file = File::open("test/Clare00_AutoSave.MassEffectSave").await?;
            file.read_to_end(&mut input).await?;
        }

        // Deserialize
        let mut cursor = SaveCursor::new(input);
        let me1_save_game = Me1SaveGame::deserialize(&mut cursor)?;

        // Serialize
        let mut output = Vec::new();
        Me1SaveGame::serialize(&me1_save_game, &mut output)?;

        // Deserialize (again)
        let mut cursor = SaveCursor::new(output.clone());
        let me1_save_game = Me1SaveGame::deserialize(&mut cursor)?;

        // Serialize (again)
        let mut output_2 = Vec::new();
        Me1SaveGame::serialize(&me1_save_game, &mut output_2)?;

        // Check 2nd serialize = first serialize
        assert_eq!(&output_2, &output);

        Ok(())
    }
}
