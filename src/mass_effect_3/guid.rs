use anyhow::Result;
use std::fmt::Debug;

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

    fn draw_raw_ui(&mut self, _ui: &Ui, _ident: &'static str) {}
}

impl Debug for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for b in self.0.iter() {
            string += &format!("{:2x}", b);
        }
        f.write_str(&string)
    }
}
