use std::cell::{Ref, RefMut};

use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::{prelude::*, utils::NeqAssign};

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

    fn guid_mut(&mut self) -> RefMut<'_, Guid> {
        self.guid.borrow_mut()
    }
}

pub struct RawUiGuid {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for RawUiGuid {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        RawUiGuid { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(event) => {
                let input: HtmlInputElement = event.target_unchecked_into();
                if let Ok(guid) = Uuid::parse_str(&input.value()) {
                    *self.props.guid_mut() = Guid::from(guid);
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let value = self.props.guid().hyphenated();
        let onchange = self.link.callback(Msg::Change);
        html! {
            <label class="flex-auto flex items-center gap-1">
                <input type="text" class="input w-1/3" placeholder="<empty>" {value} {onchange} />
                { &self.props.label }
            </label>
        }
    }
}
