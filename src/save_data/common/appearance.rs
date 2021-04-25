use anyhow::*;
use imgui::ImString;

use crate::{
    gui::Gui,
    save_data::{SaveCursor, SaveData},
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
    head_morph: HasHeadMorph,
}

#[derive(SaveData, Clone)]
enum PlayerAppearanceType {
    Parts,
    Full,
}

#[derive(Clone)]
pub struct HasHeadMorph {
    has_head_morph: bool,
    head_morph: Option<HeadMorph>,
}

impl SaveData for HasHeadMorph {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let has_head_morph = <bool>::deserialize(cursor)?;

        let head_morph = if has_head_morph { Some(SaveData::deserialize(cursor)?) } else { None };
        Ok(Self { has_head_morph, head_morph })
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        self.has_head_morph.serialize(output)?;
        if self.has_head_morph {
            let head_morph = self.head_morph.as_ref().context("You cannot enable head morph without head morph data. Please import a head morph first.")?;
            head_morph.serialize(output)?;
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

#[derive(SaveData, Clone)]
pub struct HeadMorph {
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
pub struct LinearColor([f32; 4]);

impl SaveData for LinearColor {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        Ok(Self([
            <f32>::deserialize(cursor)?,
            <f32>::deserialize(cursor)?,
            <f32>::deserialize(cursor)?,
            <f32>::deserialize(cursor)?,
        ]))
    }

    fn serialize(&self, output: &mut Vec<u8>) -> Result<()> {
        for byte in self.0.iter() {
            <f32>::serialize(byte, output)?;
        }
        Ok(())
    }

    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        gui.draw_edit_color(ident, &mut self.0);
    }
}

#[derive(SaveData, Default, Clone)]
struct TextureParameter {
    name: ImString,
    value: ImString,
}
