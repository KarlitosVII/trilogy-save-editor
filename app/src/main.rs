#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, windows_subsystem = "console")]
#![warn(clippy::all)]

#[cfg(target_os = "windows")]
mod auto_update;

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
        use tokio::fs;

        // Install WebView2
        let should_install_webview2 = std::panic::catch_unwind(|| {
            wry::webview::webview_version().expect("Unable to get webview2 version")
        })
        .is_err();
        if should_install_webview2 {
            if let Err(err) = install_webview2().await {
                anyhow::bail!(err)
            }
        }

        // Clear WebView2 Code Cache
        let code_cache_dir =
            concat!(env!("CARGO_BIN_NAME"), ".exe.WebView2/EBWebView/Default/Code Cache");
        if fs::metadata(code_cache_dir).await.is_ok() {
            let _ = fs::remove_dir_all(code_cache_dir).await;
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
            _ => (),
        }
    });
}

fn protocol(request: &http::Request) -> wry::Result<http::Response> {
    let mut path = request.uri().trim_start_matches("tse://localhost/");
    if path.is_empty() {
        path = "index.html"
    }

    let response = http::ResponseBuilder::new()
        // Prevent caching
        // .header("Cache-Control", "max-age=0, no-cache, no-store, must-revalidate")
        // .header("Expires", "Thu, 01 Jan 1970 00:00:00 GMT")
        // .header("Pragma", "no-cache")
        ;

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

#[cfg(target_os = "windows")]
async fn install_webview2() -> Result<()> {
    use std::env;
    use tokio::{fs, process};

    let should_install = rfd::AsyncMessageDialog::new()
        .set_title("Install WebView2 Runtime")
        .set_description("The WebView2 Runtime must be installed to use this program. Install now?")
        .set_level(rfd::MessageLevel::Warning)
        .set_buttons(rfd::MessageButtons::YesNo)
        .show()
        .await;

    if !should_install {
        anyhow::bail!("WebView2 install cancelled by user");
    }

    let setup =
        reqwest::get("https://go.microsoft.com/fwlink/p/?LinkId=2124703").await?.bytes().await?;

    let temp_dir = env::temp_dir().join("trilogy-save-editor");
    let path = temp_dir.join("MicrosoftEdgeWebview2Setup.exe");

    // If not exists
    if fs::metadata(&temp_dir).await.is_err() {
        fs::create_dir(temp_dir).await?;
    }
    fs::write(&path, setup).await?;

    let status = process::Command::new(path)
        .arg("/install")
        .status()
        .await
        .expect("Failed to launch WebView2 install");

    if !status.success() {
        anyhow::bail!("Failed to install WebView2");
    }
    Ok(())
}
