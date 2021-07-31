use std::cell::{Ref, RefMut};
use yew::prelude::*;

use crate::gui::RcUi;

use super::CallbackType;

pub enum Msg {
    Input(String),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub label: String,
    pub value: RcUi<String>,
    #[prop_or_default]
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

pub struct InputText {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for InputText {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        InputText { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(value) => {
                if let Some(ref callback) = self.props.oninput {
                    callback.emit(CallbackType::String(value.clone()));
                }

                *self.props.value_mut() = value;
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let Props { label, value, oninput: onchange } = &props;
        if self.props.label != *label
            || !self.props.value.ptr_eq(value)
            || self.props.oninput != *onchange
        {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <label class="flex items-center gap-1 align-bottom">
                <input type="text" class="input w-2/3" placeholder="<empty>" value=self.props.value().to_owned()
                    oninput=self.link.callback(|data: InputData| Msg::Input(data.value))
                />
                { &self.props.label }
            </label>
        }
    }
}
