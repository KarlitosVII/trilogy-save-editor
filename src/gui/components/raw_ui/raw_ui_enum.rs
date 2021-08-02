use std::cell::{Ref, RefMut};
use yew::prelude::*;

use crate::gui::{components::Select, RcUi};

pub enum Msg {
    Changed(usize),
}

#[derive(Properties, Clone)]
pub struct Props<T>
where
    T: From<usize> + Into<usize> + Clone + 'static,
{
    pub label: String,
    pub items: &'static [&'static str],
    pub value: RcUi<T>,
}

impl<T> Props<T>
where
    T: From<usize> + Into<usize> + Clone + 'static,
{
    fn value(&self) -> Ref<'_, T> {
        self.value.borrow()
    }

    fn value_mut(&self) -> RefMut<'_, T> {
        self.value.borrow_mut()
    }
}

pub struct RawUiEnum<T>
where
    T: From<usize> + Into<usize> + Clone + 'static,
{
    props: Props<T>,
    link: ComponentLink<Self>,
}

impl<T> Component for RawUiEnum<T>
where
    T: From<usize> + Into<usize> + Clone + 'static,
{
    type Message = Msg;
    type Properties = Props<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        RawUiEnum { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Changed(idx) => {
                *self.props.value_mut() = T::from(idx);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let Props { label, items, value } = &props;
        if self.props.label != *label || self.props.items != *items || self.props.value != *value {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let current_idx: usize = self.props.value().clone().into();
        html! {
            <div class="flex items-center gap-1 cursor-default">
                <Select
                    options=self.props.items
                    current_idx=current_idx
                    onselect=self.link.callback(Msg::Changed)
                />
                { &self.props.label }
            </div>
        }
    }
}
