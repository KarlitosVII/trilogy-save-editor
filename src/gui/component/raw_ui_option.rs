use std::cell::{Ref, RefMut};
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::gui::{RawUi, RcUi};

pub enum Msg {
    Remove,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T>
where
    T: RawUi + PartialEq,
{
    pub label: String,
    pub option: RcUi<Option<T>>,
}

impl<T> Props<T>
where
    T: RawUi + PartialEq,
{
    fn option(&self) -> Ref<'_, Option<T>> {
        self.option.borrow()
    }

    fn option_mut(&self) -> RefMut<'_, Option<T>> {
        self.option.borrow_mut()
    }
}

pub struct RawUiOption<T>
where
    T: RawUi + PartialEq,
{
    props: Props<T>,
    link: ComponentLink<Self>,
}

impl<T> Component for RawUiOption<T>
where
    T: RawUi + PartialEq,
{
    type Message = Msg;
    type Properties = Props<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        RawUiOption { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Remove => {
                *self.props.option_mut() = None;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        match *self.props.option() {
            Some(ref content) => html! {
                <div class="flex flex-row gap-1">
                    <div>
                        <a
                            class="rounded-none hover:bg-theme-hover active:bg-theme-active bg-theme-bg px-1 py-0 cursor-pointer select-none"
                            onclick=self.link.callback(|_| Msg::Remove)
                        >
                            {"remove"}
                        </a>
                    </div>
                    { content.view(&self.props.label) }
                </div>
            },
            None => html! {
                <div class="flex flex-row gap-1">
                    <span class="w-2/3">{ "None" }</span>
                    <span>{ &self.props.label }</span>
                </div>
            },
        }
    }
}
