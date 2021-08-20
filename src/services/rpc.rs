use std::path::PathBuf;

use anyhow::{anyhow, Result};
use js_sys::JsString;
use serde::{de, Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/services/rpc.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn js_save_file(rpc_file: JsValue) -> Result<(), JsString>;

    #[wasm_bindgen(catch)]
    async fn js_open_file(method: &str) -> Result<JsValue, JsString>;

    #[wasm_bindgen(catch)]
    async fn js_open_file_with_path(method: &str, path: &str) -> Result<JsValue, JsString>;

    #[wasm_bindgen(catch)]
    async fn js_save_file_dialog(method: &str, path: &str) -> Result<JsValue, JsString>;
}

pub async fn save_file(rpc_file: RpcFile) -> Result<()> {
    let js_rpc_file =
        serde_wasm_bindgen::to_value(&rpc_file).map_err(|e| anyhow!(e.to_string()))?;
    js_save_file(js_rpc_file).await.map_err(|e| anyhow!(String::from(e)))
}

pub async fn open_save() -> Result<Option<RpcFile>> {
    js_open_file("open_save").await.map(from_js_value).map_err(|e| anyhow!(String::from(e)))?
}

pub async fn open_command_line_save() -> Result<Option<RpcFile>> {
    js_open_file("open_command_line_save")
        .await
        .map(from_js_value)
        .map_err(|e| anyhow!(String::from(e)))?
}

pub async fn save_save_dialog(path: PathBuf) -> Result<Option<PathBuf>> {
    js_save_file_dialog("save_save_dialog", &path.to_string_lossy())
        .await
        .map(from_js_value)
        .map_err(|e| anyhow!(String::from(e)))?
}

pub async fn reload_save(path: PathBuf) -> Result<RpcFile> {
    js_open_file_with_path("reload_save", &path.to_string_lossy())
        .await
        .map(from_js_value)
        .map_err(|e| anyhow!(String::from(e)))?
}

pub async fn import_head_morph() -> Result<Option<RpcFile>> {
    js_open_file("import_head_morph")
        .await
        .map(from_js_value)
        .map_err(|e| anyhow!(String::from(e)))?
}

pub async fn export_head_morph_dialog() -> Result<Option<PathBuf>> {
    js_save_file_dialog("export_head_morph_dialog", "")
        .await
        .map(from_js_value)
        .map_err(|e| anyhow!(String::from(e)))?
}

pub async fn load_database(path: &str) -> Result<RpcFile> {
    js_open_file_with_path("load_database", path)
        .await
        .map(from_js_value)
        .map_err(|e| anyhow!(String::from(e)))?
}

fn from_js_value<T>(js: JsValue) -> Result<T>
where
    T: for<'a> de::Deserialize<'a>,
{
    serde_wasm_bindgen::from_value(js).map_err(|e| anyhow!(e.to_string()))
}

#[derive(Deserialize, Serialize)]
pub struct RpcFile {
    pub path: PathBuf,
    pub file: Base64File,
}

#[derive(Deserialize, Serialize)]
pub struct Base64File {
    pub unencoded_size: usize,
    pub base64: String,
}

impl Base64File {
    pub fn decode(self) -> Result<Vec<u8>> {
        let mut vec = Vec::with_capacity(self.unencoded_size);
        base64::decode_config_buf(self.base64, base64::STANDARD, &mut vec)?;
        Ok(vec)
    }
}
