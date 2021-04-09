use anyhow::Error;
use anyhow::Result;
use flume::{Receiver, Sender};
use std::path::{Path, PathBuf};
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{
    mass_effect_3::Me3SaveGame,
    save_data::{SaveCursor, SaveData},
    ui::UiEvent,
};

pub enum MainEvent {
    OpenSave(PathBuf),
    SaveSave((PathBuf, Box<Me3SaveGame>)),
}

pub async fn event_loop(rx: Receiver<MainEvent>, ui_addr: Sender<UiEvent>) {
    while let Ok(event) = rx.recv_async().await {
        let result = async {
            match event {
                MainEvent::OpenSave(path) => open_save(path, &ui_addr).await?,
                MainEvent::SaveSave((path, me3_save_game)) => {
                    save_save(path, me3_save_game, &ui_addr).await?
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
        let mut file = File::open(path).await?;
        file.read_to_end(&mut input).await?;
    }

    let mut cursor = SaveCursor::new(input);
    let me3_save_game = Me3SaveGame::deserialize(&mut cursor)?;

    let _ = ui_addr.send_async(UiEvent::OpenMassEffect3(Box::new(me3_save_game))).await;
    Ok(())
}

async fn save_save(
    path: PathBuf, me3_save_game: Box<Me3SaveGame>, ui_addr: &Sender<UiEvent>,
) -> Result<()> {
    let mut output = Vec::new();

    Me3SaveGame::serialize(&me3_save_game, &mut output)?;

    // if save exists
    if fs::metadata(&path).await.is_ok() {
        if let Some(ext) = path.extension() {
            let to = Path::with_extension(&path, ext.to_string_lossy().to_string() + ".bak");
            fs::rename(&path, to).await?;
        }
    }

    let mut file = File::create(path).await?;
    file.write_all(&output).await?;

    let _ = ui_addr.send_async(UiEvent::Notification("Saved")).await;
    Ok(())
}
