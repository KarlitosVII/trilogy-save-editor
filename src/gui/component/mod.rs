mod check_box;
mod clipper;
mod input_number;
mod input_text;
mod nav_bar;
mod raw_ui;
mod select;
mod tab_bar;
mod table;

pub use self::{
    check_box::*, clipper::*, input_number::*, input_text::*, nav_bar::*, raw_ui::*, select::*,
    tab_bar::*, table::*,
};

pub enum CallbackType {
    Byte(u8),
    Integer(i32),
    Float(f32),
    String(String),
}
