// pub mod data;
// pub mod player;
pub mod plot_db;
// pub mod state;

// use self::{player::*, state::*};

// use std::fmt;
// use std::io::{Cursor, Read, Write};

// use anyhow::Result;
// use serde::{de, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
// use zip::{write::FileOptions, CompressionMethod, ZipArchive, ZipWriter};

// use super::{Dummy, List};
// use crate::unreal;

// #[derive(Clone)]
// pub struct Me1SaveGame {
//     _begin: Dummy<8>,
//     zip_offset: u32,
//     _no_mans_land: List<u8>,
//     pub player: Player,
//     pub state: State,
//     _world_save_package: Option<WorldSavePackage>,
// }

// impl Me1SaveGame {
//     fn unzip(input: &[u8]) -> Result<(Player, State, Option<WorldSavePackage>)> {
//         let mut zip = ZipArchive::new(Cursor::new(input))?;

//         let player: Player = {
//             let mut bytes = Vec::new();
//             zip.by_name("player.sav")?.read_to_end(&mut bytes)?;
//             unreal::Deserializer::from_bytes(&bytes)?
//         };

//         let state: State = {
//             let mut bytes = Vec::new();
//             zip.by_name("state.sav")?.read_to_end(&mut bytes)?;
//             unreal::Deserializer::from_bytes(&bytes)?
//         };

//         let _world_save_package: Option<WorldSavePackage> =
//             if zip.file_names().any(|f| f == "WorldSavePackage.sav") {
//                 Some({
//                     let mut bytes = Vec::new();
//                     zip.by_name("WorldSavePackage.sav")?.read_to_end(&mut bytes)?;
//                     unreal::Deserializer::from_bytes(&bytes)?
//                 })
//             } else {
//                 None
//             };

//         Ok((player, state, _world_save_package))
//     }

//     fn zip(&self) -> Result<List<u8>> {
//         let mut zip = Vec::new();
//         {
//             let mut zipper = ZipWriter::new(Cursor::new(&mut zip));
//             let options = FileOptions::default().compression_method(CompressionMethod::DEFLATE);

//             // Player
//             {
//                 let player_data = unreal::Serializer::to_byte_buf(&self.player)?;
//                 zipper.start_file("player.sav", options)?;
//                 zipper.write_all(&player_data)?;
//             }
//             // State
//             {
//                 let state_data = unreal::Serializer::to_byte_buf(&self.state)?;
//                 zipper.start_file("state.sav", options)?;
//                 zipper.write_all(&state_data)?;
//             }
//             // WorldSavePackage
//             if let Some(ref _world_save_package) = self._world_save_package {
//                 let world_save_package_data = unreal::Serializer::to_byte_buf(_world_save_package)?;
//                 zipper.start_file("WorldSavePackage.sav", options)?;
//                 zipper.write_all(&world_save_package_data)?;
//             }
//         }
//         Ok(zip.into())
//     }
// }

// impl<'de> Deserialize<'de> for Me1SaveGame {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         struct Me1SaveGameVisitor;
//         impl<'de> de::Visitor<'de> for Me1SaveGameVisitor {
//             type Value = Me1SaveGame;

//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("a Me1SaveGame")
//             }

//             fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
//             where
//                 A: de::SeqAccess<'de>,
//             {
//                 let _begin = seq.next_element()?.unwrap();
//                 let zip_offset = seq.next_element()?.unwrap();

//                 // No man's land
//                 let mut _no_mans_land = Vec::new();
//                 for _ in 0..(zip_offset - 12) {
//                     _no_mans_land.push(seq.next_element()?.unwrap());
//                 }

//                 let zip_data: List<u8> = seq.next_element()?.unwrap();
//                 let (player, state, _world_save_package) =
//                     Me1SaveGame::unzip(&zip_data).map_err(de::Error::custom)?;

//                 Ok(Me1SaveGame {
//                     _begin,
//                     zip_offset,
//                     _no_mans_land: _no_mans_land.into(),
//                     player,
//                     state,
//                     _world_save_package,
//                 })
//             }
//         }
//         deserializer.deserialize_tuple_struct("Me1SaveGame", usize::MAX, Me1SaveGameVisitor)
//     }
// }

// impl serde::Serialize for Me1SaveGame {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         use serde::ser::Error;
//         let Me1SaveGame {
//             _begin,
//             zip_offset,
//             _no_mans_land,
//             player: _,
//             state: _,
//             _world_save_package,
//         } = self;

//         let mut s = serializer.serialize_struct("Me1SaveGame", 4)?;
//         s.serialize_field("_begin", _begin)?;
//         s.serialize_field("zip_offset", zip_offset)?;
//         s.serialize_field("_no_mans_land", _no_mans_land)?;
//         s.serialize_field("zip", &self.zip().map_err(Error::custom)?)?;
//         s.end()
//     }
// }

// #[derive(Deserialize, Serialize, Clone)]
// pub(super) struct WorldSavePackage {
//     data: List<u8>,
// }

// #[cfg(test)]
// mod test {
//     use std::fs;
//     use std::time::Instant;

//     use anyhow::Result;

//     use super::*;
//     use crate::unreal;

//     #[test]
//     fn unzip_deserialize_serialize_zip() -> Result<()> {
//         let files = [
//             "test/ME1Save.MassEffectSave", // Normal save
//             "test/ME1Export.MassEffectSave", // Export save
//         ];

//         for file in files {
//             let input = fs::read(file)?;

//             let now = Instant::now();

//             // Deserialize
//             let me1_save_game: Me1SaveGame = unreal::Deserializer::from_bytes(&input)?;

//             println!("Deserialize 1 : {:?}", Instant::now() - now);
//             let now = Instant::now();

//             // Serialize
//             let output = unreal::Serializer::to_byte_buf(&me1_save_game)?;

//             println!("Serialize 1 : {:?}", Instant::now() - now);
//             let now = Instant::now();

//             // Deserialize (again)
//             let me1_save_game: Me1SaveGame = unreal::Deserializer::from_bytes(&output.clone())?;

//             println!("Deserialize 2 : {:?}", Instant::now() - now);
//             let now = Instant::now();

//             // Serialize (again)
//             let output_2 = unreal::Serializer::to_byte_buf(&me1_save_game)?;

//             println!("Serialize 2 : {:?}", Instant::now() - now);

//             // Check 2nd serialize = first serialize
//             // let cmp = output.chunks(4).zip(output_2.chunks(4));
//             // for (i, (a, b)) in cmp.enumerate() {
//             //     if a != b {
//             //         panic!("0x{:02x?} : {:02x?} != {:02x?}", i * 4, a, b);
//             //     }
//             // }

//             // Check 2nd serialize = first serialize
//             assert_eq!(output, output_2);
//         }
//         Ok(())
//     }
// }
