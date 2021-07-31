use std::cell::{Ref, RefMut};
use uuid::Uuid;
use yew::prelude::*;

use crate::{gui::RcUi, save_data::Guid};

pub enum Msg {
    Change(ChangeData),
}

#[derive(Properties, Clone)]
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
            Msg::Change(ChangeData::Value(value)) => {
                if let Ok(guid) = Uuid::parse_str(&value) {
                    *self.props.guid_mut() = Guid::from(guid.to_hyphenated().to_string());
                }
                true
            }
            _ => unreachable!(),
        }
    }

    fn change(&mut self, mut props: Self::Properties) -> ShouldRender {
        let Props { label, guid } = &mut props;
        // Prevent struct to close
        if self.props.label != *label || !self.props.guid.ptr_eq(guid) {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <label class="flex items-center gap-1 align-bottom">
                <input type="text" class="input w-1/3" placeholder="<empty>" value=self.props.guid().to_string()
                    onchange=self.link.callback(Msg::Change)
                />
                { &self.props.label }
            </label>
        }
    }
}
