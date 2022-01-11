use yew::prelude::*;

use crate::{
    gui::{
        components::{raw_ui::RawUiStruct, Table},
        raw_ui::{RawUi, RawUiChildren},
    },
    save_data::{
        mass_effect_1_le::{
            legacy::{BaseObject, Object, OptionObjectProxy},
            Me1LeSaveData, NoExport,
        },
        RcRef,
    },
};

mod bonus_talents;
mod general;
mod inventory;

pub use self::{general::*, inventory::*};

impl RawUi for RcRef<Me1LeSaveData> {
    fn view(&self, _: &str) -> yew::Html {
        let no_export = self
            .borrow()
            .no_export()
            .as_ref()
            .map(|no_export_data| no_export_data.children())
            .unwrap_or_else(|| vec![html! { "Export Save" }])
            .into_iter();

        let children = self.children();
        let len = children.len();
        html! {
            <Table>
                { for children.into_iter().take(len - 1) }
                { for no_export }
            </Table>
        }
    }
}

impl RawUi for RcRef<NoExport> {
    fn view(&self, _: &str) -> yew::Html {
        Default::default()
    }
}

impl RawUi for RcRef<BaseObject> {
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
            <RawUiStruct label={label.to_owned()}>
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

impl RawUi for RcRef<OptionObjectProxy> {
    fn view(&self, label: &str) -> yew::Html {
        self.borrow().proxy.view(label)
    }
}
