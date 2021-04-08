use anyhow::Error;
use anyhow::Result;
use flume::{Receiver, Sender};
use std::path::PathBuf;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{
    mass_effect_3::Me3SaveGame,
    save_data::{SaveCursor, SaveData},
    ui::UiEvent,
};

pub enum MainEvent {
    OpenSave(PathBuf),
    SaveSave((PathBuf, Me3SaveGame)),
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

    let mut input = SaveCursor::new(input);
    let me3_save_game = Me3SaveGame::deserialize(&mut input)?;
    let _ = ui_addr.send_async(UiEvent::MassEffect3(Box::new(me3_save_game))).await;
    Ok(())
}

async fn save_save(
    path: PathBuf, me3_save_game: Me3SaveGame, ui_addr: &Sender<UiEvent>,
) -> Result<()> {
    let mut output = Vec::new();

    Me3SaveGame::serialize(&me3_save_game, &mut output)?;

    let mut file = File::create(path).await?;
    file.write_all(&output).await?;
    Ok(())
}
