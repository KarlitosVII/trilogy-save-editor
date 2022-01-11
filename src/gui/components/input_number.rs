use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{gui::components::Helper, save_data::RcCell};

use super::CallbackType;

#[derive(Clone)]
pub enum NumberType {
    Byte(RcCell<u8>),
    Int(RcCell<i32>),
    Float(RcCell<f32>),
}

impl PartialEq for NumberType {
    fn eq(&self, other: &NumberType) -> bool {
        match (self, other) {
            (NumberType::Byte(byte), NumberType::Byte(other)) => byte == other,
            (NumberType::Int(integer), NumberType::Int(other)) => integer == other,
            (NumberType::Float(float), NumberType::Float(other)) => float == other,
            _ => false,
        }
    }
}

pub enum Msg {
    Change(Event),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub value: NumberType,
    pub helper: Option<&'static str>,
    pub onchange: Option<Callback<CallbackType>>,
}

pub struct InputNumber;

impl Component for InputNumber {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        InputNumber
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Change(event) => {
                if let Some(input) = event.target_dyn_into::<HtmlInputElement>() {
                    let value = input.value_as_number();

                    if value.is_nan() {
                        return true;
                    }

                    match ctx.props().value {
                        NumberType::Byte(ref byte) => {
                            let value = value as u8;
                            byte.set(value);

                            if let Some(ref callback) = ctx.props().onchange {
                                callback.emit(CallbackType::Byte(value));
                            }
                        }
                        NumberType::Int(ref integer) => {
                            let value = value as i32;
                            integer.set(value);

                            if let Some(ref callback) = ctx.props().onchange {
                                callback.emit(CallbackType::Int(value));
                            }
                        }
                        NumberType::Float(ref float) => {
                            let value = value.clamp(f32::MIN as f64, f32::MAX as f64) as f32;
                            float.set(value);

                            if let Some(ref callback) = ctx.props().onchange {
                                callback.emit(CallbackType::Float(value));
                            }
                        }
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (value, placeholder) = match ctx.props().value {
            NumberType::Byte(ref byte) => (byte.get().to_string(), "<byte>"),
            NumberType::Int(ref integer) => (integer.get().to_string(), "<integer>"),
            NumberType::Float(ref float) => {
                let mut ryu = ryu::Buffer::new();
                (ryu.format(float.get()).trim_end_matches(".0").to_owned(), "<float>")
            }
        };

        let helper = ctx.props().helper.as_ref().map(|&helper| {
            html! {
                <Helper text={helper} />
            }
        });

        html! {
            <label class="flex items-center gap-1">
                <input type="number" class="input w-[110px]" step="any"
                    {placeholder}
                    {value}
                    onchange={ctx.link().callback(Msg::Change)}
                />
                { &ctx.props().label }
                { for helper }
            </label>
        }
    }
}
