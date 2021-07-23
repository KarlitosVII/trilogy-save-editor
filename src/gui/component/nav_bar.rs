use web_sys::HtmlElement;
use yew::{prelude::*, services::ConsoleService};

pub enum Msg {
    Open,
    Save,
    Reload,
    AboutMenuOpen,
    AboutMenuClose,
    AboutMenuBlur,
}

pub struct NavBar {
    link: ComponentLink<Self>,
    about_opened: bool,
    about_ref: NodeRef,
}

impl Component for NavBar {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NavBar { link, about_opened: false, about_ref: Default::default() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let log = ConsoleService::log;
        match msg {
            Msg::Open => {
                log("Open");
                false
            }
            Msg::Save => {
                log("Save");
                false
            }
            Msg::Reload => {
                log("Reload");
                false
            }
            Msg::AboutMenuOpen => {
                self.about_opened = true;
                true
            }
            Msg::AboutMenuClose => {
                self.about_opened = false;
                true
            }
            Msg::AboutMenuBlur => {
                if let Some(about) = self.about_ref.cast::<HtmlElement>() {
                    let _ = about.blur();
                }
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <nav class="bg-menu-bar select-none">
                <div class="flex gap-2 px-1">
                    <button class="btn" onclick=self.link.callback(|_| Msg::Open)>
                        {"Open"}
                    </button>
                    <button class="btn" onclick=self.link.callback(|_| Msg::Save)>
                        {"Save"}
                    </button>
                    <span>{"-"}</span>
                    <button class="btn" onclick=self.link.callback(|_| Msg::Reload)>
                        {"Reload"}
                    </button>
                    { self.view_about_menu() }
                </div>
            </nav>
        }
    }
}

impl NavBar {
    fn view_about_menu(&self) -> Html {
        let onclick = if !self.about_opened {
            self.link.callback(|_| Msg::AboutMenuOpen)
        } else {
            self.link.callback(|_| Msg::AboutMenuBlur)
        };

        html! {
            <div class="relative" tabindex="0"
                onblur=self.about_opened.then(||self.link.callback(|_| Msg::AboutMenuClose))
                ref=self.about_ref.clone()
            >
                <a
                    class=classes![
                        "hover:bg-theme-hover",
                        "px-2",
                        "py-px",
                        "cursor-pointer",
                        self.about_opened.then(|| "bg-theme-hover" )
                    ]
                    onclick=onclick
                >
                    {"About"}
                </a>
                <div class=classes![
                    "absolute",
                    "left-0",
                    "flex",
                    "flex-col",
                    "bg-popup/80",
                    "border",
                    "border-default-border",
                    "p-1",
                    "z-50",
                    (!self.about_opened).then(|| "hidden" )
                ]>
                    <hr class="border-default-border" />
                    <span class="px-1 whitespace-nowrap">
                        {"© 2021 Karlitos"}
                    </span>
                    <hr class="border-default-border mb-px" />
                    <a class="px-1 hover:bg-theme-hover whitespace-nowrap cursor-pointer relative navbar-chevron">
                        {"License"}
                    </a>
                </div>
            </div>
        }
    }
}
