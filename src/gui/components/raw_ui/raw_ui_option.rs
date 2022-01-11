use std::{
    cell::{Ref, RefMut},
    marker::PhantomData,
};

use yew::prelude::*;

use crate::{gui::raw_ui::RawUi, save_data::RcRef};

pub enum Msg {
    Remove,
}

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: RawUi,
{
    pub label: String,
    pub option: RcRef<Option<T>>,
}

impl<T> Props<T>
where
    T: RawUi,
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
    T: RawUi,
{
    _marker: PhantomData<T>,
}

impl<T> Component for RawUiOption<T>
where
    T: RawUi,
{
    type Message = Msg;
    type Properties = Props<T>;

    fn create(_ctx: &Context<Self>) -> Self {
        RawUiOption { _marker: PhantomData }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Remove => {
                *ctx.props().option_mut() = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match *ctx.props().option() {
            Some(ref content) => html! {
                <div class="flex gap-1">
                    <div class="py-px">
                        <a
                            class={classes![
                                "rounded-none",
                                "select-none",
                                "hover:bg-theme-hover",
                                "active:bg-theme-active",
                                "bg-theme-bg",
                                "px-1",
                                "py-0",
                                "cursor-pointer",
                            ]}
                            onclick={ctx.link().callback(|_| Msg::Remove)}
                        >
                            {"remove"}
                        </a>
                    </div>
                    { content.view(&ctx.props().label) }
                </div>
            },
            None => html! {
                <div class="flex-auto flex items-center gap-1">
                    <span class="w-2/3">{ "None" }</span>
                    { &ctx.props().label }
                </div>
            },
        }
    }
}
