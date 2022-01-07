use yew::prelude::*;

use crate::gui::{components::Select, RcUi};

#[derive(Properties)]
pub struct Props<T>
where
    T: From<usize> + Into<usize> + Clone + 'static,
{
    pub label: String,
    pub items: &'static [&'static str],
    pub value: RcUi<T>,
}

impl<T> PartialEq for Props<T>
where
    T: From<usize> + Into<usize> + Clone + 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && self.items == other.items && self.value == other.value
    }
}

#[function_component(RawUiEnum)]
pub fn raw_ui_enum<T>(props: &Props<T>) -> Html
where
    T: From<usize> + Into<usize> + Clone + 'static,
{
    let options = props.items;
    let current_idx: usize = props.value.borrow().clone().into();
    let onselect = {
        let value = RcUi::clone(&props.value);
        Callback::from(move |idx| *value.borrow_mut() = T::from(idx))
    };
    html! {
        <div class="flex items-center gap-1 cursor-default">
            <Select {options} {current_idx} {onselect} />
            { &props.label }
        </div>
    }
}
