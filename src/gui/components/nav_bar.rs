use web_sys::HtmlElement;
use yew::{prelude::*, utils::NeqAssign};

use crate::gui::components::{Tab, TabBar};

pub enum Msg {
    MenuOpen,
    MenuClose,
    MenuBlur,
    LicensesHover,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub save_loaded: bool,
    pub onopen: Callback<()>,
    pub onsave: Callback<()>,
    pub onreload: Callback<()>,
}

pub struct NavBar {
    props: Props,
    link: ComponentLink<Self>,
    about_ref: NodeRef,
    about_opened: bool,
    licenses_opened: bool,
}

impl Component for NavBar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NavBar {
            props,
            link,
            about_ref: Default::default(),
            about_opened: false,
            licenses_opened: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let loaded_buttons = self.props.save_loaded.then(|| {
            html! { <>
                <button class="button" onclick={self.props.onsave.reform(|_| ())}>
                    {"Save"}
                </button>
                <span>{"-"}</span>
                <button class="button" onclick={self.props.onreload.reform(|_| ())}>
                    {"Reload"}
                </button>
            </> }
        });

        html! {
            <nav class="bg-menu-bar select-none">
                <div class="flex items-center gap-2 px-1">
                    <button class="button" onclick={self.props.onopen.reform(|_| ())}>
                        {"Open"}
                    </button>
                    { for loaded_buttons }
                    { self.view_about_menu() }
                </div>
            </nav>
        }
    }
}

impl NavBar {
    fn view_about_menu(&self) -> Html {
        let onclick = if !self.about_opened {
            self.link.callback(|_| Msg::MenuOpen)
        } else {
            self.link.callback(|_| Msg::MenuBlur)
        };

        html! {
            <div class="relative" tabindex="0"
                onblur={self.about_opened.then(||self.link.callback(|_| Msg::MenuClose))}
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
                        <hr class="border-default-border mb-px" />
                        <div class="relative flex">
                            { self.view_licenses() }
                            <a class={classes![
                                    "flex-auto",
                                    "px-1",
                                    "hover:bg-theme-hover",
                                    "whitespace-nowrap",
                                    "cursor-pointer",
                                    "navbar-chevron",
                                ]}
                                onmouseover={self.link.callback(|_| Msg::LicensesHover)}
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
                "w-[570px]",
                "h-[570px]",
                "z-40",
                (!self.licenses_opened).then(|| "hidden" )
            ]}>
                <TabBar>
                    <Tab title="English">
                        <pre class="px-2">
                            { include_str!("../../../LICENSE.txt") }
                        </pre>
                    </Tab>
                    <Tab title="French">
                        <pre class="px-2">
                            { include_str!("../../../LICENSE_FRENCH.txt") }
                        </pre>
                    </Tab>
                </TabBar>
            </div>
        }
    }
}
