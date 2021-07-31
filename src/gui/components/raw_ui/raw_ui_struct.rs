use yew::prelude::*;

use crate::gui::components::Table;

pub enum Msg {
    Toggle,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub label: String,
    pub children: Children,
    #[prop_or(false)]
    pub opened: bool,
}

pub struct RawUiStruct {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for RawUiStruct {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        RawUiStruct { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                self.props.opened = !self.props.opened;
                true
            }
        }
    }

    fn change(&mut self, mut props: Self::Properties) -> ShouldRender {
        let Props { label, opened, children } = &mut props;
        // Prevent struct to close
        if self.props.label != *label || self.props.children != *children {
            *opened = self.props.opened;
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let Props { ref label, ref children, opened } = self.props;
        let chevron = if opened { "table-chevron-down" } else { "table-chevron-right" };

        let content = opened
            .then(|| {
                html! {
                    <div class="p-1">
                        <Table>
                            { children.clone() }
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
                        { label }
                    </button>
                </div>
                { content }
            </div>
        }
    }
}
