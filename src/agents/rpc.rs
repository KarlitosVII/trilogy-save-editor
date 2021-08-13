use std::path::PathBuf;

use anyhow::{anyhow, Result};
use js_sys::JsString;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/agents/rpc.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn js_open() -> Result<JsValue, JsString>;

    #[wasm_bindgen(catch)]
    async fn js_reload(path: &str) -> Result<JsValue, JsString>;

    #[wasm_bindgen(catch)]
    async fn js_load_database(path: &str) -> Result<JsValue, JsString>;
}

pub async fn open() -> Result<RpcFile> {
    js_open().await.map(js_to_rpc_file).map_err(|e| anyhow!(String::from(e)))?
}

pub async fn reload(path: PathBuf) -> Result<RpcFile> {
    js_reload(&path.to_string_lossy())
        .await
        .map(js_to_rpc_file)
        .map_err(|e| anyhow!(String::from(e)))?
}

pub async fn load_database(path: &str) -> Result<RpcFile> {
    js_load_database(path).await.map(js_to_rpc_file).map_err(|e| anyhow!(String::from(e)))?
}

fn js_to_rpc_file(js: JsValue) -> Result<RpcFile> {
    js.into_serde().map_err(Into::into)
}

#[derive(Deserialize)]
pub struct RpcFile {
    pub path: PathBuf,
    pub file: Base64File,
}

#[derive(Deserialize)]
pub struct Base64File {
    unencoded_size: usize,
    base64: String,
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
