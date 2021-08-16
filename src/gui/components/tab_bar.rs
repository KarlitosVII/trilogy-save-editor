use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::PopStateEvent;
use yew::{prelude::*, utils::NeqAssign};

use crate::gui::Theme;

const MAIN_BUTTON: i16 = 0;

pub enum Msg {
    TabClicked(MouseEvent, String),
    MainTabChanged(String),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: ChildrenWithProps<Tab>,
    #[prop_or(false)]
    pub is_main_tab_bar: bool,
}

pub struct TabBar {
    props: Props,
    link: ComponentLink<Self>,
    main_tab_listener: Option<EventListener>,
    current_tab: String,
}

impl Component for TabBar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let current_tab = Self::first_tab(&props.children);
        let main_tab_listener = props.is_main_tab_bar.then(|| {
            let link = link.clone();
            Self::event_listener(link)
        });

        TabBar { props, link, current_tab, main_tab_listener }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
                let children = &self.props.children;
                if children.iter().any(|child| child.props.title == main_tab) {
                    self.current_tab = main_tab;
                } else {
                    self.current_tab = Self::first_tab(children);
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.neq_assign(props) {
            let is_main_tab_bar = self.props.is_main_tab_bar;
            let is_event_listening = self.main_tab_listener.is_some();

            if !is_main_tab_bar && is_event_listening {
                self.main_tab_listener = None;
            } else if is_main_tab_bar && !is_event_listening {
                let link = self.link.clone();
                self.main_tab_listener = Some(Self::event_listener(link));
            }

            // Go to first tab if current tab doesn't exist
            let children = &self.props.children;
            if !children.iter().any(|child| child.props.title == self.current_tab) {
                self.current_tab = Self::first_tab(&children);
            }
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let tabs = self.props.children.iter().map(|child| {
            let title = child.props.title.clone();
            let onmousedown = (title != self.current_tab).then(|| {
                let title = title.clone();
                self.link.callback(move |event| Msg::TabClicked(event, title.clone()))
            });
            html_nested! {
                <a class={classes![
                        "rounded-b-none",
                        "rounded-t",
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

        let content = self.props.children.iter().find_map(|content| {
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
            <div class="flex flex-col flex-nowrap flex-auto">
                <div class="flex flex-wrap gap-1 border-b border-theme-active mb-1">
                    { for tabs }
                </div>
                { for content }
            </div>
        }
    }
}

impl TabBar {
    fn event_listener(link: ComponentLink<Self>) -> EventListener {
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

#[derive(Properties, Clone, PartialEq)]
pub struct TabProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
    pub theme: Option<Theme>,
}

pub struct Tab {
    props: TabProps,
    // link: ComponentLink<Self>,
}

impl Component for Tab {
    type Message = ();
    type Properties = TabProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Tab { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        self.props.children.iter().collect::<Html>()
    }
}
