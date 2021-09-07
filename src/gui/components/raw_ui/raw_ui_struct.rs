use yew::prelude::*;

use crate::gui::components::Table;

pub enum Msg {
    Toggle,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: String,
    pub children: Children,
    #[prop_or(false)]
    pub opened: bool,
}

pub struct RawUiStruct {
    opened: bool,
}

impl Component for RawUiStruct {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        RawUiStruct { opened: ctx.props().opened }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle => {
                self.opened = !self.opened;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { ref label, ref children, .. } = ctx.props();
        let chevron = if self.opened { "table-chevron-down" } else { "table-chevron-right" };

        let content = self.opened.then(|| {
            html! {
                <div class="p-1">
                    <Table>
                        { children.clone() }
                    </Table>
                </div>
            }
        });

        html! {
            <div class="flex-auto flex flex-col">
                <div class="p-px">
                    <button
                        class={classes![
                            "rounded-none",
                            "hover:bg-theme-hover",
                            "active:bg-theme-active",
                            "px-1",
                            "w-full",
                            "text-left",
                            "pl-6",
                            chevron,
                        ]}
                        onclick={ctx.link().callback(|_| Msg::Toggle)}
                    >
                        { label }
                    </button>
                </div>
                { for content }
            </div>
        }
    }
}
