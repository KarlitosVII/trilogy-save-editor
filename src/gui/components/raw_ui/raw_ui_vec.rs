use std::{
    any::Any,
    cell::{Ref, RefMut},
    fmt::Display,
    marker::PhantomData,
};

use yew::prelude::*;

use crate::{
    gui::{components::Table, raw_ui::RawUi},
    save_data::RcRef,
};

pub enum Msg {
    Toggle,
    Add,
    Remove(usize),
}

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: RawUi + Default + Display,
{
    pub label: String,
    pub vec: RcRef<Vec<T>>,
    #[prop_or(true)]
    pub is_editable: bool,
}

impl<T> Props<T>
where
    T: RawUi + Default + Display,
{
    fn vec(&self) -> Ref<'_, Vec<T>> {
        self.vec.borrow()
    }

    fn vec_mut(&self) -> RefMut<'_, Vec<T>> {
        self.vec.borrow_mut()
    }
}

pub struct RawUiVec<T>
where
    T: RawUi + Default + Display,
{
    _marker: PhantomData<T>,
    opened: bool,
    new_item_idx: usize,
}

impl<T> Component for RawUiVec<T>
where
    T: RawUi + Default + Display,
{
    type Message = Msg;
    type Properties = Props<T>;

    fn create(_ctx: &Context<Self>) -> Self {
        RawUiVec { _marker: PhantomData, opened: false, new_item_idx: 0 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle => {
                self.opened = !self.opened;
                if self.opened {
                    // Prevent last item to reopen
                    self.new_item_idx = ctx.props().vec().len();
                }
                true
            }
            Msg::Add => {
                // Open added item
                self.new_item_idx = ctx.props().vec().len();

                ctx.props().vec_mut().push(Default::default());
                true
            }
            Msg::Remove(idx) => {
                ctx.props().vec_mut().remove(idx);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let chevron = if self.opened { "table-chevron-down" } else { "table-chevron-right" };

        let content = self
            .opened
            .then(|| {
                let vec = ctx.props().vec();
                let is_editable = ctx.props().is_editable;

                // Exceptions
                macro_rules! display_idx {
                    ($vec:ident => $($type:ty)*) => {
                        $((&*$vec as &dyn Any).is::<Vec<RcRef<$type>>>()) ||*
                    }
                }
                let display_idx = display_idx!(vec => u8 i32 f32 bool String);

                let items = vec.iter().enumerate().map(|(idx, item)| {
                    let label = item.to_string();
                    let opened = self.new_item_idx == idx;
                    let item = if display_idx || label.is_empty() {
                        item.view_opened(&idx.to_string(), opened)
                    } else {
                        item.view_opened(&label, opened)
                    };

                    let remove = is_editable.then(|| html!{
                        <div class="py-px">
                            <a class={classes![
                                    "rounded-none",
                                    "select-none",
                                    "hover:bg-theme-hover",
                                    "active:bg-theme-active",
                                    "bg-theme-bg",
                                    "px-1",
                                    "py-0",
                                    "cursor-pointer",
                                ]}
                                onclick={ctx.link().callback(move |_| Msg::Remove(idx))}
                            >
                                {"remove"}
                            </a>
                        </div>
                    });

                    html! {
                        <div class="flex gap-1">
                            { for remove }
                            { item }
                        </div>
                    }
                });

                let empty = (!is_editable && vec.is_empty()).then(|| html!{ "<empty>" }).into_iter();

                let add = is_editable.then(|| html!{
                    <button class="rounded-none hover:bg-theme-hover active:bg-theme-active bg-theme-bg px-1"
                        onclick={ctx.link().callback(|_| Msg::Add)}
                    >
                        {"add"}
                    </button>
                }).into_iter();

                html! {
                    <div class="p-1">
                        <Table>
                            { for items }
                            { for empty }
                            { for add }
                        </Table>
                    </div>
                }
            });

        html! {
            <div class="flex-auto flex flex-col">
                <div class="p-px">
                    <button class={classes![
                            "rounded-none",
                            "hover:bg-theme-hover",
                            "active:bg-theme-active",
                            "px-1",
                            "pl-6",
                            "w-full",
                            "text-left",
                            chevron,
                        ]}
                        onclick={ctx.link().callback(|_| Msg::Toggle)}
                    >
                        { &ctx.props().label }
                    </button>
                </div>
                { for content }
            </div>
        }
    }
}
