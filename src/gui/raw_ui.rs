use indexmap::IndexMap;
use std::{any::Any, fmt::Display};
use yew::prelude::*;

use crate::{
    gui::components::{raw_ui::*, *},
    save_data::{mass_effect_1_le::legacy::BaseObject, shared::appearance::LinearColor, Guid},
};

use super::RcUi;

pub trait RawUi
where
    Self: Clone + PartialEq + 'static,
{
    fn view(&self, label: &str) -> yew::Html;
    fn view_opened(&self, label: &str, _opened: bool) -> yew::Html {
        self.view(label)
    }
}

pub trait RawUiChildren
where
    Self: Clone + PartialEq + 'static,
{
    fn children(&self) -> Vec<yew::Html>;
}

// Implémentation des types std
impl RawUi for RcUi<u8> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <InputNumber label=label.to_owned() value=NumberType::Byte(RcUi::clone(self)) />
        }
    }
}

impl RawUi for RcUi<i32> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <InputNumber label=label.to_owned() value=NumberType::Integer(RcUi::clone(self)) />
        }
    }
}

impl RawUi for RcUi<f32> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <InputNumber label=label.to_owned() value=NumberType::Float(RcUi::clone(self)) />
        }
    }
}

impl RawUi for RcUi<bool> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <CheckBox label=label.to_owned() value=RcUi::clone(self) />
        }
    }
}

impl RawUi for RcUi<String> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <InputText label=label.to_owned() value=RcUi::clone(self) />
        }
    }
}

impl<T> RawUi for RcUi<Option<T>>
where
    T: RawUi,
{
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <RawUiOption<T> label=label.to_owned() option=RcUi::clone(self) />
        }
    }
}

impl<T> RawUi for RcUi<Vec<T>>
where
    T: RawUi + Default + Display,
{
    fn view(&self, label: &str) -> yew::Html {
        // Make Vec of BaseObject not editable
        let is_editable = !(self as &dyn Any).is::<RcUi<Vec<RcUi<BaseObject>>>>();
        html! {
            <RawUiVec<T> label=label.to_owned() vec=RcUi::clone(self) is_editable=is_editable />
        }
    }
}

impl<K, V> RawUi for RcUi<IndexMap<K, V>>
where
    K: Clone + 'static,
    V: RawUi + Default,
    RcUi<IndexMap<K, V>>: Into<IndexMapKeyType<V>>,
{
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <RawUiIndexMap<V> label=label.to_owned() index_map=RcUi::clone(self).into() />
        }
    }
}

// Shared
impl RawUi for RcUi<Guid> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <RawUiGuid label=label.to_owned() guid=RcUi::clone(self) />
        }
    }
}

impl RawUi for RcUi<LinearColor> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <ColorPicker label=label.to_owned() color=RcUi::clone(self) />
        }
    }
}
