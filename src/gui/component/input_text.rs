use std::cell::{Ref, RefMut};
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::gui::RcUi;

pub enum Msg {
    Input(String),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: String,
    pub value: RcUi<String>,
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
                *self.props.value_mut() = value;
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let oninput = self.link.callback(|data: InputData| Msg::Input(data.value));

        html! {
            <label>
                <input type="text" class="input w-2/3" value=self.props.value().to_owned() oninput=oninput />
                { &self.props.label }
            </label>
        }
    }
}
