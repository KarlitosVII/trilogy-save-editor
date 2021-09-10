use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::PopStateEvent;
use yew::{html::Scope, prelude::*};

use crate::gui::Theme;

const MAIN_BUTTON: i16 = 0;

pub enum Msg {
    TabClicked(MouseEvent, String),
    MainTabChanged(String),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: ChildrenWithProps<Tab>,
    #[prop_or(false)]
    pub is_main_tab_bar: bool,
}

pub struct TabBar {
    main_tab_listener: Option<EventListener>,
    current_tab: String,
}

impl Component for TabBar {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let current_tab = Self::first_tab(&ctx.props().children);
        let main_tab_listener = ctx.props().is_main_tab_bar.then(|| {
            let link = ctx.link().clone();
            Self::event_listener(link)
        });

        // TODO: Tab history

        TabBar { current_tab, main_tab_listener }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TabClicked(event, title) => {
                if event.button() == MAIN_BUTTON {
                    self.current_tab = title;
                    true
                } else {
                    false
                }
            }
            Msg::MainTabChanged(main_tab) => {
                let children = &ctx.props().children;
                if children.iter().any(|child| child.props.title == main_tab) {
                    self.current_tab = main_tab;
                } else {
                    self.current_tab = Self::first_tab(children);
                }
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let is_main_tab_bar = ctx.props().is_main_tab_bar;
        let is_event_listening = self.main_tab_listener.is_some();

        if !is_main_tab_bar && is_event_listening {
            self.main_tab_listener = None;
        } else if is_main_tab_bar && !is_event_listening {
            let link = ctx.link().clone();
            self.main_tab_listener = Some(Self::event_listener(link));
        }

        // Go to first tab if current tab doesn't exist
        let children = &ctx.props().children;
        if !children.iter().any(|child| child.props.title == self.current_tab) {
            self.current_tab = Self::first_tab(children);
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let tabs = ctx.props().children.iter().map(|child| {
            let title = child.props.title.clone();
            let onmousedown = (title != self.current_tab).then(|| {
                let title = title.clone();
                ctx.link().callback(move |event| Msg::TabClicked(event, title.clone()))
            });
            html! {
                <a class={classes![
                        "rounded-b-none",
                        "rounded-t-[3px]",
                        "cursor-pointer",
                        "select-none",
                        "leading-[19px]",
                        "px-1",
                        "bg-theme-tab",
                        "hover:!bg-theme-hover",
                        (title == self.current_tab).then(|| "!bg-theme-active"),
                        child.props.theme,
                    ]}
                    {onmousedown}
                >
                    { title }
                </a>
            }
        });

        let content = ctx.props().children.iter().find_map(|content| {
            (content.props.title == self.current_tab).then(|| {
                html! {
                    <div class={classes![
                        "flex-auto",
                        "flex",
                        "flex-col",
                        "h-0",
                        "overflow-y-auto",
                        content.props.theme,
                    ]}>
                        { content }
                    </div>
                }
            })
        });

        html! {
            <div class="flex-auto flex flex-col min-w-0">
                <div class="flex flex-wrap gap-1 border-b border-theme-active mb-1">
                    { for tabs }
                </div>
                { for content }
            </div>
        }
    }
}

impl TabBar {
    fn event_listener(link: Scope<Self>) -> EventListener {
        EventListener::new(&yew::utils::window(), "popstate", {
            move |event| {
                if let Some(event) = event.dyn_ref::<PopStateEvent>() {
                    let main_tab: String =
                        serde_wasm_bindgen::from_value(event.state()).unwrap_or_default();
                    link.send_message(Msg::MainTabChanged(main_tab));
                }
            }
        })
    }

    fn first_tab(children: &ChildrenWithProps<Tab>) -> String {
        children.iter().next().map(|child| child.props.title.clone()).unwrap_or_default()
    }
}

#[derive(Properties, PartialEq)]
pub struct TabProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
    pub theme: Option<Theme>,
}

pub struct Tab;

impl Component for Tab {
    type Message = ();
    type Properties = TabProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Tab
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        ctx.props().children.iter().collect::<Html>()
    }
}
