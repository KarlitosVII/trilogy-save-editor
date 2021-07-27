use std::cell::{Ref, RefMut};
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::gui::{component::Table, RawUi, RcUi};

pub enum Msg {
    Toggle,
    Add,
    Remove(usize),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T>
where
    T: RawUi + PartialEq + Default,
{
    pub label: String,
    pub vec: RcUi<Vec<T>>,
}

impl<T> Props<T>
where
    T: RawUi + PartialEq + Default,
{
    fn vec(&self) -> Ref<'_, Vec<T>> {
        self.vec.borrow()
    }

    fn vec_mut(&self) -> RefMut<'_, Vec<T>> {
        self.vec.borrow_mut()
    }
}

pub struct RawUiVec<T>
where
    T: RawUi + PartialEq + Default,
{
    props: Props<T>,
    link: ComponentLink<Self>,
    opened: bool,
}

impl<T> Component for RawUiVec<T>
where
    T: RawUi + PartialEq + Default,
{
    type Message = Msg;
    type Properties = Props<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        RawUiVec { props, link, opened: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                self.opened = !self.opened;
                true
            }
            Msg::Add => {
                self.props.vec_mut().push(Default::default());
                true
            }
            Msg::Remove(idx) => {
                self.props.vec_mut().remove(idx);
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
                let vec = self.props.vec();
                let items = vec.iter().enumerate().map(|(i, item)| {
                    html_nested! {
                        <div class="flex flex-row gap-1">
                            <div>
                                <a
                                    class="rounded-none hover:bg-theme-hover active:bg-theme-active bg-theme-bg px-1 py-0 cursor-pointer select-none"
                                    onclick=self.link.callback(move |_| Msg::Remove(i))
                                >
                                    {"remove"}
                                </a>
                            </div>
                            { item.view(&i.to_string()) }
                        </div>
                    }
                });
                html! {
                    <div class="p-1">
                        <Table>
                            { for items }
                            <button
                                class="rounded-none hover:bg-theme-hover active:bg-theme-active bg-theme-bg px-1"
                                onclick=self.link.callback(|_| Msg::Add)
                            >
                                {"add"}
                            </button>
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
                            "pl-6",
                            "w-full",
                            "text-left",
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
