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
        common::appearance::HeadMorph,
        mass_effect_1::{known_plot::Me1KnownPlot, Me1SaveGame},
        mass_effect_2::{self, known_plot::Me2KnownPlot, Me2SaveGame},
        mass_effect_3::{known_plot::Me3KnownPlot, Me3SaveGame},
        SaveCursor, SaveData,
    },
    unreal,
};

pub enum MainEvent {
    OpenSave(PathBuf),
    SaveSave(PathBuf, SaveGame),
    LoadKnownPlots,
    ImportHeadMorph(PathBuf),
    ExportHeadMorph(PathBuf, Box<HeadMorph>),
}

#[derive(Clone)]
pub enum SaveGame {
    MassEffect1(Box<Me1SaveGame>),
    MassEffect2(Box<Me2SaveGame>),
    MassEffect3(Box<Me3SaveGame>),
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
                MainEvent::LoadKnownPlots => {
                    let me1_handle = tokio::spawn(load_me1_known_plot(Sender::clone(&ui_addr)));
                    let me2_handle = tokio::spawn(load_me2_known_plot(Sender::clone(&ui_addr)));
                    let me3_handle = tokio::spawn(load_me3_known_plot(ui_addr));

                    let (me1_result, me2_result, me3_result) =
                        tokio::join!(me1_handle, me2_handle, me3_handle);

                    me1_result?.context("Failed to parse Me1KnownPlot.ron")?;
                    me2_result?.context("Failed to parse Me2KnownPlot.ron")?;
                    me3_result?.context("Failed to parse Me3KnownPlot.ron")
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

async fn open_save(path: PathBuf, ui_addr: Sender<UiEvent>) -> Result<()> {
    let mut input = Vec::new();
    {
        let mut file = File::open(&path).await?;
        file.read_to_end(&mut input).await?;
    }

    if let Some(ext) = path.extension() {
        let save_game = match ext.to_string_lossy().to_lowercase().as_str() {
            "masseffectsave" => {
                let mut cursor = SaveCursor::new(input);
                SaveGame::MassEffect1(Box::new(Me1SaveGame::deserialize(&mut cursor)?))
            }
            _ => {
                let is_me2 =
                    unreal::Deserializer::from_bytes::<mass_effect_2::Version>(&input).is_ok();
                if is_me2 {
                    SaveGame::MassEffect2(Box::new(unreal::Deserializer::from_bytes(&input)?))
                } else {
                    SaveGame::MassEffect3(Box::new(unreal::Deserializer::from_bytes(&input)?))
                }
            }
        };

        let _ = ui_addr.send_async(UiEvent::OpenedSave(save_game)).await;
        let _ = ui_addr.send_async(UiEvent::Notification("Opened")).await;
    }
    Ok(())
}

async fn save_save(path: PathBuf, save_game: SaveGame, ui_addr: Sender<UiEvent>) -> Result<()> {
    let output = match save_game {
        SaveGame::MassEffect1(save_game) => unreal::Serializer::to_byte_buf(&save_game)?,
        SaveGame::MassEffect2(save_game) => {
            let mut output = unreal::Serializer::to_byte_buf(&save_game)?;

            let crc = Crc::<u32>::new(&CRC_32_BZIP2);
            let checksum = crc.checksum(&output);
            output.extend(&u32::to_le_bytes(checksum));
            output
        }
        SaveGame::MassEffect3(save_game) => {
            let mut output = unreal::Serializer::to_byte_buf(&save_game)?;

            let crc = Crc::<u32>::new(&CRC_32_BZIP2);
            let checksum = crc.checksum(&output);
            output.extend(&u32::to_le_bytes(checksum));
            output
        }
    };

    // Backup si fichier existe
    if fs::metadata(&path).await.is_ok() {
        if let Some(ext) = path.extension() {
            let to = Path::with_extension(&path, ext.to_string_lossy().into_owned() + ".bak");
            fs::rename(&path, to).await?;
        }
    }

    let mut file = File::create(&path).await?;
    file.write_all(&output).await?;

    let _ = ui_addr.send_async(UiEvent::Notification("Saved")).await;
    Ok(())
}

async fn load_me1_known_plot(ui_addr: Sender<UiEvent>) -> Result<()> {
    let mut input = String::new();
    {
        let mut file = File::open("plot/Me1KnownPlot.ron").await?;
        file.read_to_string(&mut input).await?;
    }

    let me1_known_plot: Me1KnownPlot = ron::from_str(&input)?;

    let _ = ui_addr.send_async(UiEvent::LoadedMe1KnownPlot(me1_known_plot)).await;
    Ok(())
}

async fn load_me2_known_plot(ui_addr: Sender<UiEvent>) -> Result<()> {
    let mut input = String::new();
    {
        let mut file = File::open("plot/Me2KnownPlot.ron").await?;
        file.read_to_string(&mut input).await?;
    }

    let me2_known_plot: Me2KnownPlot = ron::from_str(&input)?;

    let _ = ui_addr.send_async(UiEvent::LoadedMe2KnownPlot(me2_known_plot)).await;
    Ok(())
}

async fn load_me3_known_plot(ui_addr: Sender<UiEvent>) -> Result<()> {
    let mut input = String::new();
    {
        let mut file = File::open("plot/Me3KnownPlot.ron").await?;
        file.read_to_string(&mut input).await?;
    }

    let me3_known_plot: Me3KnownPlot = ron::from_str(&input)?;

    let _ = ui_addr.send_async(UiEvent::LoadedMe3KnownPlot(me3_known_plot)).await;
    Ok(())
}

async fn import_head_morph(path: PathBuf, ui_addr: Sender<UiEvent>) -> Result<()> {
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
    path: PathBuf, head_morph: Box<HeadMorph>, ui_addr: Sender<UiEvent>,
) -> Result<()> {
    let pretty_config =
        PrettyConfig::new().with_enumerate_arrays(true).with_new_line(String::from('\n'));
    let export = ron::ser::to_string_pretty(&head_morph, pretty_config)?;

    let mut file = File::create(&path).await?;
    file.write_all(export.as_bytes()).await?;

    let _ = ui_addr.send_async(UiEvent::Notification("Exported")).await;
    Ok(())
}
