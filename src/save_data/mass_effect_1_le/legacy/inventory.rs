use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::save_data::mass_effect_1_le::player::ItemLevel;

use super::{BaseObject, HasObject};

#[rcize_fields_derive(RawUiMe1Legacy)]
#[derive(Deserialize, Serialize, Clone)]
pub struct Shop {
    last_player_level: i32,
    is_initialized: bool,
    inventory: Vec<HasObject>,
}

#[rcize_fields_derive(RawUiMe1Legacy)]
#[derive(Deserialize, Serialize, Clone)]
pub struct Inventory {
    items: Vec<BaseObject>,
    plot_items: Vec<PlotItem>,
    credits: i32,
    grenades: i32,
    medigel: f32,
    omnigel: f32,
}

#[rcize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone, Default, Display)]
#[display(fmt = "")]
struct PlotItem {
    localized_name: i32,
    localized_desc: i32,
    export_id: i32,
    base_price: i32,
    shop_gui_image_id: i32,
    plot_conditional_id: i32,
}

#[rcize_fields_derive(RawUiMe1Legacy)]
#[derive(Deserialize, Serialize, Clone)]
pub struct Item {
    item_id: i32,
    item_level: ItemLevel,
    manufacturer_id: i32,
    plot_conditional_id: i32,
    slot_specs: Vec<ModdableSlotSpec>,
}

#[rcize_fields_derive(RawUi)]
#[derive(Deserialize, Serialize, Clone, Default, Display)]
#[display(fmt = "")]
struct ModdableSlotSpec {
    type_id: i32,
    mods: Vec<HasObject>,
}

#[rcize_fields_derive(RawUiMe1Legacy)]
#[derive(Deserialize, Serialize, Clone)]
pub struct ItemMod {
    item_id: i32,
    item_level: ItemLevel,
    manufacturer_id: i32,
    plot_conditional_id: i32,
    type_id: i32,
}
