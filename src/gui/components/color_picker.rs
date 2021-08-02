use anyhow::bail;
use std::cell::{Ref, RefMut};
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::{
    gui::{
        components::{CallbackType, InputNumber, NumberType},
        RcUi,
    },
    save_data::shared::appearance::LinearColor,
};

pub enum Msg {
    R(CallbackType),
    G(CallbackType),
    B(CallbackType),
    A(CallbackType),
    Change(ChangeData),
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

    fn color_mut(&mut self) -> RefMut<'_, LinearColor> {
        self.color.borrow_mut()
    }
}

pub struct ColorPicker {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for ColorPicker {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ColorPicker { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::R(CallbackType::Byte(value)) => {
                self.props.color_mut().r = value as f32 / 255.0;
                true
            }
            Msg::G(CallbackType::Byte(value)) => {
                self.props.color_mut().g = value as f32 / 255.0;
                true
            }
            Msg::B(CallbackType::Byte(value)) => {
                self.props.color_mut().b = value as f32 / 255.0;
                true
            }
            Msg::A(CallbackType::Byte(value)) => {
                self.props.color_mut().a = value as f32 / 255.0;
                true
            }
            Msg::Change(ChangeData::Value(color)) => {
                let hex_to_rgb = |hex: String| {
                    if hex.len() != 7 {
                        bail!("invalid color");
                    }

                    let r = u8::from_str_radix(&hex[1..3], 16)?;
                    let g = u8::from_str_radix(&hex[3..5], 16)?;
                    let b = u8::from_str_radix(&hex[5..7], 16)?;
                    Ok((r, g, b))
                };

                if let Ok((r, g, b)) = hex_to_rgb(color) {
                    let mut color = self.props.color_mut();
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let (r, g, b, a) = {
            let colors = self.props.color();
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
                <InputNumber label="R" value=NumberType::Byte(RcUi::new(r)) onchange=self.link.callback(Msg::R) />
                <InputNumber label="G" value=NumberType::Byte(RcUi::new(g)) onchange=self.link.callback(Msg::G) />
                <InputNumber label="B" value=NumberType::Byte(RcUi::new(b)) onchange=self.link.callback(Msg::B) />
                <InputNumber label="A" value=NumberType::Byte(RcUi::new(a)) onchange=self.link.callback(Msg::A) />
                <label class="flex-auto flex items-center gap-1">
                    <span class="border border-default-border w-5 h-5" style=format!("background-color: {}", hex_color)>
                        <input type="color"
                            class="opacity-0"
                            value=hex_color
                            onchange=self.link.callback(Msg::Change)
                        />
                    </span>
                    { &self.props.label }
                </label>
            </div>
        }
    }
}
