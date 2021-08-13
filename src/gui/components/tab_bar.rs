use yew::{prelude::*, utils::NeqAssign};

use crate::gui::Theme;

const MAIN_BUTTON: i16 = 0;

pub enum Msg {
    TabClicked(MouseEvent, usize),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: ChildrenWithProps<Tab>,
}

pub struct TabBar {
    props: Props,
    link: ComponentLink<Self>,
    current_tab: usize,
}

impl Component for TabBar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TabBar { props, link, current_tab: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TabClicked(event, idx) => {
                if event.button() == MAIN_BUTTON {
                    self.current_tab = idx;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let tabs = self.props.children.iter().enumerate().map(|(idx, child)| {
            let onmousedown = (idx != self.current_tab)
                .then(|| self.link.callback(move |event| Msg::TabClicked(event, idx)));
            html_nested! {
                <a class={classes![
                        "rounded-b-none",
                        "rounded-t",
                        "cursor-pointer",
                        "leading-[19px]",
                        "px-1",
                        "bg-theme-tab",
                        "hover:!bg-theme-hover",
                        (idx == self.current_tab).then(|| "!bg-theme-active"),
                        child.props.theme,
                    ]}
                    {onmousedown}
                >
                    { child.props.title }
                </a>
            }
        });

        let content = self.props.children.iter().enumerate().find_map(|(idx, content)| {
            (idx == self.current_tab).then(|| {
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
