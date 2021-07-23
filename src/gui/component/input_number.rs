use yew::prelude::*;
use yewtil::NeqAssign;

use crate::gui::RcUi;

#[derive(Clone, PartialEq)]
pub enum NumberType {
    Byte(RcUi<u8>),
    Integer(RcUi<i32>),
    Float(RcUi<f32>),
}

pub enum Msg {
    Change(ChangeData),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: String,
    pub value: NumberType,
}

pub struct InputNumber {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for InputNumber {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        InputNumber { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(data) => match data {
                ChangeData::Value(mut value) => {
                    if value.is_empty() {
                        value = String::from("0");
                    }

                    if let Ok(value) = value.parse::<f64>() {
                        match self.props.value {
                            NumberType::Byte(ref byte) => {
                                *byte.borrow_mut() = value as u8;
                            }
                            NumberType::Integer(ref integer) => {
                                *integer.borrow_mut() = value as i32
                            }
                            NumberType::Float(ref float) => *float.borrow_mut() = value as f32,
                        };
                    }
                    true
                }
                _ => false,
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let (value, placeholder) = match self.props.value {
            NumberType::Byte(ref byte) => (byte.borrow().to_string(), "Byte"),
            NumberType::Integer(ref integer) => (integer.borrow().to_string(), "Integer"),
            NumberType::Float(ref float) => (float.borrow().to_string(), "Float"),
        };

        html! {
            <label>
                <input type="number" class="input w-[120px]" value=value placeholder=placeholder onchange=self.link.callback(Msg::Change) />
                { &self.props.label }
            </label>
        }
    }
}
