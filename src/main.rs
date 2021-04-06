#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]

#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate save_data_derive;

use anyhow::Result;
use std::{
    panic::{self, PanicInfo},
};

mod mass_effect_3;
mod serializer;
mod event_handler;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
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

    ui::run(event_addr, ui_rx, || {
        // Code de fin de programme
    });
    let _ = event_loop.await;
    Ok(())
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
    use bindings::Windows::Win32::SystemServices::{AllocConsole, AttachConsole};

    #[allow(clippy::missing_safety_doc)]
    pub fn attach() {
        unsafe {
            // u32::MAX = DWORD(-1) soit ATTACH_PARENT_PROCESS
            if !AttachConsole(u32::MAX).as_bool() {
                AllocConsole();
            }
        }
    }
}
