use std::cell::{Ref, RefMut};
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::gui::RcUi;

pub enum Msg {
    Toggle,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: String,
    pub value: RcUi<bool>,
}

impl Props {
    fn value(&self) -> Ref<'_, bool> {
        self.value.borrow()
    }

    fn value_mut(&self) -> RefMut<'_, bool> {
        self.value.borrow_mut()
    }
}

pub struct CheckBox {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for CheckBox {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CheckBox { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                let value = *self.props.value();
                *self.props.value_mut() = !value;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <label>
                <input type="checkbox" class="checkbox" checked=*self.props.value() onchange=self.link.callback(|_| Msg::Toggle) />
                { &self.props.label }
            </label>
        }
    }
}
