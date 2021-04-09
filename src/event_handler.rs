use anyhow::{bail, Error, Result};
use flume::{Receiver, Sender};
use std::path::{Path, PathBuf};
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{
    save_data::{
        mass_effect_2::{self, Me2SaveGame},
        mass_effect_3::Me3SaveGame,
        SaveCursor, SaveData,
    },
    ui::UiEvent,
};

pub enum MainEvent {
    OpenSave(PathBuf),
    SaveSave((PathBuf, SaveGame)),
}

// TODO: virer annotation quand ME1 implémenté
#[allow(dead_code)]
#[derive(Clone)]
pub enum SaveGame {
    MassEffect1,
    MassEffect2(Box<Me2SaveGame>),
    MassEffect3(Box<Me3SaveGame>),
}

pub async fn event_loop(rx: Receiver<MainEvent>, ui_addr: Sender<UiEvent>) {
    while let Ok(event) = rx.recv_async().await {
        let result = async {
            match event {
                MainEvent::OpenSave(path) => open_save(path, &ui_addr).await?,
                MainEvent::SaveSave((path, save_game)) => {
                    save_save(path, save_game, &ui_addr).await?
                }
            };
            Ok::<_, Error>(())
        };

        if let Err(err) = result.await {
            let _ = ui_addr.send_async(UiEvent::Error(err)).await;
        }
    }
}

async fn open_save(path: PathBuf, ui_addr: &Sender<UiEvent>) -> Result<()> {
    let mut input = Vec::new();
    {
        let mut file = File::open(&path).await?;
        file.read_to_end(&mut input).await?;
    }

    if let Some(ext) = path.extension() {
        let mut cursor = SaveCursor::new(input);
        let save_game = match ext.to_string_lossy().to_lowercase().as_str() {
            "masseffectsave" => {bail!("Mass Effect 1 not implemented (yet)");},
            _ => {
                let is_me2 = mass_effect_2::Version::deserialize(&mut cursor).is_ok();
                cursor.rshift_position(4)?;
                if is_me2 {
                    SaveGame::MassEffect2(Box::new(Me2SaveGame::deserialize(&mut cursor)?))
                } else {
                    SaveGame::MassEffect3(Box::new(Me3SaveGame::deserialize(&mut cursor)?))
                }
            }
        };

        let _ = ui_addr.send_async(UiEvent::OpenedSave(save_game)).await;
    }
    Ok(())
}

async fn save_save(path: PathBuf, save_game: SaveGame, ui_addr: &Sender<UiEvent>) -> Result<()> {
    let mut output = Vec::new();

    match save_game {
        SaveGame::MassEffect1 => todo!(),
        SaveGame::MassEffect2(save_game) => Me2SaveGame::serialize(&save_game, &mut output)?,
        SaveGame::MassEffect3(save_game) => Me3SaveGame::serialize(&save_game, &mut output)?,
    };

    // if save exists
    if fs::metadata(&path).await.is_ok() {
        if let Some(ext) = path.extension() {
            let to = Path::with_extension(&path, ext.to_string_lossy().into_owned() + ".bak");
            fs::rename(&path, to).await?;
        }
    }

    let mut file = File::create(path).await?;
    file.write_all(&output).await?;

    let _ = ui_addr.send_async(UiEvent::Notification("Saved")).await;
    Ok(())
}
