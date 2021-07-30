use std::{
    cell::{Ref, RefMut},
    fmt::Display,
};
use yew::prelude::*;

use crate::gui::{component::Table, RawUi, RcUi};

pub enum Msg {
    Toggle,
    Add,
    Remove(usize),
}

#[derive(Properties, Clone)]
pub struct Props<T>
where
    T: RawUi + Default + Display,
{
    pub label: String,
    pub vec: RcUi<Vec<T>>,
}

impl<T> Props<T>
where
    T: RawUi + Default + Display,
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
    T: RawUi + Default + Display,
{
    props: Props<T>,
    link: ComponentLink<Self>,
    opened: bool,
}

impl<T> Component for RawUiVec<T>
where
    T: RawUi + Default + Display,
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
        let Props { label, vec } = &props;
        if self.props.label != *label || !self.props.vec.ptr_eq(vec) {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let chevron = if self.opened { "table-chevron-down" } else { "table-chevron-right" };

        let content = self
            .opened
            .then(|| {
                let vec = self.props.vec();
                
                // Exceptions
                macro_rules! display_idx {
                    ($vec:ident => $($type:ty)*) => {
                        $((&*$vec as &dyn std::any::Any).is::<Vec<RcUi<$type>>>()) ||*
                    }
                }
                let display_idx = display_idx!(vec => u8 i32 f32 bool String);

                let items = vec.iter().enumerate().map(|(i, item)| {
                    let label = item.to_string();
                    let row = if display_idx || label.is_empty() {
                        item.view(&i.to_string())
                    } else {
                        item.view(&label)
                    };

                    html_nested! {
                        <div class="flex gap-1">
                            <div class="py-px">
                                <a
                                    class="rounded-none select-none hover:bg-theme-hover active:bg-theme-active bg-theme-bg px-1 py-0 cursor-pointer"
                                    onclick=self.link.callback(move |_| Msg::Remove(i))
                                >
                                    {"remove"}
                                </a>
                            </div>
                            { row }
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
