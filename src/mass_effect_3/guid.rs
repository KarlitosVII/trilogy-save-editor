use anyhow::Result;

use crate::{
    save_data::{SaveCursor, SaveData},
    ui::Ui,
};

pub(super) struct Guid(Vec<u8>);

impl SaveData for Guid {
    fn deserialize(input: &mut SaveCursor) -> Result<Self> {
        let guid = input.read(16)?.to_owned();
        Ok(Self(guid))
    }

    fn draw_raw_ui(&mut self, _: &Ui, _: &str) {}
}
