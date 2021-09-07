use std::cell::{Ref, RefMut};

use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::gui::RcUi;
use crate::save_data::Guid;

pub enum Msg {
    Change(Event),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: String,
    pub guid: RcUi<Guid>,
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
                let input: HtmlInputElement = event.target_unchecked_into();
                if let Ok(guid) = Uuid::parse_str(&input.value()) {
                    *ctx.props().guid_mut() = Guid::from(guid);
                }
                true
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
