use anyhow::Result;
use indexmap::IndexMap;
use serde::{
    de,
    ser::{Error, SerializeStruct, SerializeTupleStruct},
    Deserialize, Serialize,
};
use std::fmt;

use crate::{
    gui::Gui,
    save_data::{ImguiString, RawUi},
};

use super::Vector;

#[derive(Deserialize, Serialize, RawUi, Clone)]
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

#[derive(Deserialize, Serialize, RawUi, Clone)]
enum PlayerAppearanceType {
    Parts,
    Full,
}

#[derive(Clone)]
pub struct HasHeadMorph {
    pub has_head_morph: bool,
    pub head_morph: Option<HeadMorph>,
}

impl RawUi for HasHeadMorph {
    fn draw_raw_ui(&mut self, gui: &Gui, _: &str) {
        self.has_head_morph.draw_raw_ui(gui, "Has Head Morph");
        if let Some(head_morph) = &mut self.head_morph {
            head_morph.draw_raw_ui(gui, "Head Morph");
        }
    }
}

impl<'de> serde::Deserialize<'de> for HasHeadMorph {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct HasHeadMorphVisitor;
        impl<'de> de::Visitor<'de> for HasHeadMorphVisitor {
            type Value = HasHeadMorph;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a seq")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let has_head_morph = seq.next_element()?.unwrap();
                let head_morph =
                    if has_head_morph { Some(seq.next_element()?.unwrap()) } else { None };
                Ok(HasHeadMorph { has_head_morph, head_morph })
            }
        }
        deserializer.deserialize_tuple_struct("HasHeadMorph", 2, HasHeadMorphVisitor)
    }
}

impl serde::Serialize for HasHeadMorph {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("HasHeadMorph", 2)?;
        s.serialize_field("has_head_morph", &self.has_head_morph)?;
        if self.has_head_morph {
            match self.head_morph {
                Some(ref head_morph) => s.serialize_field("head_morph", head_morph)?,
                None => return Err(S::Error::custom("You cannot enable head morph without head morph data. Please import a head morph first.")),
            }
        }
        s.end()
    }
}

#[derive(Deserialize, Serialize, RawUi, Clone)]
pub struct HeadMorph {
    pub hair_mesh: ImguiString,
    pub accessory_mesh: Vec<ImguiString>,
    pub morph_features: IndexMap<ImguiString, f32>,
    pub offset_bones: IndexMap<ImguiString, Vector>,
    pub lod0_vertices: Vec<Vector>,
    pub lod1_vertices: Vec<Vector>,
    pub lod2_vertices: Vec<Vector>,
    pub lod3_vertices: Vec<Vector>,
    pub scalar_parameters: IndexMap<ImguiString, f32>,
    pub vector_parameters: IndexMap<ImguiString, LinearColor>,
    pub texture_parameters: IndexMap<ImguiString, ImguiString>,
}

#[derive(Default, Clone)]
pub struct LinearColor([f32; 4]);

impl RawUi for LinearColor {
    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        gui.draw_edit_color(ident, &mut self.0);
    }
}

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
