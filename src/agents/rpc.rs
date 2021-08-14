use std::path::PathBuf;

use anyhow::{anyhow, Result};
use js_sys::JsString;
use serde::{de, Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/agents/rpc.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn js_open() -> Result<JsValue, JsString>;

    #[wasm_bindgen(catch)]
    async fn js_save_dialog(path: String) -> Result<JsValue, JsString>;

    #[wasm_bindgen(catch)]
    async fn js_save(rpc_file: JsValue) -> Result<(), JsString>;

    #[wasm_bindgen(catch)]
    async fn js_reload(path: String) -> Result<JsValue, JsString>;

    #[wasm_bindgen(catch)]
    async fn js_load_database(path: &str) -> Result<JsValue, JsString>;
}

pub async fn open() -> Result<RpcFile> {
    js_open().await.map(from_js_value).map_err(|e| anyhow!(String::from(e)))?
}

pub async fn save_dialog(path: PathBuf) -> Result<Option<PathBuf>> {
    js_save_dialog(path.to_string_lossy().into_owned())
        .await
        .map(from_js_value)
        .map_err(|e| anyhow!(String::from(e)))?
}

pub async fn save(rpc_file: RpcFile) -> Result<()> {
    let js_rpc_file =
        serde_wasm_bindgen::to_value(&rpc_file).map_err(|e| anyhow!(e.to_string()))?;
    js_save(js_rpc_file).await.map_err(|e| anyhow!(String::from(e)))
}

pub async fn reload(path: PathBuf) -> Result<RpcFile> {
    js_reload(path.to_string_lossy().into_owned())
        .await
        .map(from_js_value)
        .map_err(|e| anyhow!(String::from(e)))?
}

pub async fn load_database(path: &str) -> Result<RpcFile> {
    js_load_database(path).await.map(from_js_value).map_err(|e| anyhow!(String::from(e)))?
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
    pub fn into_string(self) -> Result<String> {
        let mut vec = Vec::with_capacity(self.unencoded_size);
        base64::decode_config_buf(self.base64, base64::STANDARD, &mut vec)?;
        String::from_utf8(vec).map_err(Into::into)
    }

    pub fn into_bytes(self) -> Result<Vec<u8>> {
        let mut vec = Vec::with_capacity(self.unencoded_size);
        base64::decode_config_buf(self.base64, base64::STANDARD, &mut vec)?;
        Ok(vec)
    }
}
