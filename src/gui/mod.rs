use std::{cell::{Ref, RefCell, RefMut}, fmt::{self, Display}, rc::Rc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

mod app;
pub mod components;
mod mass_effect_1;
mod mass_effect_2;
pub mod raw_ui;

pub use self::app::*;

// RcUi
#[derive(Clone, Default)]
pub struct RcUi<T>(Rc<RefCell<T>>);

impl<T> RcUi<T> {
    pub fn new(inner: T) -> Self {
        RcUi(Rc::new(RefCell::new(inner)))
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        RefCell::borrow(&self.0)
    }

    pub fn borrow_mut(&mut self) -> RefMut<'_, T> {
        RefCell::borrow_mut(&self.0)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for RcUi<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner: T = Deserialize::deserialize(deserializer)?;
        Ok(RcUi::new(inner))
    }
}

impl<T: Serialize> serde::Serialize for RcUi<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.borrow().serialize(serializer)
    }
}

impl<T> PartialEq for RcUi<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: Display> Display for RcUi<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.borrow().fmt(f)
    }
}

#[derive(Clone, PartialEq)]
pub enum Theme {
    MassEffect1,
    MassEffect2,
    MassEffect3,
}

impl From<Theme> for yew::Classes {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::MassEffect1 => "mass-effect-1",
            Theme::MassEffect2 => "mass-effect-2",
            Theme::MassEffect3 => "mass-effect-3",
        }
        .into()
    }
}
