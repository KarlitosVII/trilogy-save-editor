// #![cfg_attr(not(test), windows_subsystem = "windows")]
// #![cfg_attr(test, windows_subsystem = "console")]
#![warn(clippy::all)]

extern crate derive_more;

#[macro_use]
extern crate raw_ui_derive;

// mod compare;
// mod event_handler;
mod agents;
mod gui;
mod save_data;
mod unreal;

pub use self::agents::*;

use gui::App;

fn main() {
    yew::start_app::<App>();
}
