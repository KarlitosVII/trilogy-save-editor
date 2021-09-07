use std::cell::{Ref, RefMut};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::CallbackType;
use crate::gui::{components::Helper, RcUi};

pub enum Msg {
    Input(InputEvent),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: String,
    pub value: RcUi<String>,
    pub helper: Option<&'static str>,
    pub oninput: Option<Callback<CallbackType>>,
}

impl Props {
    fn value(&self) -> Ref<'_, String> {
        self.value.borrow()
    }

    fn value_mut(&self) -> RefMut<'_, String> {
        self.value.borrow_mut()
    }
}

pub struct InputText;

impl Component for InputText {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        InputText
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Input(event) => {
                let input: HtmlInputElement = event.target_unchecked_into();

                if let Some(ref callback) = ctx.props().oninput {
                    callback.emit(CallbackType::String(input.value()));
                }

                *ctx.props().value_mut() = input.value();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let helper = ctx.props().helper.as_ref().map(|&helper| {
            html! {
                <Helper text={helper} />
            }
        });
        let value = ctx.props().value().clone();
        let oninput = ctx.link().callback(Msg::Input);
        html! {
            <label class="flex-auto flex items-center gap-1">
                <input type="text" class="input w-2/3" placeholder="<empty>" {value} {oninput} />
                { &ctx.props().label }
                { for helper }
            </label>
        }
    }
}
