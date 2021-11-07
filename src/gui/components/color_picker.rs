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

#[derive(Properties, PartialEq)]
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
            Msg::R(CallbackType::Float(value)) => {
                ctx.props().color_mut().r = value;
                true
            }
            Msg::G(CallbackType::Float(value)) => {
                ctx.props().color_mut().g = value;
                true
            }
            Msg::B(CallbackType::Float(value)) => {
                ctx.props().color_mut().b = value;
                true
            }
            Msg::A(CallbackType::Float(value)) => {
                ctx.props().color_mut().a = value;
                true
            }
            Msg::Change(event) => {
                if let Some(input) = event.target_dyn_into::<HtmlInputElement>() {
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
                        return true;
                    }
                }
                false
            }
            _ => unreachable!(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (r, g, b, a) = {
            let colors = ctx.props().color();
            (colors.r, colors.g, colors.b, colors.a)
        };
        let hex_color = {
            let max_color = r.max(g.max(b));
            let ratio = max_color.max(1.0);
            let r = (r * 255.0 / ratio) as u8;
            let g = (g * 255.0 / ratio) as u8;
            let b = (b * 255.0 / ratio) as u8;
            format!("#{:02X}{:02X}{:02X}", r, g, b)
        };

        html! {
            <div class="flex gap-2">
                <InputNumber label="R" value={NumberType::Float(r.into())} onchange={ctx.link().callback(Msg::R)} />
                <InputNumber label="G" value={NumberType::Float(g.into())} onchange={ctx.link().callback(Msg::G)} />
                <InputNumber label="B" value={NumberType::Float(b.into())} onchange={ctx.link().callback(Msg::B)} />
                <InputNumber label="A" value={NumberType::Float(a.into())} onchange={ctx.link().callback(Msg::A)} />
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
