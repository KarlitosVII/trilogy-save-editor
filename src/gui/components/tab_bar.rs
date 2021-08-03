use yew::prelude::*;
use yewtil::NeqAssign;

use crate::gui::Theme;

pub enum Msg {
    TabClicked(usize),
}

#[derive(Properties, Clone, PartialEq)]
pub struct TabBarProps {
    #[prop_or_default]
    current_tab: usize,
    pub children: ChildrenWithProps<Tab>,
}

pub struct TabBar {
    props: TabBarProps,
    link: ComponentLink<Self>,
}

impl Component for TabBar {
    type Message = Msg;
    type Properties = TabBarProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TabBar { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TabClicked(idx) => {
                self.props.current_tab = idx;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let tabs = self.props.children.iter().enumerate().map(|(idx, child)| {
            html_nested! {
                <button
                    class=classes![
                        "btn",
                        "leading-[1.2em]",
                        "!rounded-t",
                        (idx == self.props.current_tab).then(|| "!bg-theme-active"),
                        child.props.theme,
                    ]
                    onmousedown=(idx != self.props.current_tab).then(|| self.link.callback(move |_| Msg::TabClicked(idx)))
                >
                    { child.props.title }
                </button>
            }
        });

        let content = self
            .props
            .children
            .iter()
            .enumerate()
            .find_map(|(idx, content)| {
                (idx == self.props.current_tab).then(|| {
                    html! {
                        <div class=classes![
                            "flex-auto",
                            "flex",
                            "flex-col",
                            "h-0",
                            "overflow-y-auto",
                            content.props.theme.clone()
                        ]>
                            { content }
                        </div>
                    }
                })
            })
            .unwrap_or_default();

        html! {
            <div class="flex flex-col flex-nowrap flex-auto">
                <div class="flex flex-wrap gap-1 border-b border-theme-active mb-1">
                    { for tabs }
                </div>
                { content }
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
