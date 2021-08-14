use std::path::{Path, PathBuf};
use std::{fs, mem};

use anyhow::{bail, Context, Error, Result};
use serde::Deserialize;
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
            "save_file" => {
                let params = req.params.take().context("RpcFile required")?;
                let mut path: [RpcFile; 1] = serde_json::from_value(params)?;
                let rpc_file = mem::take(&mut path[0]);
                save_file(rpc_file).map(Some)?
            }
            "open_save" => match dialog::open_save(window) {
                Some(path) => open_file(&path).map(Some)?,
                None => Some(Value::Null),
            },
            "save_save_dialog" => {
                let params = req.params.take().context("Save path required")?;
                let path: [PathBuf; 1] = serde_json::from_value(params)?;

                match dialog::save_save(window, &path[0]) {
                    Some(path) => Some(serde_json::to_value(path)?),
                    None => Some(Value::Null),
                }
            }
            "reload_save" => {
                let params = req.params.take().context("Save path required")?;
                let path: [PathBuf; 1] = serde_json::from_value(params)?;
                open_file(&path[0]).map(Some)?
            }
            "import_head_morph" => match dialog::import_head_morph(window) {
                Some(path) => open_file(&path).map(Some)?,
                None => Some(Value::Null),
            },
            "export_head_morph_dialog" => match dialog::export_head_morph(window) {
                Some(path) => Some(serde_json::to_value(path)?),
                None => Some(Value::Null),
            },
            "load_database" => {
                let params = req.params.take().context("Database path required")?;
                let path: [PathBuf; 1] = serde_json::from_value(params)?;
                open_file(&path[0]).map(Some)?
            }
            _ => bail!("Wrong RPC method, got: {}", req.method),
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
    let file = fs::read(path.canonicalize()?)?;
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

fn save_file(rpc_file: RpcFile) -> Result<Value> {
    let RpcFile { path, file } = rpc_file;

    // Backup if file exists
    if path.exists() {
        if let Some(ext) = path.extension() {
            let mut ext = ext.to_owned();
            ext.push(".bak");
            let to = Path::with_extension(&path, ext);
            fs::copy(&path, to)?;
        }
    }
    fs::write(path, file.decode()?)?;

    Ok(Value::from(()))
}

#[derive(Deserialize, Default)]
pub struct RpcFile {
    pub path: PathBuf,
    pub file: Base64File,
}

#[derive(Deserialize, Default)]
pub struct Base64File {
    unencoded_size: usize,
    base64: String,
}

impl Base64File {
    pub fn decode(self) -> Result<Vec<u8>> {
        let mut vec = Vec::with_capacity(self.unencoded_size);
        base64::decode_config_buf(self.base64, base64::STANDARD, &mut vec)?;
        Ok(vec)
    }
}

mod dialog {
    use std::path::{Path, PathBuf};
    use wry::application::window::Window;

    pub fn open_save(window: &Window) -> Option<PathBuf> {
        rfd::FileDialog::new()
            .set_parent(window)
            .set_directory(document_dir())
            .add_filter("Mass Effect Trilogy Save", &["pcsav", "ps4sav", "MassEffectSave"])
            .add_filter("All Files", &["*"])
            .pick_file()
    }

    pub fn save_save(window: &Window, path: &Path) -> Option<PathBuf> {
        let directory = path.parent().map(ToOwned::to_owned).unwrap_or_default();
        let file_name = path
            .file_name()
            .map(ToOwned::to_owned)
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();

        // TODO: Filter by game
        rfd::FileDialog::new()
            .set_parent(window)
            .set_directory(directory)
            .set_file_name(&file_name)
            .add_filter("Mass Effect Trilogy Save", &["pcsav", "ps4sav", "MassEffectSave"])
            .add_filter("All Files", &["*"])
            .save_file()
    }

    pub fn import_head_morph(window: &Window) -> Option<PathBuf> {
        rfd::FileDialog::new()
            .set_parent(window)
            .add_filter("Head Morph", &["ron"])
            .add_filter("All Files", &["*"])
            .pick_file()
    }

    pub fn export_head_morph(window: &Window) -> Option<PathBuf> {
        rfd::FileDialog::new()
            .set_parent(window)
            .add_filter("Head Morph", &["ron"])
            .add_filter("All Files", &["*"])
            .save_file()
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
