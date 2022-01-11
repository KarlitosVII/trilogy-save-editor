mod app;
pub mod components;
mod mass_effect_1;
mod mass_effect_1_le;
mod mass_effect_2;
mod mass_effect_3;
pub mod raw_ui;
pub mod shared;

pub use self::app::*;

use std::ops::Deref;

use yew::{html, Html};

#[derive(Copy, Clone, PartialEq)]
pub enum Theme {
    MassEffect1,
    MassEffect2,
    MassEffect3,
}

impl Deref for Theme {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Theme::MassEffect1 => "mass-effect-1",
            Theme::MassEffect2 => "mass-effect-2",
            Theme::MassEffect3 => "mass-effect-3",
        }
    }
}

impl From<Theme> for yew::Classes {
    fn from(theme: Theme) -> Self {
        theme.to_string().into()
    }
}

pub fn format_code(text: impl AsRef<str>) -> Html {
    let text = text.as_ref().split('`').enumerate().map(|(i, text)| {
        if i % 2 != 0 {
            html! { <span class="bg-default-border px-1 py-px rounded-sm">{ text }</span>}
        } else {
            html! { text }
        }
    });
    html! { for text }
}
