// #![cfg_attr(not(test), windows_subsystem = "windows")]
// #![cfg_attr(test, windows_subsystem = "console")]
#![warn(clippy::all)]

#[macro_use]
extern crate derive_more;

#[macro_use]
extern crate macros;

mod gui;
mod save_data;
pub mod services;
mod unreal;

use gui::App;

fn main() {
    yew::start_app::<App>();
}
