use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Error, Result};
use serde_json::{json, Value};
use wry::application::window::Window;
use wry::webview::{RpcRequest, RpcResponse};

pub fn rpc_handler(window: &Window, mut req: RpcRequest) -> Option<RpcResponse> {
    let mut handle_request = || {
        let response = match req.method.as_str() {
            "init" => {
                window.set_visible(true);
                None
            }
            "open" => match dialog::open(window) {
                Some(path) => open_file(&path).map(Some)?,
                None => None,
            },
            "reload" => {
                let path: [PathBuf; 1] =
                    serde_json::from_value(req.params.take().context("Save path required")?)?;
                open_file(&path[0]).map(Some)?
            }
            "load_database" => {
                let path: [PathBuf; 1] =
                    serde_json::from_value(req.params.take().context("Database path required")?)?;
                open_file(&path[0]).map(Some)?
            }
            _ => None,
        };
        Ok::<_, Error>(response)
    };

    match handle_request() {
        Ok(None) => None,
        Ok(Some(response)) => Some(RpcResponse::new_result(req.id.take(), Some(response))),
        Err(error) => Some(RpcResponse::new_error(req.id.take(), Some(json!(error.to_string())))),
    }
}

fn open_file(path: &Path) -> Result<Value> {
    let path = path.canonicalize()?;
    let file = fs::read(&path)?;
    let unencoded_size = file.len();
    let base64 = base64::encode(file);
    Ok(json!({
        "path": path,
        "file": {
            "unencoded_size": unencoded_size,
            "base64": base64
        }
    }))
}

mod dialog {
    use std::path::PathBuf;
    use wry::application::window::Window;

    pub fn open(window: &Window) -> Option<PathBuf> {
        rfd::FileDialog::new()
            .set_parent(window)
            .set_directory(document_dir())
            .add_filter("Mass Effect Trilogy Save", &["pcsav", "ps4sav", "MassEffectSave"])
            .add_filter("All Files", &["*"])
            .pick_file()
    }

    #[cfg(target_os = "windows")]
    fn document_dir() -> PathBuf {
        match dirs::document_dir() {
            Some(mut path) => {
                path.push("BioWare\\");
                path
            }
            None => PathBuf::default(),
        }
    }

    // FIXME: Find some nicer way of finding where the game saves are.
    // Currently, this should be universal for everyone who has their
    // Mass Effect games installed in the default steam library, in
    // the user's home directory.
    #[cfg(target_os = "linux")]
    fn document_dir() -> PathBuf {
        match dirs::home_dir() {
            Some(mut path) => {
                path.push(".steam/root/steamapps/compatdata/1328670/pfx/drive_c/users/steamuser/My Documents/BioWare/");
                path
            }
            None => PathBuf::default(),
        }
    }

    #[cfg(all(not(target_os = "linux"), not(target_os = "windows")))]
    fn document_dir() -> PathBuf {
        PathBuf::default()
    }
}
