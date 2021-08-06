mod check_box;
mod color_picker;
mod helper;
mod input_number;
mod input_text;
mod nav_bar;
pub mod raw_ui;
mod select;
pub mod shared;
mod tab_bar;
mod table;

pub use self::{
    check_box::*, color_picker::*, helper::*, input_number::*, input_text::*, nav_bar::*,
    select::*, tab_bar::*, table::*,
};

pub enum CallbackType {
    Byte(u8),
    Integer(i32),
    Float(f32),
    String(String),
}
