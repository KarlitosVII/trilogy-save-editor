use std::{any::Any, fmt::Display};

use indexmap::IndexMap;
use yew::prelude::*;

use crate::{
    gui::components::{raw_ui::*, *},
    save_data::{
        mass_effect_1_le::legacy::BaseObject, shared::appearance::LinearColor, Guid, RcRef,
    },
};

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
impl RawUi for RcRef<u8> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <InputNumber label={label.to_owned()} value={NumberType::Byte(RcRef::clone(self))} />
        }
    }
}

impl RawUi for RcRef<i32> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <InputNumber label={label.to_owned()} value={NumberType::Int(RcRef::clone(self))} />
        }
    }
}

impl RawUi for RcRef<f32> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <InputNumber label={label.to_owned()} value={NumberType::Float(RcRef::clone(self))} />
        }
    }
}

impl RawUi for RcRef<bool> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <CheckBox label={label.to_owned()} value={RcRef::clone(self)} />
        }
    }
}

impl RawUi for RcRef<String> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <InputText label={label.to_owned()} value={RcRef::clone(self)} />
        }
    }
}

impl<T> RawUi for RcRef<Option<T>>
where
    T: RawUi,
{
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <RawUiOption<T> label={label.to_owned()} option={RcRef::clone(self)} />
        }
    }
}

impl<T> RawUi for RcRef<Vec<T>>
where
    T: RawUi + Default + Display,
{
    fn view(&self, label: &str) -> yew::Html {
        // Make Vec of BaseObject not editable
        let is_editable = !(self as &dyn Any).is::<RcRef<Vec<RcRef<BaseObject>>>>();
        html! {
            <RawUiVec<T> label={label.to_owned()} vec={RcRef::clone(self)} {is_editable} />
        }
    }
}

impl<K, V> RawUi for RcRef<IndexMap<K, V>>
where
    K: Clone + 'static,
    V: RawUi + Default,
    RcRef<IndexMap<K, V>>: Into<IndexMapKeyType<V>>,
{
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <RawUiIndexMap<V> label={label.to_owned()} index_map={RcRef::clone(self).into()} />
        }
    }
}

// Shared
impl RawUi for RcRef<Guid> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <RawUiGuid label={label.to_owned()} guid={RcRef::clone(self)} />
        }
    }
}

impl RawUi for RcRef<LinearColor> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <ColorPicker label={label.to_owned()} color={RcRef::clone(self)} />
        }
    }
}
