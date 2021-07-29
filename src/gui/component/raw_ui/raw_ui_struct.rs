use yew::prelude::*;
use yewtil::NeqAssign;

use crate::gui::component::Table;

pub enum Msg {
    Toggle,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: String,
    pub children: Children,
}

pub struct RawUiStruct {
    props: Props,
    link: ComponentLink<Self>,
    opened: bool,
}

impl Component for RawUiStruct {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        RawUiStruct { props, link, opened: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                self.opened = !self.opened;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let chevron = if self.opened { "table-chevron-down" } else { "table-chevron-right" };

        let content = self
            .opened
            .then(|| {
                html! {
                    <div class="p-1">
                        <Table>
                            { self.props.children.clone() }
                        </Table>
                    </div>
                }
            })
            .unwrap_or_default();

        html! {
            <div class="flex-auto flex flex-col">
                <div class="p-px">
                    <button
                        class=classes![
                            "rounded-none",
                            "hover:bg-theme-hover",
                            "active:bg-theme-active",
                            "px-1",
                            "w-full",
                            "text-left",
                            "pl-6",
                            chevron,
                        ]
                        onclick=self.link.callback(|_| Msg::Toggle)
                    >
                        { &self.props.label }
                    </button>
                </div>
                { content }
            </div>
        }
    }
}
