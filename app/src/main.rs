#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, windows_subsystem = "console")]
#![warn(clippy::all)]

#[cfg(target_os = "windows")]
mod windows;

mod rpc;

use anyhow::Result;
use clap::{Arg, ArgMatches};
use image::GenericImageView;
use rust_embed::RustEmbed;
use serde_json::json;
use wry::{
    application::{
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Icon, WindowBuilder},
    },
    http::{self, status::StatusCode},
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

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        // Install WebView2
        let should_install_webview2 = std::panic::catch_unwind(|| {
            wry::webview::webview_version().expect("Unable to get webview2 version")
        })
        .is_err();
        if should_install_webview2 {
            if let Err(err) = windows::install_webview2().await {
                anyhow::bail!(err)
            }
        }
    }

    let args = parse_args();

    let event_loop = EventLoop::<rpc::Event>::with_user_event();
    let window = WindowBuilder::new()
        .with_title(format!("Trilogy Save Editor - v{} by Karlitos", env!("CARGO_PKG_VERSION")))
        .with_window_icon(load_icon())
        .with_min_inner_size(LogicalSize::new(600, 300))
        .with_inner_size(LogicalSize::new(1000, 700))
        .with_visible(false)
        .with_decorations(false)
        .build(&event_loop)?;

    let mut last_maximized_state = window.is_maximized();

    let proxy = event_loop.create_proxy();
    let webview = WebViewBuilder::new(window)?
        .with_initialization_script(include_str!("init.js"))
        .with_rpc_handler(move |window, req| {
            rpc::rpc_handler(req, rpc::RpcUtils { window, event_proxy: &proxy, args: &args })
        })
        .with_custom_protocol(String::from("tse"), protocol)
        .with_url("tse://localhost/")?
        .build()?;

    let proxy = event_loop.create_proxy();
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
                        let _ = proxy.send_event(rpc::Event::DispatchCustomEvent(
                            "tse_maximized_state_changed",
                            json!({ "is_maximized": is_maximized }),
                        ));
                    }
                }
                _ => (),
            },
            Event::UserEvent(event) => rpc::event_handler(event, &webview, control_flow),
            Event::LoopDestroyed => {
                // Clear WebView2 Code Cache
                #[cfg(target_os = "windows")]
                windows::clear_code_cache();
            }
            _ => (),
        }
    });

    #[allow(unreachable_code)]
    Ok(())
}

fn protocol(request: &http::Request) -> wry::Result<http::Response> {
    let mut path = request.uri().trim_start_matches("tse://localhost/");
    if path.is_empty() {
        path = "index.html"
    }

    let response = http::ResponseBuilder::new();
    match Asset::get(path) {
        Some(asset) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream().to_string();
            response.mimetype(&mime).body(asset.data.into())
        }
        None => response.status(StatusCode::NOT_FOUND).body(vec![]),
    }
}

fn load_icon() -> Option<Icon> {
    let image = image::load_from_memory(include_bytes!("../../misc/tse.png")).unwrap();
    let (width, height) = image.dimensions();
    let rgba = image.into_rgba8().into_raw();
    Some(Icon::from_rgba(rgba, width, height).unwrap())
}
