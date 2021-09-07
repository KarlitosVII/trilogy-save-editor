use std::cell::{Ref, RefMut};

use anyhow::ensure;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::gui::{
    components::{CallbackType, InputNumber, NumberType},
    RcUi,
};
use crate::save_data::shared::appearance::LinearColor;

pub enum Msg {
    R(CallbackType),
    G(CallbackType),
    B(CallbackType),
    A(CallbackType),
    Change(Event),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: String,
    pub color: RcUi<LinearColor>,
}

impl Props {
    fn color(&self) -> Ref<'_, LinearColor> {
        self.color.borrow()
    }

    fn color_mut(&self) -> RefMut<'_, LinearColor> {
        self.color.borrow_mut()
    }
}

pub struct ColorPicker;

impl Component for ColorPicker {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        ColorPicker
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::R(CallbackType::Byte(value)) => {
                ctx.props().color_mut().r = value as f32 / 255.0;
                true
            }
            Msg::G(CallbackType::Byte(value)) => {
                ctx.props().color_mut().g = value as f32 / 255.0;
                true
            }
            Msg::B(CallbackType::Byte(value)) => {
                ctx.props().color_mut().b = value as f32 / 255.0;
                true
            }
            Msg::A(CallbackType::Byte(value)) => {
                ctx.props().color_mut().a = value as f32 / 255.0;
                true
            }
            Msg::Change(event) => {
                let input: HtmlInputElement = event.target_unchecked_into();
                let color = input.value();

                let hex_to_rgb = |hex: String| {
                    ensure!(hex.len() == 7, "invalid color");

                    let r = u8::from_str_radix(&hex[1..3], 16)?;
                    let g = u8::from_str_radix(&hex[3..5], 16)?;
                    let b = u8::from_str_radix(&hex[5..7], 16)?;
                    Ok((r, g, b))
                };

                if let Ok((r, g, b)) = hex_to_rgb(color) {
                    let mut color = ctx.props().color_mut();
                    color.r = r as f32 / 255.0;
                    color.g = g as f32 / 255.0;
                    color.b = b as f32 / 255.0;
                    true
                } else {
                    false
                }
            }
            _ => unreachable!(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (r, g, b, a) = {
            let colors = ctx.props().color();
            (
                (colors.r * 255.0) as u8,
                (colors.g * 255.0) as u8,
                (colors.b * 255.0) as u8,
                (colors.a * 255.0) as u8,
            )
        };
        let hex_color = format!("#{:02X}{:02X}{:02X}", r, g, b);

        html! {
            <div class="flex gap-2">
                <InputNumber label="R" value={NumberType::Byte(r.into())} onchange={ctx.link().callback(Msg::R)} />
                <InputNumber label="G" value={NumberType::Byte(g.into())} onchange={ctx.link().callback(Msg::G)} />
                <InputNumber label="B" value={NumberType::Byte(b.into())} onchange={ctx.link().callback(Msg::B)} />
                <InputNumber label="A" value={NumberType::Byte(a.into())} onchange={ctx.link().callback(Msg::A)} />
                <label class="flex-auto flex items-center gap-1">
                    <span class="border border-default-border w-5 h-5" style={format!("background-color: {}", hex_color)}>
                        <input type="color"
                            class="opacity-0"
                            value={hex_color}
                            onchange={ctx.link().callback(Msg::Change)}
                        />
                    </span>
                    { &ctx.props().label }
                </label>
            </div>
        }
    }
}
