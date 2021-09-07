use anyhow::{anyhow, Error};
use gloo::{
    events::EventListener,
    storage::{LocalStorage, Storage},
};
use js_sys::Date;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures as futures;
use web_sys::CustomEvent;
use yew::prelude::*;

use crate::services::rpc;

enum UpdateState {
    None,
    UpdateAvailable,
    DownloadProgress(f64),
}

pub enum Msg {
    UpdateAvailable,
    InstallUpdate,
    DownloadProgress(f64),
    Error(Error),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onerror: Callback<Error>,
}

pub struct AutoUpdate {
    _update_listener: EventListener,
    _progress_listener: EventListener,
    _error_listener: EventListener,
    update_state: UpdateState,
}

impl Component for AutoUpdate {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let update_listener = {
            let link = ctx.link().clone();
            EventListener::new(&yew::utils::document(), "tse_update_available", move |_| {
                link.send_message(Msg::UpdateAvailable);
            })
        };

        let progress_listener = {
            let link = ctx.link().clone();
            EventListener::new(&yew::utils::document(), "tse_update_progress", move |event| {
                if let Some(event) = event.dyn_ref::<CustomEvent>() {
                    #[derive(Deserialize)]
                    struct Progress {
                        progress: f64,
                    }

                    let Progress { progress } = serde_wasm_bindgen::from_value(event.detail())
                        .expect("Failed to parse Progress");
                    link.send_message(Msg::DownloadProgress(progress));
                }
            })
        };

        let error_listener = {
            let link = ctx.link().clone();
            EventListener::new(&yew::utils::document(), "tse_update_error", move |event| {
                if let Some(event) = event.dyn_ref::<CustomEvent>() {
                    #[derive(Deserialize)]
                    struct Error {
                        error: String,
                    }

                    let Error { error } = serde_wasm_bindgen::from_value(event.detail())
                        .expect("Failed to parse Error");
                    let error = anyhow!(error).context("Auto update error");
                    link.send_message(Msg::Error(error));
                }
            })
        };

        AutoUpdate::check_for_update();

        AutoUpdate {
            _update_listener: update_listener,
            _progress_listener: progress_listener,
            _error_listener: error_listener,
            update_state: UpdateState::None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateAvailable => {
                self.update_state = UpdateState::UpdateAvailable;
                true
            }
            Msg::InstallUpdate => {
                futures::spawn_local(async {
                    LocalStorage::delete("last_update_check");
                    let _ = rpc::download_and_install_update().await;
                });
                false
            }
            Msg::DownloadProgress(progress) => {
                self.update_state = UpdateState::DownloadProgress(progress);
                true
            }
            Msg::Error(err) => {
                self.update_state = UpdateState::None;
                ctx.props().onerror.emit(err);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.update_state {
            UpdateState::UpdateAvailable => html! {
                <div class="flex-auto flex items-center gap-2 px-1">
                    <div class="flex-auto text-right">{"A new update is available"}</div>
                    <button class="button"
                        onclick={ctx.link().callback(|_| Msg::InstallUpdate)}
                    >
                        {"Download and install"}
                    </button>
                </div>
            },
            UpdateState::DownloadProgress(progress) => html! {
                <div class="flex-auto px-1 text-right">{ format!("Downloading update: {}%", (progress * 100.0) as usize) }</div>
            },
            UpdateState::None => Default::default(),
        }
    }
}

impl AutoUpdate {
    fn check_for_update() {
        let should_check = LocalStorage::get("last_update_check")
            .map(|date: f64| (Date::now() - date) > 86_400_000.0) // 24h
            .unwrap_or(true);

        if should_check {
            futures::spawn_local(async {
                let _ = rpc::check_for_update().await;
                let _ = LocalStorage::set("last_update_check", Date::now());
            });
        }
    }
}
