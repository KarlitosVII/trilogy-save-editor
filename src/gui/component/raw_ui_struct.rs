use yew::prelude::*;

use crate::gui::component::Table;

pub enum Msg {
    Toggle,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: String,
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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        let chevron = if self.opened {
            // Down ▼
            "table-chevron-down"
        } else {
            // Right ▶
            "table-chevron-right"
        };

        html! {
            <div class="flex flex-col">
                <div class="flex-1 p-px">
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
                        { &self.props.title }
                    </button>
                </div>
                <Table>
                    { self.props.children.clone() }
                </Table>
            </div>
        }
    }
}
