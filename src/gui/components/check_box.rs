use std::cell::{Ref, RefMut};
use yew::prelude::*;

use crate::gui::RcUi;

pub enum Msg {
    Toggle,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub value: RcUi<bool>,
    pub onchange: Option<Callback<bool>>,
}

impl Props {
    fn value(&self) -> Ref<'_, bool> {
        self.value.borrow()
    }

    fn value_mut(&self) -> RefMut<'_, bool> {
        self.value.borrow_mut()
    }
}

pub struct CheckBox;

impl Component for CheckBox {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        CheckBox
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle => {
                let value = !*ctx.props().value();
                *ctx.props().value_mut() = value;

                if let Some(ref callback) = ctx.props().onchange {
                    callback.emit(value);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let checked = *ctx.props().value();
        let onchange = ctx.link().callback(|_| Msg::Toggle);
        html! {
            <label class="flex items-center gap-1">
                <input type="checkbox" class="checkbox" {checked} {onchange} />
                { &ctx.props().label }
            </label>
        }
    }
}
