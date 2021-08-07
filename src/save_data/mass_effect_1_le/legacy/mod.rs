use derive_more::Display;
use indexmap::IndexMap;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt;

use crate::{
    gui::RcUi,
    save_data::{
        shared::{Rotator, Vector},
        Dummy,
    },
};

mod pawn;
use self::pawn::*;

mod inventory;
use self::inventory::*;

mod art_placeable;
use self::art_placeable::*;

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, RawUi, RawUiChildren)]
pub struct Map {
    pub levels: IndexMap<String, Level>,
    pub world: Option<BaseObject>,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, RawUi, RawUiChildren)]
pub struct Level {
    pub objects: Vec<BaseObject>,
    pub actors: Vec<String>,
}

#[rcize_fields]
#[derive(Serialize, Clone, Default, Display)]
#[display(fmt = "{}", owner_name)]
pub struct BaseObject {
    pub _class_name: String,
    pub owner_name: String,
    pub owner_class: Option<String>,
    pub _object: Object,
}

impl<'de> Deserialize<'de> for BaseObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BaseObjectVisitor;
        impl<'de> de::Visitor<'de> for BaseObjectVisitor {
            type Value = BaseObject;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a BaseObject")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let class_name: String = seq.next_element()?.unwrap();
                let owner_name = seq.next_element()?.unwrap();
                let owner_class = seq.next_element()?.unwrap();
                let object = match class_name.as_str() {
                    "BioPawnBehaviorSaveObject" => {
                        Object::PawnBehavior(seq.next_element()?.unwrap())
                    }
                    "BioPawnSaveObject" => Object::Pawn(seq.next_element()?.unwrap()),
                    "BioBaseSquadSaveObject" => Object::BaseSquad(seq.next_element()?.unwrap()),
                    "BioShopSaveObject" => Object::Shop(seq.next_element()?.unwrap()),
                    "BioInventorySaveObject" => Object::Inventory(seq.next_element()?.unwrap()),
                    "BioItemXModdableSaveObject" => Object::Item(seq.next_element()?.unwrap()),
                    "BioItemXModSaveObject" => Object::ItemMod(seq.next_element()?.unwrap()),
                    "BioArtPlaceableBehaviorSaveObject" => {
                        Object::ArtPlaceableBehavior(seq.next_element()?.unwrap())
                    }
                    "BioArtPlaceableSaveObject" => {
                        Object::ArtPlaceable(seq.next_element()?.unwrap())
                    }
                    "BioVehicleBehaviorSaveObject" => {
                        Object::VehicleBehavior(seq.next_element()?.unwrap())
                    }
                    "BioVehicleSaveObject" => Object::Vehicle(seq.next_element()?.unwrap()),
                    "BioWorldInfoSaveObject" => Object::World(seq.next_element()?.unwrap()),
                    _ => unreachable!(),
                };

                Ok(BaseObject { _class_name: class_name, owner_name, owner_class, _object: object })
            }
        }
        deserializer.deserialize_tuple_struct("BaseObject", 4, BaseObjectVisitor)
    }
}

#[derive(Serialize, Clone)]
pub enum Object {
    PawnBehavior(RcUi<PawnBehavior>),
    Pawn(RcUi<Pawn>),
    BaseSquad(RcUi<BaseSquad>),
    Shop(RcUi<Shop>),
    Inventory(RcUi<Inventory>),
    Item(RcUi<Item>),
    ItemMod(RcUi<ItemMod>),
    ArtPlaceableBehavior(RcUi<ArtPlaceableBehavior>),
    ArtPlaceable(RcUi<ArtPlaceable>),
    VehicleBehavior(RcUi<VehicleBehavior>),
    Vehicle(RcUi<Vehicle>),
    World(RcUi<World>),
    Default,
}

impl Default for Object {
    fn default() -> Self {
        Object::Default
    }
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display)]
#[display(fmt = "")]
pub struct OptionObjectProxy {
    pub proxy: Option<BaseObject>,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUiChildren)]
pub struct VehicleBehavior {
    actor_type: String,
    powertrain_enabled: bool,
    vehicle_fonction_enabled: bool,
    owner: Option<BaseObject>,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUiChildren)]
pub struct Vehicle {
    location: Vector,
    rotation: Rotator,
    velocity: Vector,
    acceleration: Vector,
    script_initialized: bool,
    hidden: bool,
    stasis: bool,
    health_current: f32,
    shield_current: f32,
    first_name: String,
    localized_last_name: i32,
    _unknown: Dummy<16>,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "{}", name)]
struct WorldStreamingState {
    name: String,
    enabled: u8,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUiChildren)]
pub struct World {
    streaming_states: Vec<WorldStreamingState>,
    destination_area_map: String,
    destination: Vector,
    cinematics_seen: Vec<String>,
    scanned_clusters: Vec<i32>,
    scanned_systems: Vec<i32>,
    scanned_planets: Vec<i32>,
    journal_sort_method: u8,
    journal_showing_missions: bool,
    journal_last_selected_mission: i32,
    journal_last_selected_assignment: i32,
    codex_showing_primary: bool,
    codex_last_selected_primary: i32,
    codex_last_selected_secondary: i32,
    current_tip_id: i32,
    override_tip: i32,
    _browser_alerts: Dummy<8>, // [u8; 8]
    pending_loot: Option<BaseObject>,
}
