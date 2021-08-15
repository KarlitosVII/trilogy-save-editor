use serde::{Deserialize, Serialize};

use super::{BaseObject, OptionObjectProxy};
use crate::save_data::mass_effect_1_le::player::ItemLevel;

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUiChildren)]
pub struct Shop {
    last_player_level: i32,
    is_initialized: bool,
    inventory: Vec<OptionObjectProxy>,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUiChildren)]
pub struct Inventory {
    items: Vec<BaseObject>,
    plot_items: Vec<PlotItem>,
    credits: i32,
    grenades: i32,
    medigel: f32,
    omnigel: f32,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "")]
struct PlotItem {
    localized_name: i32,
    localized_desc: i32,
    export_id: i32,
    base_price: i32,
    shop_gui_image_id: i32,
    plot_conditional_id: i32,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUiChildren)]
pub struct Item {
    item_id: i32,
    item_level: ItemLevel,
    manufacturer_id: i32,
    plot_conditional_id: i32,
    slot_specs: Vec<ModdableSlotSpec>,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, Default, Display, RawUi)]
#[display(fmt = "")]
struct ModdableSlotSpec {
    type_id: i32,
    mods: Vec<OptionObjectProxy>,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUiChildren)]
pub struct ItemMod {
    item_id: i32,
    item_level: ItemLevel,
    manufacturer_id: i32,
    plot_conditional_id: i32,
    type_id: i32,
}
