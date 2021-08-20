#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, windows_subsystem = "console")]
#![warn(clippy::all)]

mod rpc_handler;
use self::rpc_handler::rpc_handler;

use anyhow::Result;
use clap::{Arg, ArgMatches};
use rust_embed::RustEmbed;
use wry::application::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::webview::WebViewBuilder;

#[derive(RustEmbed)]
#[folder = "../target/dist/"]
struct Asset;

fn parse_args() -> ArgMatches<'static> {
    let app = clap::App::new("Trilogy Save Editor")
        .version(env!("CARGO_PKG_VERSION"))
        .author("by Karlitos")
        .about("A save editor for Mass Effect Trilogy (and Legendary)")
        .arg(Arg::with_name("SAVE").help("Mass Effect save file"));

    app.get_matches()
}

fn main() -> Result<()> {
    let args = parse_args();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Trilogy Save Editor - by Karlitos")
        .with_min_inner_size(LogicalSize::new(480, 270))
        .with_inner_size(LogicalSize::new(1000, 670))
        .with_visible(false)
        .build(&event_loop)?;

    let webview = WebViewBuilder::new(window)?
        .with_initialization_script(include_str!("initialization.js"))
        .with_rpc_handler(move |window, req| rpc_handler(window, req, &args))
        .with_custom_protocol(String::from("tse"), protocol)
        .with_url("tse://localhost/")?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        #[allow(clippy::single_match)]
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(_) => {
                    let _ = webview.resize();
                }
                _ => (),
            },
            _ => (),
        }
    });
}

fn protocol(mut path: &str) -> wry::Result<(Vec<u8>, String)> {
    path = path.trim_start_matches("tse://localhost/");
    if path.is_empty() {
        path = "index.html"
    }

    let mime = mime_guess::from_path(path).first_or_octet_stream().to_string();
    let content = Asset::get(path).map(|file| file.data.into_owned()).unwrap_or_default();
    Ok((content, mime))
}
