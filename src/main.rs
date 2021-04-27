#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]
#![warn(clippy::all)]

#[macro_use]
extern crate raw_ui_derive;

use std::panic::{self, PanicInfo};
use tokio::task;

mod event_handler;
mod gui;
mod save_data;
mod unreal;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    console::attach();

    panic::set_hook(Box::new(|e| {
        console::attach();
        panic_hook(e);
    }));

    let (event_addr, event_rx) = flume::unbounded();
    let (ui_addr, ui_rx) = flume::unbounded();

    let event_loop = event_handler::event_loop(event_rx, ui_addr);

    let gui = task::spawn_blocking(move || gui::run(event_addr, ui_rx));

    let (_, gui) = tokio::join!(event_loop, gui);
    gui.expect("GUI panicked");
}

fn panic_hook(info: &PanicInfo<'_>) {
    let msg = match info.payload().downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "???",
        },
    };
    let location = info.location().unwrap();

    eprintln!("Panic : '{}', {}", msg, location);
}

mod console {
    use winapi::{
        shared::minwindef::FALSE,
        um::{
            consoleapi::AllocConsole,
            wincon::{AttachConsole, ATTACH_PARENT_PROCESS},
        },
    };

    pub fn attach() {
        unsafe {
            if AttachConsole(ATTACH_PARENT_PROCESS) == FALSE {
                AllocConsole();
            }
        }
    }
}
