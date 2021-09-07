use std::{
    cell::{Ref, RefMut},
    marker::PhantomData,
};

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

impl<T> PartialEq for Props<T>
where
    T: From<usize> + Into<usize> + Clone + 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && self.items == other.items && self.value == other.value
    }
}

pub struct RawUiEnum<T>
where
    T: From<usize> + Into<usize> + Clone + 'static,
{
    _marker: PhantomData<T>,
}

impl<T> Component for RawUiEnum<T>
where
    T: From<usize> + Into<usize> + Clone + 'static,
{
    type Message = Msg;
    type Properties = Props<T>;

    fn create(_ctx: &Context<Self>) -> Self {
        RawUiEnum { _marker: PhantomData }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Changed(idx) => {
                *ctx.props().value_mut() = T::from(idx);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let options = ctx.props().items;
        let current_idx: usize = ctx.props().value().clone().into();
        let onselect = ctx.link().callback(Msg::Changed);
        html! {
            <div class="flex items-center gap-1 cursor-default">
                <Select {options} {current_idx} {onselect} />
                { &ctx.props().label }
            </div>
        }
    }
}
