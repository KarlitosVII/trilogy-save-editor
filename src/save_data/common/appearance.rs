use anyhow::Result;
use imgui::ImString;

use crate::{
    save_data::{SaveCursor, SaveData},
    ui::Ui,
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
    has_head_morph: bool,
    head_morph: Option<HeadMorph>,
}

#[derive(FromPrimitive, ToPrimitive, SaveData, Clone)]
enum PlayerAppearanceType {
    Parts,
    Full,
}

#[derive(SaveData, Clone)]
struct HeadMorph {
    hair_mesh: ImString,
    accessory_mesh: Vec<ImString>,
    morph_features: Vec<MorphFeature>,
    offset_bones: Vec<OffsetBone>,
    lod0_vertices: Vec<Vector>,
    lod1_vertices: Vec<Vector>,
    lod2_vertices: Vec<Vector>,
    lod3_vertices: Vec<Vector>,
    scalar_parameters: Vec<ScalarParameter>,
    vector_parameters: Vec<VectorParameter>,
    texture_parameters: Vec<TextureParameter>,
}

#[derive(SaveData, Default, Clone)]
struct MorphFeature {
    feature: ImString,
    offset: f32,
}

#[derive(SaveData, Default, Clone)]
struct OffsetBone {
    name: ImString,
    offset: Vector,
}

#[derive(SaveData, Default, Clone)]
struct ScalarParameter {
    name: ImString,
    value: f32,
}

#[derive(SaveData, Default, Clone)]
struct VectorParameter {
    name: ImString,
    value: LinearColor,
}

#[derive(Default, Clone)]
struct LinearColor([f32; 4]);

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
        for byte in self.0.iter() {
            Self::serialize_to(byte, output)?;
        }
        Ok(())
    }

    fn draw_raw_ui(&mut self, ui: &Ui, ident: &str) {
        ui.draw_edit_color(ident, &mut self.0);
    }
}

#[derive(SaveData, Default, Clone)]
struct TextureParameter {
    name: ImString,
    value: ImString,
}