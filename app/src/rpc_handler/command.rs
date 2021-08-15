mod dialog;

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use wry::application::window::Window;

// Commands
pub fn init(window: &Window) {
    window.set_visible(true);
}

pub fn save_file(_: &Window, rpc_file: RpcFile) -> Result<()> {
    write_file(rpc_file)
}

pub fn open_save(window: &Window) -> Result<Option<RpcFile>> {
    match dialog::open_save(window) {
        Some(path) => open_file(path).map(Some),
        None => Ok(None),
    }
}

pub fn save_save_dialog(window: &Window, path: PathBuf) -> Result<Option<PathBuf>> {
    let result = dialog::save_save(window, path);
    Ok(result)
}

pub fn reload_save(_: &Window, path: PathBuf) -> Result<RpcFile> {
    open_file(path)
}

pub fn import_head_morph(window: &Window) -> Result<Option<RpcFile>> {
    match dialog::import_head_morph(window) {
        Some(path) => open_file(path).map(Some),
        None => Ok(None),
    }
}

pub fn export_head_morph_dialog(window: &Window) -> Result<Option<PathBuf>> {
    let result = dialog::export_head_morph(window);
    Ok(result)
}

pub fn load_database(_: &Window, path: PathBuf) -> Result<RpcFile> {
    open_file(path)
}

// Utils
fn open_file(path: PathBuf) -> Result<RpcFile> {
    let file = fs::read(path.canonicalize()?)?;
    let unencoded_size = file.len();
    let base64 = base64::encode(file);
    Ok(RpcFile { path: path.to_owned(), file: Base64File { unencoded_size, base64 } })
}

fn write_file(rpc_file: RpcFile) -> Result<()> {
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

    Ok(())
}

#[derive(Deserialize, Serialize, Default)]
pub struct RpcFile {
    pub path: PathBuf,
    pub file: Base64File,
}

#[derive(Deserialize, Serialize, Default)]
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
