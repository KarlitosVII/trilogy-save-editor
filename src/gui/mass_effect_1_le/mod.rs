use yew::prelude::*;

use crate::{
    gui::{
        components::{raw_ui::RawUiStruct, Table},
        raw_ui::{RawUi, RawUiMe1Legacy},
    },
    save_data::mass_effect_1_le::{
        legacy::{BaseObject, HasObject, Object},
        Me1LeSaveData,
    },
};

use super::RcUi;

mod general;

pub use self::general::*;

impl RawUi for RcUi<Me1LeSaveData> {
    fn view(&self, _: &str) -> yew::Html {
        let Me1LeSaveData {
            character_id,
            created_date,
            plot,
            timestamp,
            seconds_played,
            player,
            base_level_name,
            map_name,
            parent_map_name,
            location,
            rotation,
            squad,
            display_name,
            file_name,
            no_export,
            ..
        } = &*self.borrow();

        let no_export = no_export
            .borrow()
            .as_ref()
            .map(|no_export_data| no_export_data.children())
            .unwrap_or_else(|| vec![html! { "Export Save" }])
            .into_iter();

        html! {
            <Table>
                { character_id.view("Character Id") }
                { created_date.view("Created Date") }
                { plot.view("Plot") }
                { timestamp.view("Timestamp") }
                { seconds_played.view("Seconds Played") }
                { player.view("Player") }
                { base_level_name.view("Base Level Name") }
                { map_name.view("Map Name") }
                { parent_map_name.view("Parent Map Name") }
                { location.view("Location") }
                { rotation.view("Rotation") }
                { squad.view("Squad") }
                { display_name.view("Display Name") }
                { file_name.view("File Name") }
                { for no_export }
            </Table>
        }
    }
}

impl RawUi for RcUi<BaseObject> {
    fn view(&self, label: &str) -> yew::Html {
        let BaseObject { _class_name, owner_name, owner_class, _object } = &*self.borrow();

        let object_children = match _object {
            Object::PawnBehavior(pawn_behavior) => pawn_behavior.children(),
            Object::Pawn(pawn) => pawn.children(),
            Object::BaseSquad(squad) => squad.children(),
            Object::Shop(shop) => shop.children(),
            Object::Inventory(inventory) => inventory.children(),
            Object::Item(item) => item.children(),
            Object::ItemMod(item_mod) => item_mod.children(),
            Object::ArtPlaceableBehavior(art_placeable_behavior) => {
                art_placeable_behavior.children()
            }
            Object::ArtPlaceable(art_placeable) => art_placeable.children(),
            Object::VehicleBehavior(vehicle_behavior) => vehicle_behavior.children(),
            Object::Vehicle(vehicle) => vehicle.children(),
            Object::World(world) => world.children(),
            Object::Default => unreachable!(),
        };

        html! {
            <RawUiStruct label=label.to_owned()>
                <div class="flex-auto flex items-center gap-1">
                    <span class="w-2/3">{ &_class_name }</span>
                    { "Class Name" }
                </div>
                { owner_name.view("Owner Name") }
                { owner_class.view("Owner Class") }
                { for object_children }
            </RawUiStruct>
        }
    }
}

impl RawUi for RcUi<HasObject> {
    fn view(&self, label: &str) -> yew::Html {
        self.borrow().has_object.view(label)
    }
}
