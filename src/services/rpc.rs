use std::path::PathBuf;

use anyhow::{anyhow, Result};
use serde::{de, Deserialize, Serialize};
use wasm_bindgen::JsValue;

mod js {
    use js_sys::JsString;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/services/rpc.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn call(method: &str) -> Result<JsValue, JsString>;

        #[wasm_bindgen(catch)]
        pub async fn call_with_params(method: &str, params: JsValue) -> Result<JsValue, JsString>;
    }
}

// Call
fn from_js_value<T>(js: JsValue) -> Result<T>
where
    T: for<'a> de::Deserialize<'a>,
{
    serde_wasm_bindgen::from_value(js).map_err(|e| anyhow!(e.to_string()))
}

async fn call<T>(method: &str) -> Result<T>
where
    T: for<'a> de::Deserialize<'a>,
{
    js::call(method).await.map(from_js_value).map_err(|e| anyhow!(String::from(e)))?
}

async fn call_with_params<P, T>(method: &str, params: P) -> Result<T>
where
    P: Serialize,
    T: for<'a> de::Deserialize<'a>,
{
    let js_params = serde_wasm_bindgen::to_value(&params).map_err(|e| anyhow!(e.to_string()))?;
    js::call_with_params(method, js_params)
        .await
        .map(from_js_value)
        .map_err(|e| anyhow!(String::from(e)))?
}

// Commands
pub async fn check_for_update() -> Result<()> {
    call("check_for_update").await
}

pub async fn download_and_install_update() -> Result<()> {
    call("download_and_install_update").await
}

pub async fn open_external_link(link: &str) -> Result<()> {
    call_with_params("open_external_link", link).await
}

pub async fn save_file(rpc_file: RpcFile) -> Result<()> {
    call_with_params("save_file", rpc_file).await
}

pub async fn open_save() -> Result<Option<RpcFile>> {
    call("open_save").await
}

pub async fn open_command_line_save() -> Result<Option<RpcFile>> {
    call("open_command_line_save").await
}

pub async fn save_save_dialog(params: DialogParams) -> Result<Option<PathBuf>> {
    call_with_params("save_save_dialog", params).await
}

pub async fn reload_save(path: PathBuf) -> Result<RpcFile> {
    call_with_params("reload_save", path).await
}

pub async fn import_head_morph() -> Result<Option<RpcFile>> {
    call("import_head_morph").await
}

pub async fn export_head_morph_dialog() -> Result<Option<PathBuf>> {
    call("export_head_morph_dialog").await
}

pub async fn load_database(path: &str) -> Result<RpcFile> {
    call_with_params("load_database", path).await
}

// Utils
#[derive(Serialize)]
pub struct DialogParams {
    pub path: PathBuf,
    pub filters: Vec<(&'static str, Vec<&'static str>)>,
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
        let mut vec = vec![0; self.unencoded_size];
        base64::decode_config_slice(self.base64, base64::STANDARD, &mut vec)?;
        Ok(vec)
    }
}
