use anyhow::Result;
use crc::{Crc, CRC_32_BZIP2};
use flume::{Receiver, Sender};
use ron::ser::PrettyConfig;
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::{
    gui::UiEvent,
    save_data::{
        mass_effect_1::Me1SaveGame,
        mass_effect_1_le::Me1LeSaveGame,
        mass_effect_2::{Me2LeSaveGame, Me2LeVersion, Me2SaveGame, Me2Version},
        mass_effect_3::Me3SaveGame,
        shared::appearance::HeadMorph,
    },
    unreal,
};

pub enum MainEvent {
    OpenSave(String),
    SaveSave(String, SaveGame),
    ImportHeadMorph(String),
    ExportHeadMorph(String, Box<HeadMorph>),
}

#[derive(Clone)]
pub enum SaveGame {
    MassEffect1 { file_path: String, save_game: Box<Me1SaveGame> },
    MassEffect1Le { file_path: String, save_game: Box<Me1LeSaveGame> },
    MassEffect2 { file_path: String, save_game: Box<Me2SaveGame> },
    MassEffect2Le { file_path: String, save_game: Box<Me2LeSaveGame> },
    MassEffect3 { file_path: String, save_game: Box<Me3SaveGame> },
}

pub async fn event_loop(rx: Receiver<MainEvent>, ui_addr: Sender<UiEvent>) {
    while let Ok(event) = rx.recv_async().await {
        let result = async {
            let ui_addr = Sender::clone(&ui_addr);
            match event {
                MainEvent::OpenSave(path) => tokio::spawn(open_save(path, ui_addr)).await?,
                MainEvent::SaveSave(path, save_game) => {
                    tokio::spawn(save_save(path, save_game, ui_addr)).await?
                }
                MainEvent::ImportHeadMorph(path) => {
                    tokio::spawn(import_head_morph(path, ui_addr)).await?
                }
                MainEvent::ExportHeadMorph(path, head_morph) => {
                    tokio::spawn(export_head_morph(path, head_morph, ui_addr)).await?
                }
            }
        };

        if let Err(err) = result.await {
            let _ = ui_addr.send_async(UiEvent::Error(err)).await;
        }
    }
}

async fn open_save(file_path: String, ui_addr: Sender<UiEvent>) -> Result<()> {
    if let Some(ext) = Path::new(&file_path).extension() {
        let input = fs::read(&file_path).await?;

        let save_game = if unicase::eq(ext.to_string_lossy().to_string().as_str(), "MassEffectSave")
        {
            // ME1
            SaveGame::MassEffect1 {
                file_path,
                save_game: Box::new(unreal::Deserializer::from_bytes(&input)?),
            }
        } else if input[0..4] == [0xC1, 0x83, 0x2A, 0x9E] {
            // ME1 Legendary
            SaveGame::MassEffect1Le {
                file_path,
                save_game: Box::new(unreal::Deserializer::from_bytes(&input)?),
            }
        } else if unreal::Deserializer::from_bytes::<Me2Version>(&input).is_ok() {
            // ME2
            SaveGame::MassEffect2 {
                file_path,
                save_game: Box::new(unreal::Deserializer::from_bytes(&input)?),
            }
        } else if unreal::Deserializer::from_bytes::<Me2LeVersion>(&input).is_ok() {
            // ME2 Legendary
            SaveGame::MassEffect2Le {
                file_path,
                save_game: Box::new(unreal::Deserializer::from_bytes(&input)?),
            }
        } else {
            // ME3
            SaveGame::MassEffect3 {
                file_path,
                save_game: Box::new(unreal::Deserializer::from_bytes(&input)?),
            }
        };

        let _ = ui_addr.send_async(UiEvent::OpenedSave(save_game)).await;
        let _ = ui_addr.send_async(UiEvent::Notification("Opened")).await;
    }

    Ok(())
}

async fn save_save(path: String, save_game: SaveGame, ui_addr: Sender<UiEvent>) -> Result<()> {
    let output = match save_game {
        SaveGame::MassEffect1 { save_game, .. } => unreal::Serializer::to_byte_buf(&save_game)?,
        SaveGame::MassEffect1Le { save_game, .. } => {
            let mut output = unreal::Serializer::to_byte_buf(&save_game)?;

            // Checksum
            let checksum_offset = output.len() - 12;
            let crc = Crc::<u32>::new(&CRC_32_BZIP2);
            let checksum = crc.checksum(&output[..checksum_offset]);

            // Update checksum
            let end = checksum_offset + 4;
            output[checksum_offset..end].swap_with_slice(&mut u32::to_le_bytes(checksum));
            output
        }
        SaveGame::MassEffect2 { save_game, .. } => {
            let mut output = unreal::Serializer::to_byte_buf(&save_game)?;

            let crc = Crc::<u32>::new(&CRC_32_BZIP2);
            let checksum = crc.checksum(&output);
            output.extend(&u32::to_le_bytes(checksum));
            output
        }
        SaveGame::MassEffect2Le { save_game, .. } => {
            let mut output = unreal::Serializer::to_byte_buf(&save_game)?;

            let crc = Crc::<u32>::new(&CRC_32_BZIP2);
            let checksum = crc.checksum(&output);
            output.extend(&u32::to_le_bytes(checksum));
            output
        }
        SaveGame::MassEffect3 { save_game, .. } => {
            let mut output = unreal::Serializer::to_byte_buf(&save_game)?;

            let crc = Crc::<u32>::new(&CRC_32_BZIP2);
            let checksum = crc.checksum(&output);
            output.extend(&u32::to_le_bytes(checksum));
            output
        }
    };

    // Backup si fichier existe
    let path = PathBuf::from(path);
    if fs::metadata(&path).await.is_ok() {
        if let Some(ext) = path.extension() {
            let to = Path::with_extension(&path, ext.to_string_lossy().into_owned() + ".bak");
            fs::copy(&path, to).await?;
        }
    }

    fs::write(&path, &output).await?;

    let _ = ui_addr.send_async(UiEvent::Notification("Saved")).await;
    Ok(())
}

async fn import_head_morph(path: String, ui_addr: Sender<UiEvent>) -> Result<()> {
    let import = fs::read_to_string(&path).await?;
    let head_morph: HeadMorph = ron::from_str(&import)?;

    let _ = ui_addr.send_async(UiEvent::ImportedHeadMorph(Box::new(head_morph))).await;
    let _ = ui_addr.send_async(UiEvent::Notification("Imported")).await;
    Ok(())
}

async fn export_head_morph(
    path: String, head_morph: Box<HeadMorph>, ui_addr: Sender<UiEvent>,
) -> Result<()> {
    let pretty_config =
        PrettyConfig::new().with_enumerate_arrays(true).with_new_line(String::from('\n'));

    let export = ron::ser::to_string_pretty(&head_morph, pretty_config)?;
    fs::write(&path, export.as_bytes()).await?;

    let _ = ui_addr.send_async(UiEvent::Notification("Exported")).await;
    Ok(())
}
