use anyhow::Result;
use async_trait::async_trait;

use crate::gui::Gui;

use super::{SaveCursor, SaveData};

#[derive(Clone)]
pub(super) struct Player {
    data: Vec<u8>,
}

#[async_trait(?Send)]
impl SaveData for Player {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let data = cursor.read_to_end()?.to_owned();
        Ok(Self { data })
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        Ok(output.extend(&self.data))
    }

    async fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
}
