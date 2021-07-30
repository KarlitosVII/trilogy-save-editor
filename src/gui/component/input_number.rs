use yew::prelude::*;

use crate::gui::RcUi;

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
            NumberType::Byte(byte) => {
                if let NumberType::Byte(other) = other {
                    byte.ptr_eq(other)
                } else {
                    false
                }
            }
            NumberType::Integer(integer) => {
                if let NumberType::Integer(other) = other {
                    integer.ptr_eq(other)
                } else {
                    false
                }
            }
            NumberType::Float(float) => {
                if let NumberType::Float(other) = other {
                    float.ptr_eq(other)
                } else {
                    false
                }
            }
        }
    }
}

pub enum Msg {
    Change(ChangeData),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub label: String,
    pub value: NumberType,
    #[prop_or_default]
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
            Msg::Change(data) => match data {
                ChangeData::Value(value) => {
                    if value.is_empty() {
                        return true;
                    }

                    if let Ok(value) = value.parse::<f64>() {
                        match self.props.value {
                            NumberType::Byte(ref byte) => {
                                let value: u8 = value as u8;
                                *byte.borrow_mut() = value;

                                if let Some(ref callback) = self.props.onchange {
                                    callback.emit(CallbackType::Byte(value));
                                }
                            }
                            NumberType::Integer(ref integer) => {
                                let value = value as i32;
                                *integer.borrow_mut() = value;

                                if let Some(ref callback) = self.props.onchange {
                                    callback.emit(CallbackType::Integer(value));
                                }
                            }
                            NumberType::Float(ref float) => {
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
                _ => false,
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let Props { label, value, onchange } = &props;
        if self.props.label != *label
            || self.props.value != *value
            || self.props.onchange != *onchange
        {
            self.props = props;
            true
        } else {
            false
        }
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

        html! {
            <label class="flex items-center gap-1">
                <input type="number" class="input w-[120px]" placeholder=placeholder value=value onchange=self.link.callback(Msg::Change) />
                { &self.props.label }
            </label>
        }
    }
}
