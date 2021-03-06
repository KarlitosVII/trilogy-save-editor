use std::cell::{Ref, RefMut};

use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::save_data::Guid;
use crate::save_data::RcRef;

pub enum Msg {
    Change(Event),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub guid: RcRef<Guid>,
}

impl Props {
    fn guid(&self) -> Ref<'_, Guid> {
        self.guid.borrow()
    }

    fn guid_mut(&self) -> RefMut<'_, Guid> {
        self.guid.borrow_mut()
    }
}

pub struct RawUiGuid;

impl Component for RawUiGuid {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        RawUiGuid
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Change(event) => {
                if let Some(input) = event.target_dyn_into::<HtmlInputElement>() {
                    if let Ok(guid) = Uuid::parse_str(&input.value()) {
                        *ctx.props().guid_mut() = Guid::from(guid);
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let value = ctx.props().guid().hyphenated();
        let onchange = ctx.link().callback(Msg::Change);
        html! {
            <label class="flex-auto flex items-center gap-1">
                <input type="text" class="input w-1/3" placeholder="<empty>" {value} {onchange} />
                { &ctx.props().label }
            </label>
        }
    }
}
