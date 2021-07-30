use derive_more::From;
use indexmap::IndexMap;
use yew::prelude::*;

use crate::gui::{
    component::{CallbackType, InputNumber, InputText, NumberType, RawUiStruct, Table},
    RawUi, RcUi,
};

#[derive(Clone, From)]
pub enum IndexMapKeyType<T>
where
    T: RawUi + Default,
{
    I32(RcUi<IndexMap<i32, T>>),
    String(RcUi<IndexMap<String, T>>),
}

impl<T> PartialEq for IndexMapKeyType<T>
where
    T: RawUi + Default,
{
    fn eq(&self, other: &IndexMapKeyType<T>) -> bool {
        match self {
            IndexMapKeyType::I32(index_map) => {
                if let IndexMapKeyType::I32(other) = other {
                    index_map.ptr_eq(other)
                } else {
                    false
                }
            }
            IndexMapKeyType::String(index_map) => {
                if let IndexMapKeyType::String(other) = other {
                    index_map.ptr_eq(other)
                } else {
                    false
                }
            }
        }
    }
}

pub enum Msg {
    Toggle,
    Add,
    Remove(usize),
    EditKey(usize, CallbackType),
}

#[derive(Properties, Clone)]
pub struct Props<T>
where
    T: RawUi + Default,
{
    pub label: String,
    pub index_map: IndexMapKeyType<T>,
}

pub struct RawUiIndexMap<T>
where
    T: RawUi + Default,
{
    props: Props<T>,
    link: ComponentLink<Self>,
    opened: bool,
    new_item_idx: usize,
}

impl<T> Component for RawUiIndexMap<T>
where
    T: RawUi + Default,
{
    type Message = Msg;
    type Properties = Props<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        RawUiIndexMap { props, link, opened: false, new_item_idx: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                self.opened = !self.opened;
                if self.opened {
                    // Prevent last item to reopen
                    self.new_item_idx = match self.props.index_map {
                        IndexMapKeyType::I32(ref index_map) => index_map.borrow().len(),
                        IndexMapKeyType::String(ref index_map) => index_map.borrow().len(),
                    };
                }
                true
            }
            Msg::Add => {
                match self.props.index_map {
                    IndexMapKeyType::I32(ref index_map) => {
                        // Open added item
                        self.new_item_idx = index_map.borrow().len();

                        index_map.borrow_mut().entry(-1).or_default();
                    }
                    IndexMapKeyType::String(ref index_map) => {
                        // Open added item
                        self.new_item_idx = index_map.borrow().len();

                        index_map.borrow_mut().entry(Default::default()).or_default();
                    }
                }
                true
            }
            Msg::Remove(idx) => {
                match self.props.index_map {
                    IndexMapKeyType::I32(ref index_map) => {
                        index_map.borrow_mut().shift_remove_index(idx);
                    }
                    IndexMapKeyType::String(ref index_map) => {
                        index_map.borrow_mut().shift_remove_index(idx);
                    }
                }
                true
            }
            Msg::EditKey(idx, new_key) => match self.props.index_map {
                IndexMapKeyType::I32(ref index_map) => match new_key {
                    CallbackType::Integer(new_key) => {
                        if let Some((key, _)) = index_map.borrow_mut().get_index_mut(idx) {
                            *key = new_key;
                        }
                        true
                    }
                    _ => false,
                },
                IndexMapKeyType::String(ref index_map) => match new_key {
                    CallbackType::String(new_key) => {
                        if let Some((key, _)) = index_map.borrow_mut().get_index_mut(idx) {
                            *key = new_key;
                        }
                        true
                    }
                    _ => false,
                },
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let Props { label, index_map } = &props;
        if self.props.label != *label || self.props.index_map != *index_map {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let chevron = if self.opened { "table-chevron-down" } else { "table-chevron-right" };

        let content = self
            .opened
            .then(|| {
                let item = |idx, label, key, value| {
                    html_nested! {
                        <div class="flex gap-1">
                            <div class="py-px">
                                <a class="rounded-none select-none hover:bg-theme-hover active:bg-theme-active bg-theme-bg px-1 py-0 cursor-pointer"
                                    onclick=self.link.callback(move |_| Msg::Remove(idx))
                                >
                                    {"remove"}
                                </a>
                            </div>
                            <RawUiStruct label=label opened=self.new_item_idx == idx>
                                { key }
                                { RawUi::view(value, "Value") }
                            </RawUiStruct>
                        </div>
                    }
                };

                let items = match self.props.index_map {
                    IndexMapKeyType::I32(ref index_map) => index_map
                        .borrow()
                        .iter()
                        .enumerate()
                        .map(|(idx, (key, value))| {
                            let input_k = html!{
                                <InputNumber label=String::from("Key") value=NumberType::Integer(RcUi::new(*key))
                                    onchange=self.link.callback(move |callback| Msg::EditKey(idx, callback))
                                />
                            };
                            item(idx, key.to_string(), input_k, value)
                        })
                        .collect::<Html>(),
                    IndexMapKeyType::String(ref index_map) => index_map
                        .borrow()
                        .iter()
                        .enumerate()
                        .map(|(idx, (key, value))| {
                            let input_k = html!{
                                <InputText label=String::from("Key") value=RcUi::new(key.to_owned())
                                    oninput=self.link.callback(move |callback| Msg::EditKey(idx, callback))
                                />
                            };
                            let label = if !key.is_empty() {
                                key.to_owned()
                            } else {
                                String::from("<empty>")
                            };
                            item(idx, label, input_k, value)
                        })
                        .collect::<Html>(),
                };

                html! {
                    <div class="p-1">
                        <Table>
                            { items }
                            <button class="rounded-none hover:bg-theme-hover active:bg-theme-active bg-theme-bg px-1"
                                onclick=self.link.callback(|_| Msg::Add)
                            >
                                {"add"}
                            </button>
                        </Table>
                    </div>
                }
            })
            .unwrap_or_default();

        html! {
            <div class="flex-auto flex flex-col">
                <div class="p-px">
                    <button
                        class=classes![
                            "rounded-none",
                            "hover:bg-theme-hover",
                            "active:bg-theme-active",
                            "px-1",
                            "pl-6",
                            "w-full",
                            "text-left",
                            chevron,
                        ]
                        onclick=self.link.callback(|_| Msg::Toggle)
                    >
                        { &self.props.label }
                    </button>
                </div>
                { content }
            </div>
        }
    }
}
