use anyhow::Result;
use indexmap::IndexMap;
use serde::{ser::SerializeTupleStruct, Deserialize, Deserializer, Serialize, Serializer};

use super::Vector;

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUi)]
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
    pub head_morph: Option<HeadMorph>,
}

#[derive(Deserialize, Serialize, Clone, RawUi)]
enum PlayerAppearanceType {
    Parts,
    Full,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUi, RawUiChildren)]
pub struct HeadMorph {
    pub hair_mesh: String,
    pub accessory_mesh: Vec<String>,
    pub morph_features: IndexMap<String, f32>,
    pub offset_bones: IndexMap<String, Vector>,
    pub lod0_vertices: Vec<Vector>,
    pub lod1_vertices: Vec<Vector>,
    pub lod2_vertices: Vec<Vector>,
    pub lod3_vertices: Vec<Vector>,
    pub scalar_parameters: IndexMap<String, f32>,
    pub vector_parameters: IndexMap<String, LinearColor>,
    pub texture_parameters: IndexMap<String, String>,
}

#[derive(Default, Clone)]
pub struct LinearColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl<'de> Deserialize<'de> for LinearColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct LinearColor(f32, f32, f32, f32);

        let LinearColor(r, g, b, a) = Deserialize::deserialize(deserializer)?;
        Ok(Self { r, g, b, a })
    }
}

impl serde::Serialize for LinearColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut linear_color = serializer.serialize_tuple_struct("LinearColor", 4)?;
        linear_color.serialize_field(&self.r)?;
        linear_color.serialize_field(&self.g)?;
        linear_color.serialize_field(&self.b)?;
        linear_color.serialize_field(&self.a)?;
        linear_color.end()
    }
}
