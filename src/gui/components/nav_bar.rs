use wasm_bindgen_futures as futures;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::{
    gui::components::{Tab, TabBar},
    services::rpc,
};

const NEXUSMODS_LINK: &str = "https://www.nexusmods.com/masseffectlegendaryedition/mods/20";
const GITHUB_LINK: &str = "https://github.com/KarlitosVII/trilogy-save-editor";
const DONATION_LINK: &str = "https://www.paypal.com/donate/?business=karlitos.vii@laposte.net";

pub enum Msg {
    MenuOpen,
    MenuClose,
    MenuBlur,
    LicensesHover,
    OpenLink(&'static str),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    pub save_loaded: bool,
    pub onopen: Callback<()>,
    pub onsave: Callback<()>,
    pub onreload: Callback<()>,
}

pub struct NavBar {
    about_ref: NodeRef,
    about_opened: bool,
    licenses_opened: bool,
}

impl Component for NavBar {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        NavBar { about_ref: Default::default(), about_opened: false, licenses_opened: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MenuOpen => {
                self.about_opened = true;
                true
            }
            Msg::MenuClose => {
                self.about_opened = false;
                self.licenses_opened = false;
                true
            }
            Msg::MenuBlur => {
                if let Some(about) = self.about_ref.cast::<HtmlElement>() {
                    let _ = about.blur();
                }
                false
            }
            Msg::LicensesHover => {
                self.licenses_opened = true;
                true
            }
            Msg::OpenLink(link) => {
                futures::spawn_local(async move {
                    let _ = rpc::open_external_link(link).await;
                });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let loaded_buttons = ctx.props().save_loaded.then(|| {
            html! { <>
                <button class="button" onclick={ctx.props().onsave.reform(|_| ())}>
                    {"Save"}
                </button>
                <span>{"-"}</span>
                <button class="button" onclick={ctx.props().onreload.reform(|_| ())}>
                    {"Reload"}
                </button>
            </> }
        });

        html! {
            <nav class="bg-menu-bar select-none flex">
                <div class="flex items-center gap-2 px-1">
                    <button class="button" onclick={ctx.props().onopen.reform(|_| ())}>
                        {"Open"}
                    </button>
                    { for loaded_buttons }
                    { self.view_about_menu(ctx) }
                </div>
                { ctx.props().children.clone() }
            </nav>
        }
    }
}

impl NavBar {
    fn view_about_menu(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let onclick = if !self.about_opened {
            link.callback(|_| Msg::MenuOpen)
        } else {
            link.callback(|_| Msg::MenuBlur)
        };

        let licenses = self.licenses_opened.then(|| self.view_licenses());

        html! {
            <div class="relative" tabindex="0"
                onblur={self.about_opened.then(|| link.callback(|_| Msg::MenuClose))}
                ref={self.about_ref.clone()}
            >
                <a
                    class={classes![
                        "hover:bg-theme-hover",
                        "px-2",
                        "py-px",
                        "cursor-pointer",
                        self.about_opened.then(|| "bg-theme-hover" )
                    ]}
                    {onclick}
                >
                    {"About"}
                </a>
                <div class={classes![
                    "absolute",
                    "left-0",
                    "flex",
                    "flex-col",
                    "gap-px",
                    "bg-popup/90",
                    "border",
                    "border-default-border",
                    "p-1",
                    "z-40",
                    (!self.about_opened).then(|| "hidden" )
                ]}>
                        <hr class="border-default-border" />
                        <span class="px-1 whitespace-nowrap">
                            {"© 2021 Karlitos"}
                        </span>
                        <hr class="border-default-border" />
                        <a class={classes![
                                "px-1",
                                "hover:bg-theme-hover",
                                "active:bg-theme-active",
                                "whitespace-nowrap",
                                "cursor-pointer",
                                "link",
                            ]}
                            title={NEXUSMODS_LINK}
                            onclick={link.callback(|_| Msg::OpenLink(NEXUSMODS_LINK))}
                        >
                            {"NexusMods"}
                        </a>
                        <hr class="border-default-border" />
                        <a class={classes![
                                "px-1",
                                "hover:bg-theme-hover",
                                "active:bg-theme-active",
                                "whitespace-nowrap",
                                "cursor-pointer",
                                "link",
                            ]}
                            title={GITHUB_LINK}
                            onclick={link.callback(|_| Msg::OpenLink(GITHUB_LINK))}
                        >
                            {"Github"}
                        </a>
                        <hr class="border-default-border" />
                        <a class={classes![
                                "px-1",
                                "hover:bg-theme-hover",
                                "active:bg-theme-active",
                                "whitespace-nowrap",
                                "cursor-pointer",
                                "link",
                            ]}
                            title={DONATION_LINK}
                            onclick={link.callback(|_| Msg::OpenLink(DONATION_LINK))}
                        >
                            {"Donate"}
                        </a>
                        <hr class="border-default-border" />
                        <div class="relative flex">
                            { for licenses }
                            <a class={classes![
                                    "flex-auto",
                                    "px-1",
                                    "hover:bg-theme-hover",
                                    "whitespace-nowrap",
                                    "cursor-pointer",
                                    "navbar-chevron",
                                ]}
                                onmouseover={link.callback(|_| Msg::LicensesHover)}
                            >
                                {"License"}
                            </a>
                        </div>
                </div>
            </div>
        }
    }

    fn view_licenses(&self) -> Html {
        html! {
            <div class={classes![
                "absolute",
                "left-full",
                "flex",
                "bg-popup/90",
                "border",
                "border-default-border",
                "p-1",
                "w-[582px]",
                "h-[570px]",
                "z-40",
            ]}>
                <TabBar>
                    <Tab title="English">
                        <pre class="px-2 select-text">
                            { include_str!("../../../LICENSE.txt") }
                        </pre>
                    </Tab>
                    <Tab title="French">
                        <pre class="px-2 select-text">
                            { include_str!("../../../LICENSE_FRENCH.txt") }
                        </pre>
                    </Tab>
                </TabBar>
            </div>
        }
    }
}
