use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::{
    gui::Gui,
    save_data::{ImguiString, SaveCursor, SaveData},
};

use super::Vector;

#[derive(SaveData, Clone)]
pub struct Appearance {
    combat_appearance: PlayerAppearanceType,
    casual_id: i32,
    full_body_id: i32,
    torso_id: i32,
    shoulder_id: i32,
    arm_id: i32,
    leg_id: i32,
    specular_id: i32,
    tint1_id: i32,
    tint2_id: i32,
    tint3_id: i32,
    pattern_id: i32,
    pattern_color_id: i32,
    helmet_id: i32,
    pub head_morph: HasHeadMorph,
}

#[derive(SaveData, Clone)]
enum PlayerAppearanceType {
    Parts,
    Full,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct HasHeadMorph {
    pub has_head_morph: bool,
    pub head_morph: Option<HeadMorph>,
}

impl SaveData for HasHeadMorph {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let has_head_morph: bool = SaveData::deserialize(cursor)?;
        let head_morph = if has_head_morph { Some(SaveData::deserialize(cursor)?) } else { None };
        Ok(Self { has_head_morph, head_morph })
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        SaveData::serialize(&self.has_head_morph, output)?;
        if self.has_head_morph {
            let head_morph = self.head_morph.as_ref().context("You cannot enable head morph without head morph data. Please import a head morph first.")?;
            SaveData::serialize(head_morph, output)?;
        }
        Ok(())
    }

    fn draw_raw_ui(&mut self, gui: &Gui, _: &str) {
        self.has_head_morph.draw_raw_ui(gui, "has_head_morph");
        if let Some(head_morph) = &mut self.head_morph {
            head_morph.draw_raw_ui(gui, "head_morph");
        }
    }
}

#[derive(SaveData, Deserialize, Serialize, Clone)]
pub struct HeadMorph {
    pub hair_mesh: ImguiString,
    pub accessory_mesh: Vec<ImguiString>,
    pub morph_features: Vec<MorphFeature>,
    pub offset_bones: Vec<OffsetBone>,
    pub lod0_vertices: Vec<Vector>,
    pub lod1_vertices: Vec<Vector>,
    pub lod2_vertices: Vec<Vector>,
    pub lod3_vertices: Vec<Vector>,
    pub scalar_parameters: Vec<ScalarParameter>,
    pub vector_parameters: Vec<VectorParameter>,
    pub texture_parameters: Vec<TextureParameter>,
}

#[derive(SaveData, Deserialize, Serialize, Default, Clone)]
pub struct MorphFeature {
    feature: ImguiString,
    offset: f32,
}

#[derive(SaveData, Deserialize, Serialize, Default, Clone)]
pub struct OffsetBone {
    name: ImguiString,
    offset: Vector,
}

#[derive(SaveData, Deserialize, Serialize, Default, Clone)]
pub struct ScalarParameter {
    name: ImguiString,
    value: f32,
}

#[derive(SaveData, Deserialize, Serialize, Default, Clone)]
pub struct VectorParameter {
    name: ImguiString,
    value: LinearColor,
}

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct LinearColor([f32; 4]);

impl SaveData for LinearColor {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        Ok(Self([
            SaveData::deserialize(cursor)?,
            SaveData::deserialize(cursor)?,
            SaveData::deserialize(cursor)?,
            SaveData::deserialize(cursor)?,
        ]))
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        for float in self.0.iter() {
            SaveData::serialize(float, output)?;
        }
        Ok(())
    }

    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        gui.draw_edit_color(ident, &mut self.0);
    }
}

#[derive(SaveData, Deserialize, Serialize, Default, Clone)]
pub struct TextureParameter {
    name: ImguiString,
    value: ImguiString,
}
