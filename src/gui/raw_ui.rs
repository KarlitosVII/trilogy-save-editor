use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Display,
    hash::Hash,
    rc::Rc,
};
use yew::prelude::*;

use crate::{
    gui::component::*,
    save_data::shared::{appearance::LinearColor, plot::BoolVec, Guid},
};

pub trait RawUi {
    fn view(&self, label: &str) -> yew::Html;
}

// pub trait RawUiMe1Legacy {
//     fn draw_fields<'a>(&'a mut self, gui: &'a Gui) -> Vec<Box<dyn FnMut() + 'a>>;
// }

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct RcUi<T>(Rc<RefCell<T>>);

impl<T> RcUi<T> {
    pub fn new(inner: T) -> Self {
        RcUi(Rc::new(RefCell::new(inner)))
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        (*self.0).borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        (*self.0).borrow_mut()
    }
}

// Implémentation des types std
impl RawUi for RcUi<u8> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <InputNumber label=label.to_owned() value=NumberType::Byte(self.clone()) />
        }
    }
}

impl RawUi for RcUi<i32> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <InputNumber label=label.to_owned() value=NumberType::Integer(self.clone()) />
        }
    }
}

impl RawUi for RcUi<f32> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <InputNumber label=label.to_owned() value=NumberType::Float(self.clone()) />
        }
    }
}

impl RawUi for RcUi<bool> {
    fn view(&self, _label: &str) -> yew::Html {
        todo!()
    }
}

impl RawUi for RcUi<String> {
    fn view(&self, label: &str) -> yew::Html {
        html! {
            <InputText label=label.to_owned() value=self.clone() />
        }
    }
}

impl<T> RawUi for RcUi<Option<T>>
where
    T: RawUi,
{
    fn view(&self, _label: &str) -> yew::Html {
        todo!()
    }
}

impl<T> RawUi for RcUi<Vec<T>>
where
    T: RawUi + Default,
{
    fn view(&self, _label: &str) -> yew::Html {
        todo!()
    }
}

impl<K, V> RawUi for RcUi<IndexMap<K, V>>
where
    // K: RawUi + Eq + Hash + Default + Display + 'static,
    K: Eq + Hash + Display,
    V: RawUi + Default,
{
    fn view(&self, _label: &str) -> yew::Html {
        todo!()
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
    fn view(&self, _label: &str) -> yew::Html {
        todo!()
    }
}

impl RawUi for RcUi<Guid> {
    fn view(&self, _label: &str) -> yew::Html {
        todo!()
    }
}

impl RawUi for RcUi<LinearColor> {
    fn view(&self, _label: &str) -> yew::Html {
        todo!()
    }
}
