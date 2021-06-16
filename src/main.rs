#![cfg_attr(not(test), windows_subsystem = "windows")]
#![cfg_attr(test, windows_subsystem = "console")]
#![warn(clippy::all)]

extern crate derive_more;

use clap::{Arg, ArgMatches};
use std::{
    panic::{self, PanicInfo},
    process,
};
use tokio::task;

#[macro_use]
extern crate raw_ui_derive;

mod compare;
mod databases;
mod event_handler;
mod gui;
mod save_data;
mod unreal;

fn parse_args() -> ArgMatches<'static> {
    let app = clap::App::new("Trilogy Save Editor")
        .version(env!("CARGO_PKG_VERSION"))
        .author("by Karlitos")
        .about("A save editor for Mass Effect Trilogy (and Legendary)")
        .arg(Arg::with_name("Vulkan").long("vulkan").help("Use Vulkan backend"));

    // Backends
    #[cfg(target_os = "windows")]
    let app = app
        .arg(Arg::with_name("DirectX12").long("dx12").help("Use DirectX 12 backend"))
        .arg(Arg::with_name("DirectX11").long("dx11").help("Use DirectX 11 backend"));

    #[cfg(target_os = "macos")]
    let app = app.arg(Arg::with_name("Metal").long("metal").help("Use Metal backend"));

    // Compare
    let app = app.arg(
        Arg::with_name("Compare")
            .short("c")
            .long("compare")
            .value_name("OTHER_SAVE")
            .requires("SAVE")
            .help("Compare `SAVE` and `OTHER_SAVE` plots and generate a `compare_result.ron` (Rusty Object Notation) file"),
    );

    // File
    let app = app.arg(Arg::with_name("SAVE").help("Mass Effect save file"));

    app.get_matches()
}

#[tokio::main]
async fn main() {
    #[cfg(target_os = "windows")]
    console::attach_if_run_in_console();

    panic::set_hook(Box::new(|e| {
        #[cfg(target_os = "windows")]
        console::attach();
        panic_hook(e);
    }));

    let args = parse_args();

    let (event_addr, event_rx) = flume::unbounded();
    let (ui_addr, ui_rx) = flume::unbounded();

    let event_loop = tokio::spawn(event_handler::event_loop(event_rx, ui_addr));

    if let Some(cmp) = args.value_of("Compare") {
        let src = args.value_of("SAVE").unwrap();
        if let Err(err) = compare::compare(event_addr, ui_rx, src, cmp).await {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    } else {
        task::block_in_place(move || gui::run(event_addr, ui_rx, args));
    }

    event_loop.await.unwrap();
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

#[cfg(target_os = "windows")]
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

    pub fn attach_if_run_in_console() -> bool {
        unsafe { AttachConsole(ATTACH_PARENT_PROCESS) != FALSE }
    }
}
