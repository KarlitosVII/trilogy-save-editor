use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

use super::{dialog, Event, RpcUtils};

// Commands
pub fn init(utils: &RpcUtils) {
    utils.window.set_visible(true);
}

pub fn minimize(utils: &RpcUtils) {
    utils.window.set_minimized(true);
}

pub fn toggle_maximize(utils: &RpcUtils) {
    let is_maximized = utils.window.is_maximized();
    utils.window.set_maximized(!is_maximized);
}

pub fn drag_window(utils: &RpcUtils) {
    let _ = utils.window.drag_window();
}

pub fn close(utils: &RpcUtils) {
    let _ = utils.event_proxy.send_event(Event::CloseWindow);
}

#[cfg(target_os = "windows")]
pub fn check_for_update(utils: &RpcUtils) -> Result<()> {
    use crate::windows::auto_update::AUTO_UPDATE;

    let proxy = utils.event_proxy.clone();
    tokio::spawn(async move {
        AUTO_UPDATE.check_for_update(proxy).await;
    });

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn download_and_install_update(utils: &RpcUtils) -> Result<()> {
    use crate::windows::auto_update::AUTO_UPDATE;

    let proxy = utils.event_proxy.clone();
    tokio::spawn(async move {
        AUTO_UPDATE.download_and_install(proxy).await;
    });

    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn check_for_update(_: &RpcUtils) -> Result<()> {
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn download_and_install_update(_: &RpcUtils) -> Result<()> {
    Ok(())
}

pub fn open_external_link(_: &RpcUtils, link: PathBuf) -> Result<()> {
    opener::open(link).map_err(Error::from)
}

pub fn save_file(_: &RpcUtils, rpc_file: RpcFile) -> Result<()> {
    write_file(rpc_file)
}

pub fn open_save(utils: &RpcUtils) -> Result<Option<RpcFile>> {
    match dialog::open_save(utils.window) {
        Some(path) => open_file(path).map(Some),
        None => Ok(None),
    }
}

pub fn save_save_dialog(utils: &RpcUtils, params: DialogParams) -> Result<Option<PathBuf>> {
    let result = dialog::save_save(utils.window, params);
    Ok(result)
}

pub fn reload_save(_: &RpcUtils, path: PathBuf) -> Result<RpcFile> {
    open_file(path)
}

pub fn import_head_morph(utils: &RpcUtils) -> Result<Option<RpcFile>> {
    match dialog::import_head_morph(utils.window) {
        Some(path) => open_file(path).map(Some),
        None => Ok(None),
    }
}

pub fn export_head_morph_dialog(utils: &RpcUtils) -> Result<Option<PathBuf>> {
    let result = dialog::export_head_morph(utils.window);
    Ok(result)
}

pub fn load_database(_: &RpcUtils, path: PathBuf) -> Result<RpcFile> {
    #[cfg(not(debug_assertions))]
    let path = std::env::current_exe()?.parent().map(|parent| parent.join(&path)).unwrap_or(path);

    open_file(path)
}

// Utils
fn open_file(path: PathBuf) -> Result<RpcFile> {
    let file = fs::read(path.canonicalize()?)?;
    let unencoded_size = file.len();
    let base64 = base64::encode(file);
    Ok(RpcFile { path, file: Base64File { unencoded_size, base64 } })
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

#[derive(Deserialize, Default)]
pub struct DialogParams {
    pub path: PathBuf,
    pub filters: Vec<(String, Vec<String>)>,
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
