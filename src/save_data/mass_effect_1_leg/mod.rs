use anyhow::Result;
use flate2::{
    read::{ZlibDecoder, ZlibEncoder},
    Compression,
};
use serde::{
    de,
    ser::{self, SerializeStruct},
    Deserialize, Serialize,
};
use std::{fmt, io::Read};

use crate::unreal;

use super::{
    shared::{plot::Me1PlotTable, Rotator, SaveTimeStamp, Vector},
    Dummy, ImguiString, List,
};

pub mod player;
use self::player::*;

pub mod squad;
use self::squad::*;

#[derive(Serialize, Clone)]
struct ChunkHeader {
    compressed_size: u32,
    uncompressed_size: u32,
}

#[derive(Clone)]
pub struct Me1LegSaveGame {
    _magic_number: u32,
    _block_size: u32,
    _headers: List<ChunkHeader>,
    pub save_data: Me1LegSaveData,
    _checksum: u32,
    _unknown: Dummy<4>,
    _uncompressed_size: u32,
}

impl<'de> serde::Deserialize<'de> for Me1LegSaveGame {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Me1LegSaveGameVisitor;
        impl<'de> de::Visitor<'de> for Me1LegSaveGameVisitor {
            type Value = Me1LegSaveGame;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a seq")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let _magic_number = seq.next_element()?.unwrap();
                let _block_size = seq.next_element()?.unwrap();

                // Headers
                let mut headers = Vec::new();
                {
                    let full_header = ChunkHeader {
                        compressed_size: seq.next_element()?.unwrap(),
                        uncompressed_size: seq.next_element()?.unwrap(),
                    };
                    headers.push(full_header);

                    let mut finished = false;
                    while !finished {
                        let header = ChunkHeader {
                            compressed_size: seq.next_element()?.unwrap(),
                            uncompressed_size: seq.next_element()?.unwrap(),
                        };
                        if header.uncompressed_size < _block_size {
                            finished = true;
                        }
                        headers.push(header);
                    }
                }

                // Save data
                let save_data: Me1LegSaveData = {
                    let mut uncompressed = Vec::new();

                    for header in &headers[1..] {
                        let mut compressed = Vec::new();
                        for _ in 0..header.compressed_size {
                            compressed.push(seq.next_element()?.unwrap());
                        }

                        let mut z = ZlibDecoder::new(&compressed[..]);
                        z.read_to_end(&mut uncompressed).map_err(de::Error::custom)?;
                    }

                    unreal::Deserializer::from_bytes(&uncompressed).map_err(de::Error::custom)?
                };

                let _checksum = seq.next_element()?.unwrap();
                let _unknown = seq.next_element()?.unwrap();
                let _uncompressed_size = seq.next_element()?.unwrap();

                Ok(Me1LegSaveGame {
                    _magic_number,
                    _block_size,
                    _headers: headers.into(),
                    save_data,
                    _checksum,
                    _unknown,
                    _uncompressed_size,
                })
            }
        }
        deserializer.deserialize_tuple_struct("Me1LegSaveGame", usize::MAX, Me1LegSaveGameVisitor)
    }
}

impl serde::Serialize for Me1LegSaveGame {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let Me1LegSaveGame {
            _magic_number,
            _block_size,
            _headers,
            save_data,
            _checksum,
            _unknown,
            _uncompressed_size,
        } = self;

        let mut headers = Vec::new();

        let uncompressed =
            unreal::Serializer::to_byte_buf(save_data).map_err(ser::Error::custom)?;

        headers
            .push(ChunkHeader { compressed_size: 0, uncompressed_size: uncompressed.len() as u32 });

        // Compresse chaque chunk
        let mut compressed = Vec::new();
        for chunk in uncompressed.chunks(*_block_size as usize) {
            let uncompressed_size = chunk.len() as u32;

            let mut compressed_chunk = Vec::new();
            {
                let mut z = ZlibEncoder::new(chunk, Compression::default());
                z.read_to_end(&mut compressed_chunk).map_err(ser::Error::custom)?;
            }

            let compressed_size = compressed_chunk.len() as u32;

            headers[0].compressed_size += compressed_size;
            headers.push(ChunkHeader { compressed_size, uncompressed_size });

            compressed.extend(&compressed_chunk);
        }
        let headers: List<_> = headers.into();
        let save_data: List<u8> = compressed.into();

        let mut s = serializer.serialize_struct("Me1LegSaveGame", 4)?;
        s.serialize_field("_magic_number", _magic_number)?;
        s.serialize_field("_block_size", _block_size)?;
        s.serialize_field("_headers", &headers)?;
        s.serialize_field("save_data", &save_data)?;
        s.serialize_field("_checksum", _checksum)?;
        s.serialize_field("_unknown", _unknown)?;
        s.serialize_field("_uncompressed_size", &headers[0].uncompressed_size)?;
        s.end()
    }
}

#[derive(Deserialize, Serialize, RawUi, Clone)]
pub struct Me1LegSaveData {
    _version: Me1LegVersion,
    character_id: ImguiString,
    character_creation_date: SaveTimeStamp,
    pub plot: Me1PlotTable,
    _unknown2: Dummy<4>,
    _unknown3: Vec<Unknown3>,
    _unknown4: Vec<Dummy<4>>,
    _unknown5: Vec<Vec<Dummy<8>>>,
    _unknown6: Vec<Dummy<4>>,
    timestamp: SaveTimeStamp,
    seconds_played: i32,
    pub player: Player,
    _unknown7: Dummy<16>,
    pub difficulty: Difficulty,
    _unknown8: Dummy<177>,
    player_controller: PlayerController,
    map_name: ImguiString,
    maybe_sub_map_name_1: ImguiString,
    maybe_sub_map_name_2: ImguiString,
    location: Vector,
    rotation: Rotator,
    pub squad: Vec<Henchman>,
    // ---
    _remaining_bytes: List<u8>,
}

#[derive(Serialize, Clone)]
pub struct Me1LegVersion(i32);

impl<'de> serde::Deserialize<'de> for Me1LegVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let version: i32 = serde::Deserialize::deserialize(deserializer)?;

        if version != 50 {
            return Err(de::Error::custom(
                "Wrong save version, please use a save from the latest version of the game",
            ));
        }

        Ok(Self(version))
    }
}

#[derive(Deserialize, Serialize, Clone)]
struct Unknown3 {
    _unknown1: Dummy<8>,
    _unknown2: Vec<Dummy<4>>,
}

#[derive(RawUi, Clone)]
#[repr(u32)]
pub enum Difficulty {
    Casual,
    Normal,
    Veteran,
    Hardcore,
    Insanity,
}

impl<'de> serde::Deserialize<'de> for Difficulty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let idx: u32 = serde::Deserialize::deserialize(deserializer)?;

        let difficulty = match idx {
            0 => Difficulty::Casual,
            1 => Difficulty::Normal,
            2 => Difficulty::Veteran,
            3 => Difficulty::Hardcore,
            4 => Difficulty::Insanity,
            _ => return Err(de::Error::custom("invalid Difficulty variant")),
        };
        Ok(difficulty)
    }
}

impl serde::Serialize for Difficulty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.clone() as u32)
    }
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
struct PlayerController {
    action_map: ImguiString,
    _unknown: Dummy<4>,
    hotkeys: Vec<Hotkey>,
    current_weapon: ImguiString,
    last_weapon: ImguiString,
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
struct Hotkey {
    pawn: i32,
    event: i32,
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use crc::{Crc, CRC_32_BZIP2};
    use std::{fs::File, time::Instant};

    use crate::unreal;

    use super::*;

    #[test]
    fn deserialize_serialize() -> Result<()> {
        let mut input = Vec::new();
        {
            let mut file = File::open("test/ME1Leg00_QuickSave.pcsav")?;
            file.read_to_end(&mut input)?;
        }

        let now = Instant::now();

        // Deserialize
        let me1_save_game: Me1LegSaveGame = unreal::Deserializer::from_bytes(&input)?;

        println!("Deserialize 1 : {:?}", Instant::now().saturating_duration_since(now));
        let now = Instant::now();

        // Serialize
        let mut output = unreal::Serializer::to_byte_buf(&me1_save_game)?;

        // Checksum
        {
            let checksum_offset = output.len() - 12;
            let crc = Crc::<u32>::new(&CRC_32_BZIP2);
            let checksum = crc.checksum(&output[..checksum_offset]);

            // Update checksum
            let end = checksum_offset + 4;
            output[checksum_offset..end].swap_with_slice(&mut u32::to_le_bytes(checksum));
        }

        println!("Serialize 1 : {:?}", Instant::now().saturating_duration_since(now));
        let now = Instant::now();

        // Deserialize (again)
        let me1_save_game: Me1LegSaveGame = unreal::Deserializer::from_bytes(&output.clone())?;

        println!("Deserialize 2 : {:?}", Instant::now().saturating_duration_since(now));
        let now = Instant::now();

        // Serialize (again)
        let mut output_2 = unreal::Serializer::to_byte_buf(&me1_save_game)?;

        // Checksum
        {
            let checksum_offset = output_2.len() - 12;
            let crc = Crc::<u32>::new(&CRC_32_BZIP2);
            let checksum = crc.checksum(&output_2[..checksum_offset]);

            // Update checksum
            let end = checksum_offset + 4;
            output_2[checksum_offset..end].swap_with_slice(&mut u32::to_le_bytes(checksum));
        }

        println!("Serialize 2 : {:?}", Instant::now().saturating_duration_since(now));

        // Check 2nd serialize = first serialize
        // let cmp = output.chunks(4).zip(output_2.chunks(4));
        // for (i, (a, b)) in cmp.enumerate() {
        //     if a != b {
        //         panic!("0x{:02x?} : {:02x?} != {:02x?}", i * 4, a, b);
        //     }
        // }

        // Check 2nd serialize = first serialize
        assert_eq!(output, output_2);
        Ok(())
    }

    // #[test]
    // fn uncompress() -> Result<()> {
    //     let mut input = Vec::new();
    //     {
    //         let mut file = File::open("test/Clare00_QuickSave.pcsav")?;
    //         file.read_to_end(&mut input)?;
    //     }

    //     let me1_save_game: Me1LegSaveGame = unreal::Deserializer::from_bytes(&input)?;

    //     let output = unreal::Serializer::to_byte_buf(&me1_save_game.save_data)?;
    //     {
    //         let mut file = File::create("test/Clare00_QuickSave.uncompressed")?;
    //         use std::io::Write;
    //         file.write_all(&output)?;
    //     }

    //     Ok(())
    // }
}
