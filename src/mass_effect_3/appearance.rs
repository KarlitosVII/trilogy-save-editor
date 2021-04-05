use anyhow::Result;
use std::fmt::Debug;

use crate::serializer::{SaveCursor, Serializable};

use super::Vector;

#[derive(Debug)]
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

impl Serializable for Appearance {
    fn deserialize(input: &mut SaveCursor) -> Result<Self> {
        let combat_appearance = PlayerAppearanceType::deserialize(input)?;
        let casual_id = i32::deserialize(input)?;
        let full_body_id = i32::deserialize(input)?;
        let torso_id = i32::deserialize(input)?;
        let shoulder_id = i32::deserialize(input)?;
        let arm_id = i32::deserialize(input)?;
        let leg_id = i32::deserialize(input)?;
        let specular_id = i32::deserialize(input)?;
        let tint1_id = i32::deserialize(input)?;
        let tint2_id = i32::deserialize(input)?;
        let tint3_id = i32::deserialize(input)?;
        let pattern_id = i32::deserialize(input)?;
        let pattern_color_id = i32::deserialize(input)?;
        let helmet_id = i32::deserialize(input)?;
        let has_head_morph = bool::deserialize(input)?;
        let head_morph = match has_head_morph {
            true => Some(HeadMorph::deserialize(input)?),
            false => None,
        };

        Ok(Self {
            combat_appearance,
            casual_id,
            full_body_id,
            torso_id,
            shoulder_id,
            arm_id,
            leg_id,
            specular_id,
            tint1_id,
            tint2_id,
            tint3_id,
            pattern_id,
            pattern_color_id,
            helmet_id,
            has_head_morph,
            head_morph,
        })
    }
}

#[derive(FromPrimitive, ToPrimitive, Serializable, Debug)]
enum PlayerAppearanceType {
    Parts = 0,
    Full = 1,
}

#[derive(Serializable, Debug)]
struct HeadMorph {
    hair_mesh: String,
    accessory_mesh: Vec<String>,
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

#[derive(Serializable, Debug)]
struct MorphFeature {
    feature: String,
    offset: f32,
}

#[derive(Serializable, Debug)]
struct OffsetBone {
    name: String,
    offset: Vector,
}

#[derive(Serializable, Debug)]
struct ScalarParameter {
    name: String,
    value: f32,
}

#[derive(Serializable, Debug)]
struct VectorParameter {
    name: String,
    value: LinearColor,
}

#[derive(Serializable, Debug)]
struct LinearColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[derive(Serializable, Debug)]
struct TextureParameter {
    name: String,
    value: String,
}
