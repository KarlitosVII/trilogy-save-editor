use std::env;

use anyhow::Error;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::Deserialize;
use serde_json::json;
use tokio::{fs, process};
use wry::application::event_loop::EventLoopProxy;

use crate::rpc;

const GITHUB_API: &str =
    "https://api.github.com/repos/KarlitosVII/trilogy-save-editor/releases/latest";

#[derive(Deserialize, Debug)]
struct GithubResponse {
    tag_name: String,
    prerelease: bool,
    assets: Vec<GithubAsset>,
}

#[derive(Deserialize, Debug)]
struct GithubAsset {
    // id: usize,
    name: String,
    // content_type: String,
    browser_download_url: String,
    size: usize,
}

lazy_static! {
    pub static ref AUTO_UPDATE: AutoUpdate = AutoUpdate::new();
    static ref REQWEST: reqwest::Client = {
        reqwest::Client::builder()
            .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),))
            .build()
            .expect("Failed to initialize http client")
    };
}

pub struct AutoUpdate {
    update_available: Mutex<Option<GithubAsset>>,
}

impl AutoUpdate {
    fn new() -> Self {
        AutoUpdate { update_available: Mutex::new(None) }
    }

    pub async fn check_for_update(&self, proxy: EventLoopProxy<rpc::Event>) {
        let result = async {
            let response = REQWEST.get(GITHUB_API).send().await?.json().await?;
            let GithubResponse { tag_name, prerelease, assets } = response;

            if !prerelease && tag_name.trim_start_matches('v') != env!("CARGO_PKG_VERSION") {
                if let Some(update_available) =
                    assets.into_iter().find(|asset| asset.name.ends_with("setup.exe"))
                {
                    *self.update_available.lock() = Some(update_available);
                    let _ = proxy.send_event(rpc::Event::DispatchCustomEvent(
                        "tse_update_available",
                        json!({}),
                    ));
                }
            }
            Ok::<_, Error>(())
        };

        if let Err(err) = result.await {
            let _ = proxy.send_event(rpc::Event::DispatchCustomEvent(
                "tse_update_error",
                json!({ "error": err.to_string() }),
            ));
        }
    }

    pub async fn download_and_install(&self, proxy: EventLoopProxy<rpc::Event>) {
        let asset = self.update_available.lock().take();
        if let Some(GithubAsset { name, browser_download_url, size }) = asset {
            let result = async {
                let send_progress = |progress: f64| {
                    let _ = proxy.send_event(rpc::Event::DispatchCustomEvent(
                        "tse_update_progress",
                        json!({ "progress": progress }),
                    ));
                };

                // Download
                let mut response = REQWEST.get(browser_download_url).send().await?;
                let mut setup = Vec::with_capacity(size);

                send_progress(0.0);
                let size = size as f64;
                while let Some(chunk) = response.chunk().await? {
                    setup.extend(chunk);
                    send_progress(setup.len() as f64 / size);
                }

                // Install
                let temp_dir = env::temp_dir().join("trilogy-save-editor");
                let path = temp_dir.join(name);

                // If not exists
                if fs::metadata(&temp_dir).await.is_err() {
                    fs::create_dir(temp_dir).await?;
                }
                fs::write(&path, setup).await?;

                process::Command::new(path).arg("/SILENT").arg("/NOICONS").spawn()?;

                let _ = proxy.send_event(rpc::Event::CloseWindow);

                Ok::<_, Error>(())
            };

            if let Err(err) = result.await {
                let _ = proxy.send_event(rpc::Event::DispatchCustomEvent(
                    "tse_update_error",
                    json!({ "error": err.to_string() }),
                ));
            }
        }
    }
}
