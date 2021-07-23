use anyhow::Result;
use indexmap::IndexMap;
use serde::{ser::SerializeTupleStruct, Deserialize, Serialize};

use super::Vector;

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone)]
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

#[derive(Deserialize, Serialize, Clone, PartialEq, RawUi)]
enum PlayerAppearanceType {
    Parts,
    Full,
}

#[rc_ize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone)]
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
pub struct LinearColor([f32; 4]);

impl<'de> serde::Deserialize<'de> for LinearColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct LinearColor(f32, f32, f32, f32);

        let LinearColor(r, g, b, a) = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self([r, g, b, a]))
    }
}

impl serde::Serialize for LinearColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut linear_color = serializer.serialize_tuple_struct("LinearColor", 4)?;
        for float in self.0.iter() {
            linear_color.serialize_field(float)?;
        }
        linear_color.end()
    }
}
