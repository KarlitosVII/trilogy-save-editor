#![warn(clippy::all)]

#[macro_use]
extern crate derive_more;

#[macro_use]
extern crate macros;

mod gui;
mod save_data;
mod services;
mod unreal;

use gui::App;

fn main() {
    let document = gloo::utils::document();
    let body = document.body().unwrap();
    let mount_point = body.last_element_child().unwrap();

    document.get_element_by_id("title").unwrap().set_text_content(Some(&format!(
        "Trilogy Save Editor - v{} by Karlitos",
        env!("CARGO_PKG_VERSION")
    )));

    yew::start_app_in_element::<App>(mount_point);
}
