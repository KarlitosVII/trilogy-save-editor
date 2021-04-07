use std::path::PathBuf;

use anyhow::Error;
use flume::{Receiver, Sender};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    mass_effect_3::Me3SaveGame,
    save_data::{SaveCursor, SaveData},
    ui::UiEvent,
};

pub enum MainEvent {
    OpenSave(PathBuf),
}

pub async fn event_loop(rx: Receiver<MainEvent>, ui_addr: Sender<UiEvent>) {
    while let Ok(event) = rx.recv_async().await {
        let result = async {
            match event {
                MainEvent::OpenSave(path) => {
                    let mut input = Vec::new();
                    {
                        let mut file = File::open(path).await?;
                        file.read_to_end(&mut input).await?;
                    }

                    let mut input = SaveCursor::new(input);
                    let me3_save_game = Me3SaveGame::deserialize(&mut input)?;
                    let _ = ui_addr.send_async(UiEvent::MassEffect3(Box::new(me3_save_game))).await;
                }
            };
            Ok::<_, Error>(())
        };

        if let Err(err) = result.await {
            let _ = ui_addr.send_async(UiEvent::Error(err)).await;
        }
    }
}
