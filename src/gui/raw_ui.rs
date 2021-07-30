use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::{self, Display},
    rc::Rc,
};
use yew::prelude::*;

use crate::{
    gui::component::*,
    save_data::{
        shared::{appearance::LinearColor, plot::BoolVec},
        Guid,
    },
};

pub trait RawUi
where
    Self: Clone + 'static,
{
    fn view(&self, label: &str) -> yew::Html;
    fn view_opened(&self, label: &str, _opened: bool) -> yew::Html {
        self.view(label)
    }
}

// pub trait RawUiMe1Legacy {
//     fn draw_fields<'a>(&'a mut self, gui: &'a Gui) -> Vec<Box<dyn FnMut() + 'a>>;
// }

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct RcUi<T>(Rc<RefCell<T>>);

impl<T> RcUi<T> {
    pub fn new(inner: T) -> Self {
        RcUi(Rc::new(RefCell::new(inner)))
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        RefCell::borrow(&*self.0)
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        RefCell::borrow_mut(&*self.0)
    }

    pub fn ptr_eq(&self, other: &RcUi<T>) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: Display> Display for RcUi<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.borrow().fmt(f)
    }
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
        html! {
            <RawUiVec<T> label=label.to_owned() vec=RcUi::clone(self) />
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

impl<T> RawUi for Box<T>
where
    T: RawUi,
{
    fn view(&self, _label: &str) -> yew::Html {
        todo!()
    }
}

// Shared
impl RawUi for RcUi<BoolVec> {
    fn view(&self, label: &str) -> yew::Html {
        // TODO
        html! {
            {label}
        }
    }
}

impl RawUi for RcUi<Guid> {
    fn view(&self, label: &str) -> yew::Html {
        // TODO
        html! {
            {label}
        }
    }
}

impl RawUi for RcUi<LinearColor> {
    fn view(&self, label: &str) -> yew::Html {
        // TODO
        html! {
            {label}
        }
    }
}
