mod command;
mod dialog;

use std::env;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::ArgMatches;
use serde_json::{json, Value};
use wry::{
    application::{
        event_loop::{ControlFlow, EventLoopProxy},
        window::Window,
    },
    webview::{RpcRequest, RpcResponse, WebView},
};

macro_rules! notify_commands {
    ($req:ident, $utils:ident => [$(command::$command:ident),* $(,)?]) => {
        $(
            if $req.method == stringify!($command) {
                command::$command(&$utils);
                return Ok(None);
            }
        )*
    };
}

macro_rules! call_commands {
    ($req:ident, $utils:ident => [$(command::$command:ident),* $(,)?]) => {
        $(
            if $req.method == stringify!($command) {
                let response = command::$command(&$utils)?;
                let js_value = serde_json::to_value(&response).map(Some)?;
                return Ok(js_value);
            }
        )*
    };
}

macro_rules! call_commands_with_param {
    ($req:ident, $utils:ident => [$(command::$command:ident),* $(,)?]) => {
        $(
            if $req.method == stringify!($command) {
                let params = $req.params.take().context("argument required")?;
                let value: [_; 1] = serde_json::from_value(params)?;
                let value = value.into_iter().next().unwrap_or_default();
                let response = command::$command(&$utils, value)?;
                let js_value = serde_json::to_value(&response).map(Some)?;
                return Ok(js_value);
            }
        )*
    };
}

pub struct RpcUtils<'a> {
    pub window: &'a Window,
    pub event_proxy: &'a EventLoopProxy<Event>,
    pub args: &'a ArgMatches<'a>,
}

pub fn rpc_handler(mut req: RpcRequest, utils: RpcUtils) -> Option<RpcResponse> {
    let mut handle_request = || -> Result<Option<Value>> {
        if req.method == "open_command_line_save" {
            let response = if let Some(path) = utils.args.value_of("SAVE") {
                let mut path = PathBuf::from(path);
                if path.is_relative() {
                    path = env::current_dir()?.join(path);
                }
                command::reload_save(&utils, path).map(Some)?
            } else {
                None
            };
            let js_value = serde_json::to_value(&response).map(Some)?;
            return Ok(js_value);
        }

        notify_commands!(req, utils => [
            command::init,
            command::minimize,
            command::toggle_maximize,
            command::drag_window,
            command::close,
        ]);

        call_commands!(req, utils => [
            command::check_for_update,
            command::download_and_install_update,
            command::open_save,
            command::import_head_morph,
            command::export_head_morph_dialog,
        ]);

        call_commands_with_param!(req, utils => [
            command::open_external_link,
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

pub enum Event {
    CloseWindow,
    DispatchCustomEvent(&'static str, serde_json::Value),
}

pub fn event_handler(event: Event, webview: &WebView, control_flow: &mut ControlFlow) {
    match event {
        Event::CloseWindow => *control_flow = ControlFlow::Exit,
        Event::DispatchCustomEvent(event, detail) => {
            let _ = webview.evaluate_script(&format!(
                r#"
                (() => {{
                    const event = new CustomEvent("{event}", {{
                        detail: {detail}
                    }});
                    document.dispatchEvent(event);
                }})();
                "#,
                event = event,
                detail = detail,
            ));
        }
    }
}
