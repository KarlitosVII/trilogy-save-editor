use imgui::ImString;
use std::fmt::Debug;
use anyhow::Result;

use crate::{save_data::{SaveCursor, SaveData}, ui::Ui};

use super::Vector;

#[derive(SaveData, Debug)]
pub(super) struct Appearance {
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

#[derive(FromPrimitive, ToPrimitive, SaveData, Debug)]
enum PlayerAppearanceType {
    Parts = 0,
    Full = 1,
}

#[derive(SaveData, Debug)]
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

#[derive(SaveData, Debug)]
struct MorphFeature {
    feature: ImString,
    offset: f32,
}

#[derive(SaveData, Debug)]
struct OffsetBone {
    name: ImString,
    offset: Vector,
}

#[derive(SaveData, Debug)]
struct ScalarParameter {
    name: ImString,
    value: f32,
}

#[derive(SaveData, Debug)]
struct VectorParameter {
    name: ImString,
    value: LinearColor,
}

#[derive(Debug)]
struct LinearColor (
    [f32; 4]
);

impl SaveData for LinearColor {
    fn deserialize(input: &mut SaveCursor) -> Result<Self> {
        Ok(Self ([
            SaveData::deserialize(input)?,
            SaveData::deserialize(input)?,
            SaveData::deserialize(input)?,
            SaveData::deserialize(input)?,
        ]))
    }

    fn draw_raw_ui(&mut self, ui: &Ui, ident: &str) {
        ui.draw_edit_color(ident, &mut self.0);
    }
}

#[derive(SaveData, Debug)]
struct TextureParameter {
    name: ImString,
    value: ImString,
}
