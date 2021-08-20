mod command;
mod dialog;

use std::array::IntoIter;
use std::env;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::ArgMatches;
use serde_json::{json, Value};
use wry::application::window::Window;
use wry::webview::{RpcRequest, RpcResponse};

macro_rules! emit_commands {
    ($req:ident, $window:ident => [$(command::$command:ident),* $(,)?]) => {
        $(
            if $req.method == stringify!($command) {
                command::$command($window);
                return Ok(None);
            }
        )*
    };
}

macro_rules! send_commands {
    ($req:ident, $window:ident => [$(command::$command:ident),* $(,)?]) => {
        $(
            if $req.method == stringify!($command) {
                let response = command::$command($window)?;
                let js_value = serde_json::to_value(&response).map(Some)?;
                return Ok(js_value);
            }
        )*
    };
}

macro_rules! send_commands_with_param {
    ($req:ident, $window:ident => [$(command::$command:ident),* $(,)?]) => {
        $(
            if $req.method == stringify!($command) {
                let params = $req.params.take().context("argument required")?;
                let value: [_; 1] = serde_json::from_value(params)?;
                let value = IntoIter::new(value).next().unwrap_or_default();
                let response = command::$command($window, value)?;
                let js_value = serde_json::to_value(&response).map(Some)?;
                return Ok(js_value);
            }
        )*
    };
}

pub fn rpc_handler(window: &Window, mut req: RpcRequest, args: &ArgMatches) -> Option<RpcResponse> {
    let mut handle_request = || -> Result<Option<Value>> {
        if req.method == "open_command_line_save" {
            let response = if let Some(path) = args.value_of("SAVE") {
                let mut path = PathBuf::from(path);
                if path.is_relative() {
                    path = env::current_dir()?.join(path);
                }
                command::reload_save(window, path).map(Some)?
            } else {
                None
            };
            let js_value = serde_json::to_value(&response).map(Some)?;
            return Ok(js_value);
        }

        emit_commands!(req, window => [command::init]);

        send_commands!(req, window => [
            command::open_save,
            command::import_head_morph,
            command::export_head_morph_dialog,
        ]);

        send_commands_with_param!(req, window => [
            command::save_file,
            command::save_save_dialog,
            command::reload_save,
            command::load_database,
        ]);

        bail!("Wrong RPC method, got: {}", req.method)
    };

    match handle_request() {
        Ok(None) => None,
        Ok(Some(response)) => Some(RpcResponse::new_result(req.id.take(), Some(response))),
        Err(error) => Some(RpcResponse::new_error(req.id.take(), Some(json!(error.to_string())))),
    }
}
