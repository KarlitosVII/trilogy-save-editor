use anyhow::*;
use flume::{Receiver, Sender};
use std::path::{Path, PathBuf};
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{
    gui::UiEvent,
    save_data::{
        mass_effect_1::{known_plot::Me1KnownPlot, Me1SaveGame},
        mass_effect_2::{self, known_plot::Me2KnownPlot, Me2SaveGame},
        mass_effect_3::{known_plot::Me3KnownPlot, Me3SaveGame},
        SaveCursor, SaveData,
    },
};

pub enum MainEvent {
    OpenSave(PathBuf),
    SaveSave(PathBuf, SaveGame),
    LoadKnownPlots,
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
        let mut cursor = SaveCursor::new(input);
        let save_game = match ext.to_string_lossy().to_lowercase().as_str() {
            "masseffectsave" => {
                SaveGame::MassEffect1(Box::new(Me1SaveGame::deserialize(&mut cursor)?))
            }
            _ => {
                let is_me2 = mass_effect_2::Version::deserialize(&mut cursor).is_ok();
                cursor.rshift_position(4);
                if is_me2 {
                    SaveGame::MassEffect2(Box::new(Me2SaveGame::deserialize(&mut cursor)?))
                } else {
                    SaveGame::MassEffect3(Box::new(Me3SaveGame::deserialize(&mut cursor)?))
                }
            }
        };

        let _ = ui_addr.send_async(UiEvent::OpenedSave(save_game)).await;
        let _ = ui_addr.send_async(UiEvent::Notification("Opened")).await;
    }
    Ok(())
}

async fn save_save(path: PathBuf, save_game: SaveGame, ui_addr: Sender<UiEvent>) -> Result<()> {
    let mut output = Vec::new();

    match save_game {
        SaveGame::MassEffect1(save_game) => Me1SaveGame::serialize(&save_game, &mut output)?,
        SaveGame::MassEffect2(save_game) => Me2SaveGame::serialize(&save_game, &mut output)?,
        SaveGame::MassEffect3(save_game) => Me3SaveGame::serialize(&save_game, &mut output)?,
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
