#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, windows_subsystem = "console")]
#![warn(clippy::all)]

mod rpc_handler;
use self::rpc_handler::rpc_handler;

use std::io;

use anyhow::Result;
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

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Trilogy Save Editor - by Karlitos")
        .with_min_inner_size(LogicalSize::new(480, 270))
        .with_inner_size(LogicalSize::new(1000, 670))
        .with_visible(false)
        .build(&event_loop)?;

    let webview = WebViewBuilder::new(window)?
        .with_initialization_script(include_str!("initialization.js"))
        .with_rpc_handler(rpc_handler)
        .with_custom_protocol(String::from("tse"), protocol)
        .with_url("tse://_/")?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
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

fn protocol(path: &str) -> wry::Result<(Vec<u8>, String)> {
    let path = path.trim_start_matches("tse://_/");
    let path = if !path.is_empty() { path } else { "index.html" };

    let mime = mime_guess::from_path(path).first_or_octet_stream().to_string();
    let content = Asset::get(path).map(|file| file.data.into_owned()).ok_or_else(|| {
        wry::Error::Io(io::Error::new(io::ErrorKind::Other, "Error loading requested file"))
    })?;
    Ok((content, mime))
}
