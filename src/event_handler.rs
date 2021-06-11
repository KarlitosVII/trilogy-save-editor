use anyhow::{Context, Result};
use crc::{Crc, CRC_32_BZIP2};
use flume::{Receiver, Sender};
use ron::ser::PrettyConfig;
use std::path::{Path, PathBuf};
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{
    gui::UiEvent,
    save_data::{
        mass_effect_1::{item_db::Me1ItemDb, plot_db::Me1PlotDb, Me1SaveGame},
        mass_effect_1_leg::Me1LegSaveGame,
        mass_effect_2::{
            plot_db::Me2PlotDb, Me2LegSaveGame, Me2LegVersion, Me2SaveGame, Me2Version,
        },
        mass_effect_3::{plot_db::Me3PlotDb, Me3SaveGame},
        shared::appearance::HeadMorph,
    },
    unreal,
};

pub enum MainEvent {
    OpenSave(String),
    SaveSave(String, SaveGame),
    LoadDatabases,
    ImportHeadMorph(String),
    ExportHeadMorph(String, Box<HeadMorph>),
}

#[derive(Clone)]
pub enum SaveGame {
    MassEffect1 { file_path: String, save_game: Box<Me1SaveGame> },
    MassEffect1Leg { file_path: String, save_game: Box<Me1LegSaveGame> },
    MassEffect2 { file_path: String, save_game: Box<Me2SaveGame> },
    MassEffect2Leg { file_path: String, save_game: Box<Me2LegSaveGame> },
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
                MainEvent::LoadDatabases => {
                    let me1_plot_handle = tokio::spawn(load_me1_plot_db(Sender::clone(&ui_addr)));
                    let me1_item_handle = tokio::spawn(load_me1_item_db(Sender::clone(&ui_addr)));
                    let me2_handle = tokio::spawn(load_me2_plot_db(Sender::clone(&ui_addr)));
                    let me3_handle = tokio::spawn(load_me3_plot_db(ui_addr));

                    let (me1_plot_result, me1_item_result, me2_result, me3_result) =
                        tokio::join!(me1_plot_handle, me1_item_handle, me2_handle, me3_handle);

                    me1_plot_result?.context("Failed to parse databases/me1_plot_db.ron")?;
                    me1_item_result?.context("Failed to parse databases/me1_item_db.ron")?;
                    me2_result?.context("Failed to parse databases/me2_plot_db.ron")?;
                    me3_result?.context("Failed to parse databases/me3_plot_db.ron")
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
    let mut input = Vec::new();
    {
        let mut file = File::open(&file_path).await?;
        file.read_to_end(&mut input).await?;
    }

    if let Some(ext) = Path::new(&file_path).extension() {
        let save_game = if unicase::eq(ext.to_string_lossy().to_string().as_str(), "MassEffectSave")
        {
            // ME1
            SaveGame::MassEffect1 {
                file_path,
                save_game: Box::new(unreal::Deserializer::from_bytes(&input)?),
            }
        } else if input[0..4] == [0xC1, 0x83, 0x2A, 0x9E] {
            // ME1 Legendary
            SaveGame::MassEffect1Leg {
                file_path,
                save_game: Box::new(unreal::Deserializer::from_bytes(&input)?),
            }
        } else if unreal::Deserializer::from_bytes::<Me2Version>(&input).is_ok() {
            // ME2
            SaveGame::MassEffect2 {
                file_path,
                save_game: Box::new(unreal::Deserializer::from_bytes(&input)?),
            }
        } else if unreal::Deserializer::from_bytes::<Me2LegVersion>(&input).is_ok() {
            // ME2 Legendary
            SaveGame::MassEffect2Leg {
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
        SaveGame::MassEffect1Leg { save_game, .. } => {
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
        SaveGame::MassEffect2Leg { save_game, .. } => {
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

    let mut file = File::create(&path).await?;
    file.write_all(&output).await?;

    let _ = ui_addr.send_async(UiEvent::Notification("Saved")).await;
    Ok(())
}

async fn load_me1_plot_db(ui_addr: Sender<UiEvent>) -> Result<()> {
    let mut input = String::new();
    {
        let mut file = File::open("databases/me1_plot_db.ron").await?;
        file.read_to_string(&mut input).await?;
    }

    let me1_plot_db: Me1PlotDb = ron::from_str(&input)?;

    let _ = ui_addr.send_async(UiEvent::LoadedMe1PlotDb(me1_plot_db)).await;
    Ok(())
}

async fn load_me1_item_db(ui_addr: Sender<UiEvent>) -> Result<()> {
    let mut input = String::new();
    {
        let mut file = File::open("databases/me1_item_db.ron").await?;
        file.read_to_string(&mut input).await?;
    }

    let me1_item_db: Me1ItemDb = ron::from_str(&input)?;

    let _ = ui_addr.send_async(UiEvent::LoadedMe1ItemDb(me1_item_db)).await;
    Ok(())
}

async fn load_me2_plot_db(ui_addr: Sender<UiEvent>) -> Result<()> {
    let mut input = String::new();
    {
        let mut file = File::open("databases/me2_plot_db.ron").await?;
        file.read_to_string(&mut input).await?;
    }

    let me2_plot_db: Me2PlotDb = ron::from_str(&input)?;

    let _ = ui_addr.send_async(UiEvent::LoadedMe2PlotDb(me2_plot_db)).await;
    Ok(())
}

async fn load_me3_plot_db(ui_addr: Sender<UiEvent>) -> Result<()> {
    let mut input = String::new();
    {
        let mut file = File::open("databases/me3_plot_db.ron").await?;
        file.read_to_string(&mut input).await?;
    }

    let me3_plot_db: Me3PlotDb = ron::from_str(&input)?;

    let _ = ui_addr.send_async(UiEvent::LoadedMe3PlotDb(me3_plot_db)).await;
    Ok(())
}

async fn import_head_morph(path: String, ui_addr: Sender<UiEvent>) -> Result<()> {
    let mut import = String::new();
    {
        let mut file = File::open(&path).await?;
        file.read_to_string(&mut import).await?;
    }

    let head_morph: HeadMorph = ron::from_str(&import)?;

    let _ = ui_addr.send_async(UiEvent::ImportedHeadMorph(head_morph)).await;
    let _ = ui_addr.send_async(UiEvent::Notification("Imported")).await;
    Ok(())
}

async fn export_head_morph(
    path: String, head_morph: Box<HeadMorph>, ui_addr: Sender<UiEvent>,
) -> Result<()> {
    let pretty_config =
        PrettyConfig::new().with_enumerate_arrays(true).with_new_line(String::from('\n'));

    let export = ron::ser::to_string_pretty(&head_morph, pretty_config)?;
    {
        let mut file = File::create(&path).await?;
        file.write_all(export.as_bytes()).await?;
    }

    let _ = ui_addr.send_async(UiEvent::Notification("Exported")).await;
    Ok(())
}
