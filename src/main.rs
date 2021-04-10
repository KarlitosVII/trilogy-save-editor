#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]

#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate save_data_derive;

use std::panic::{self, PanicInfo};
use tokio::{
    runtime::Handle,
    task::{self, JoinError},
};

mod event_handler;
mod gui;
mod save_data;

#[tokio::main]
async fn main() -> Result<(), JoinError> {
    #[cfg(debug_assertions)]
    console::attach();

    panic::set_hook(Box::new(|e| {
        console::attach();
        panic_hook(e);
    }));

    let (event_addr, event_rx) = flume::unbounded();
    let (ui_addr, ui_rx) = flume::unbounded();

    let event_loop = tokio::spawn(async move {
        event_handler::event_loop(event_rx, ui_addr).await;
    });

    let handle = Handle::current();
    task::spawn_blocking(move || gui::run(event_addr, ui_rx, handle)).await?;

    event_loop.await
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
