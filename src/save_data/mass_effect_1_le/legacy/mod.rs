use imgui::im_str;
use indexmap::IndexMap;
use serde::{de, Deserialize, Serialize};
use std::fmt;

use crate::{
    gui::{imgui_utils::Table, Gui},
    save_data::{
        shared::{Rotator, Vector},
        Dummy, ImguiString, RawUi, RawUiMe1Legacy,
    },
};

mod pawn;
use self::pawn::*;

mod inventory;
use self::inventory::*;

mod art_placeable;
use self::art_placeable::*;

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Map {
    levels: IndexMap<ImguiString, Level>,
    _unknown: Dummy<4>,
}

impl RawUi for Map {
    fn draw_raw_ui(&mut self, gui: &Gui, _: &str) {
        let Map { levels, _unknown } = self;
        levels.draw_raw_ui(gui, "Levels");
    }
}

#[derive(Deserialize, Serialize, Clone, Default)]
struct Level {
    objects: Vec<BaseObject>,
    actors: Vec<ImguiString>,
}

impl RawUi for Level {
    fn draw_raw_ui(&mut self, gui: &Gui, _: &str) {
        let Level { objects, actors } = self;
        objects.draw_raw_ui(gui, "Objects");
        Table::next_row();
        actors.draw_raw_ui(gui, "Actors");
    }
}

#[derive(Serialize, Clone)]
pub struct BaseObject {
    class_name: ImguiString,
    owner_name: ImguiString,
    owner_class: Option<ImguiString>,
    object: Object,
}

impl RawUi for BaseObject {
    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        let BaseObject { class_name, owner_name, owner_class, object } = self;
        let class_name = Box::new(|| gui.draw_text(class_name, Some(im_str!("Class Name"))));
        let owner_name = Box::new(|| owner_name.draw_raw_ui(gui, "Owner Name"));
        let owner_class = Box::new(|| owner_class.draw_raw_ui(gui, "Owner Class"));

        let object = match object {
            Object::PawnBehavior(pawn_behavior) => pawn_behavior.draw_fields(gui),
            Object::Pawn(pawn) => pawn.draw_fields(gui),
            Object::BaseSquad(squad) => squad.draw_fields(gui),
            Object::Shop(shop) => shop.draw_fields(gui),
            Object::Inventory(inventory) => inventory.draw_fields(gui),
            Object::Item(item) => item.draw_fields(gui),
            Object::ItemMod(item_mod) => item_mod.draw_fields(gui),
            Object::ArtPlaceableBehavior(art_placeable_behavior) => {
                art_placeable_behavior.draw_fields(gui)
            }
            Object::ArtPlaceable(art_placeable) => art_placeable.draw_fields(gui),
            Object::VehicleBehavior(vehicle_behavior) => vehicle_behavior.draw_fields(gui),
            Object::Vehicle(vehicle) => vehicle.draw_fields(gui),
            Object::World(world) => world.draw_fields(gui),
        };

        let mut base_object: Vec<Box<dyn FnMut()>> = vec![class_name, owner_name, owner_class];
        base_object.extend(object);
        gui.draw_struct(ident, &mut base_object);
    }
}

impl RawUi for Vec<BaseObject> {
    fn draw_raw_ui(&mut self, gui: &Gui, ident: &str) {
        gui.draw_vec_no_edit(ident, self);
    }
}

impl<'de> serde::Deserialize<'de> for BaseObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
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
                let class_name: ImguiString = seq.next_element()?.unwrap();
                let owner_name = seq.next_element()?.unwrap();
                let owner_class = seq.next_element()?.unwrap();
                let object = match class_name.to_str() {
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

                Ok(BaseObject { class_name, owner_name, owner_class, object })
            }
        }
        deserializer.deserialize_tuple_struct("BaseObject", 4, BaseObjectVisitor)
    }
}

#[derive(Serialize, Clone)]
enum Object {
    PawnBehavior(PawnBehavior),
    Pawn(Pawn),
    BaseSquad(BaseSquad),
    Shop(Shop),
    Inventory(Inventory),
    Item(Item),
    ItemMod(ItemMod),
    ArtPlaceableBehavior(ArtPlaceableBehavior),
    ArtPlaceable(ArtPlaceable),
    VehicleBehavior(VehicleBehavior),
    Vehicle(Vehicle),
    World(World),
}

#[derive(Deserialize, Serialize, RawUiMe1Legacy, Clone)]
struct VehicleBehavior {
    actor_type: ImguiString,
    powertrain_enabled: bool,
    vehicle_fonction_enabled: bool,
    owner: Box<Option<BaseObject>>,
}

#[derive(Deserialize, Serialize, RawUiMe1Legacy, Clone)]
struct Vehicle {
    location: Vector,
    rotation: Rotator,
    velocity: Vector,
    acceleration: Vector,
    script_initialized: bool,
    hidden: bool,
    stasis: bool,
    health_current: f32,
    shield_current: f32,
    first_name: ImguiString,
    localized_last_name: i32,
    _unknown: Dummy<16>,
}

#[derive(Deserialize, Serialize, RawUi, Clone, Default)]
struct WorldStreamingState {
    name: ImguiString,
    enabled: u8,
}

#[derive(Deserialize, Serialize, RawUiMe1Legacy, Clone)]
struct World {
    streaming_states: Vec<WorldStreamingState>,
    destination_area_map: ImguiString,
    destination: Vector,
    cinematics_seen: Vec<ImguiString>,
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
    pending_loot: Box<Option<BaseObject>>,
}
