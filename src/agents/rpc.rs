use std::path::PathBuf;

use anyhow::{anyhow, Result};
use js_sys::JsString;
use serde::{de, Deserialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/agents/rpc.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn js_open() -> Result<JsValue, JsString>;

    #[wasm_bindgen(catch)]
    async fn js_save_dialog(path: String) -> Result<JsValue, JsString>;

    #[wasm_bindgen(catch)]
    async fn js_save(path: String, unencoded_size: usize, base64: String) -> Result<(), JsString>;

    #[wasm_bindgen(catch)]
    async fn js_reload(path: String) -> Result<JsValue, JsString>;

    #[wasm_bindgen(catch)]
    async fn js_load_database(path: &str) -> Result<JsValue, JsString>;
}

pub async fn open() -> Result<RpcFile> {
    js_open().await.map(into_serde).map_err(|e| anyhow!(String::from(e)))?
}

pub async fn save_dialog(path: PathBuf) -> Result<Option<PathBuf>> {
    js_save_dialog(path.to_string_lossy().into_owned())
        .await
        .map(into_serde)
        .map_err(|e| anyhow!(String::from(e)))?
}

pub async fn save(rpc_file: RpcFile) -> Result<()> {
    let RpcFile { path, file } = rpc_file;
    js_save(path.to_string_lossy().into_owned(), file.unencoded_size, file.base64)
        .await
        .map_err(|e| anyhow!(String::from(e)))
}

pub async fn reload(path: PathBuf) -> Result<RpcFile> {
    js_reload(path.to_string_lossy().into_owned())
        .await
        .map(into_serde)
        .map_err(|e| anyhow!(String::from(e)))?
}

pub async fn load_database(path: &str) -> Result<RpcFile> {
    js_load_database(path).await.map(into_serde).map_err(|e| anyhow!(String::from(e)))?
}

fn into_serde<T>(js: JsValue) -> Result<T>
where
    T: for<'a> de::Deserialize<'a>,
{
    js.into_serde().map_err(Into::into)
}

#[derive(Deserialize)]
pub struct RpcFile {
    pub path: PathBuf,
    pub file: Base64File,
}

#[derive(Deserialize)]
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
