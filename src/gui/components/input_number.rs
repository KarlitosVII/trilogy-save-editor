use yew::prelude::*;
use yewtil::NeqAssign;

use crate::gui::{components::Helper, RcUi};

use super::CallbackType;

#[derive(Clone)]
pub enum NumberType {
    Byte(RcUi<u8>),
    Integer(RcUi<i32>),
    Float(RcUi<f32>),
}

impl PartialEq for NumberType {
    fn eq(&self, other: &NumberType) -> bool {
        match self {
            NumberType::Byte(byte) => match other {
                NumberType::Byte(other) => byte == other,
                _ => false,
            },
            NumberType::Integer(integer) => match other {
                NumberType::Integer(other) => integer == other,
                _ => false,
            },
            NumberType::Float(float) => match other {
                NumberType::Float(other) => float == other,
                _ => false,
            },
        }
    }
}

pub enum Msg {
    Change(ChangeData),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: String,
    pub value: NumberType,
    pub helper: Option<&'static str>,
    pub onchange: Option<Callback<CallbackType>>,
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
            Msg::Change(ChangeData::Value(value)) => {
                if value.is_empty() {
                    return true;
                }

                if let Ok(value) = value.parse::<f64>() {
                    match self.props.value {
                        NumberType::Byte(ref mut byte) => {
                            let value: u8 = value as u8;
                            *byte.borrow_mut() = value;

                            if let Some(ref callback) = self.props.onchange {
                                callback.emit(CallbackType::Byte(value));
                            }
                        }
                        NumberType::Integer(ref mut integer) => {
                            let value = value as i32;
                            *integer.borrow_mut() = value;

                            if let Some(ref callback) = self.props.onchange {
                                callback.emit(CallbackType::Integer(value));
                            }
                        }
                        NumberType::Float(ref mut float) => {
                            let value = value.clamp(f32::MIN as f64, f32::MAX as f64) as f32;
                            *float.borrow_mut() = value;

                            if let Some(ref callback) = self.props.onchange {
                                callback.emit(CallbackType::Float(value));
                            }
                        }
                    };
                }
                true
            }
            _ => unreachable!(),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let (value, placeholder) = match self.props.value {
            NumberType::Byte(ref byte) => (byte.borrow().to_string(), "<byte>"),
            NumberType::Integer(ref integer) => (integer.borrow().to_string(), "<integer>"),
            NumberType::Float(ref float) => {
                let mut ryu = ryu::Buffer::new();
                (ryu.format(*float.borrow()).trim_end_matches(".0").to_owned(), "<float>")
            }
        };

        let helper = self
            .props
            .helper
            .as_ref()
            .map(|&helper| {
                html! {
                    <Helper text=helper />
                }
            })
            .unwrap_or_default();

        html! {
            <label class="flex items-center gap-1">
                <input type="number" class="input w-[120px]" step="any"
                    placeholder=placeholder
                    value=value
                    onchange=self.link.callback(Msg::Change)
                />
                { &self.props.label }
                { helper }
            </label>
        }
    }
}
