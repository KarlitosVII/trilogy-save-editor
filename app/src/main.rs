#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, windows_subsystem = "console")]
#![warn(clippy::all)]

mod rpc;

use anyhow::Result;
use clap::{Arg, ArgMatches};
use rust_embed::RustEmbed;
use wry::{
    application::{
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

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

    let event_loop = EventLoop::<rpc::Event>::with_user_event();
    let proxy = event_loop.create_proxy();
    let window = WindowBuilder::new()
        .with_title(format!("Trilogy Save Editor - v{} by Karlitos", env!("CARGO_PKG_VERSION")))
        .with_min_inner_size(LogicalSize::new(600, 300))
        .with_inner_size(LogicalSize::new(1000, 700))
        .with_visible(false)
        .with_decorations(false)
        .build(&event_loop)?;

    let mut last_maximized_state = window.is_maximized();

    let webview = WebViewBuilder::new(window)?
        .with_initialization_script(include_str!("initialization.js"))
        .with_rpc_handler(move |window, req| {
            rpc::rpc_handler(req, rpc::RpcUtils { window, event_proxy: &proxy, args: &args })
        })
        .with_custom_protocol(String::from("tse"), protocol)
        .with_url("tse://localhost/")?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(_) => {
                    let _ = webview.resize();
                    let is_maximized = webview.window().is_maximized();
                    if is_maximized != last_maximized_state {
                        last_maximized_state = is_maximized;
                        let _ = webview.evaluate_script(&format!(
                            r#"
                            (() => {{
                                const event = new CustomEvent("maximized_state_changed", {{
                                    detail: {{
                                        is_maximized: {},
                                    }}
                                }});
                                document.dispatchEvent(event);
                            }})();
                            "#,
                            is_maximized
                        ));
                    }
                }
                _ => (),
            },
            Event::UserEvent(event) => rpc::event_handler(event, &webview, control_flow),
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
