use std::rc::Rc;

use gloo::timers::future::TimeoutFuture;
use indexmap::IndexMap;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

use crate::save_data::mass_effect_1_le::item_db::{DbItem, Me1ItemDb};

pub enum Msg {
    Scrolled,
    Open,
    ShouldClose,
    Focused,
    Blurred,
    BlurAll,
    Filter(InputEvent),
    Select(DbItem),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub item_db: Rc<Me1ItemDb>,
    pub current_item: DbItem,
    pub onselect: Callback<DbItem>,
}

pub struct ItemSelect {
    select_ref: NodeRef,
    drop_down_ref: NodeRef,
    scroll_ref: NodeRef,
    filter_ref: NodeRef,
    current_item: DbItem,
    focused: bool,
    opened: bool,
    is_opening: bool,
    row_height: i32,
    skip: usize,
    take: usize,
    filter: String,
    filtered_list: Option<Me1ItemDb>,
}

impl Component for ItemSelect {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ItemSelect {
            select_ref: Default::default(),
            drop_down_ref: Default::default(),
            scroll_ref: Default::default(),
            filter_ref: Default::default(),
            current_item: ctx.props().current_item,
            focused: false,
            opened: false,
            is_opening: false,
            row_height: 20,
            skip: 0,
            take: 0,
            filter: Default::default(),
            filtered_list: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Scrolled => {
                if let Some(scroll) = self.scroll_ref.cast::<HtmlElement>() {
                    let scroll_top = scroll.scroll_top();
                    let offset_height = 300;
                    let num_rows = offset_height / self.row_height + 2;

                    let len = self.filtered_list.as_ref().unwrap_or(&ctx.props().item_db).len();
                    let start = scroll_top / self.row_height;
                    self.skip = start.max(0) as usize;
                    self.take = num_rows.min(len as i32) as usize;
                }
                true
            }
            Msg::Open => {
                self.opened = true;
                self.is_opening = true;
                true
            }
            Msg::ShouldClose => {
                if !self.focused {
                    self.opened = false;
                    if let Some(drop_down) = self.drop_down_ref.cast::<HtmlElement>() {
                        let _ = drop_down.style().set_property("bottom", "auto");
                        let _ = drop_down.style().set_property("right", "auto");
                    }
                    true
                } else {
                    false
                }
            }
            Msg::Focused => {
                self.focused = true;
                false
            }
            Msg::Blurred => {
                self.focused = false;

                // The drop down blur just before the filter focus
                // so we delay the check until next event loop
                // to avoid closing when clicking into filter's input
                ctx.link().send_future(async {
                    TimeoutFuture::new(0).await;
                    Msg::ShouldClose
                });
                false
            }
            Msg::BlurAll => {
                if let Some(select) = self.select_ref.cast::<HtmlElement>() {
                    let _ = select.blur();
                }
                if let Some(filter) = self.filter_ref.cast::<HtmlElement>() {
                    let _ = filter.blur();
                }
                false
            }
            Msg::Filter(event) => {
                let input: HtmlInputElement = event.target_unchecked_into();
                let filter = input.value();

                if !filter.is_empty() {
                    let filter = filter.to_lowercase();
                    let filtered_list = ctx
                        .props()
                        .item_db
                        .iter()
                        .filter_map(|(k, v)| {
                            (v.to_lowercase().contains(&filter)).then(|| (*k, v.clone()))
                        })
                        .collect::<IndexMap<_, _>>();
                    self.filtered_list = Some(filtered_list.into());
                } else {
                    self.filtered_list = None;
                }
                self.filter = filter;
                ctx.link().send_message(Msg::Scrolled);
                false
            }
            Msg::Select(key) => {
                self.current_item = key;
                ctx.props().onselect.emit(key);
                ctx.link().send_message(Msg::BlurAll);
                false
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.current_item = ctx.props().current_item;
        if self.opened {
            ctx.link().send_message(Msg::BlurAll);
            false
        } else {
            true
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if self.is_opening {
            self.is_opening = false;

            // Focus the filter when opened
            if let Some(filter) = self.filter_ref.cast::<HtmlElement>() {
                let _ = filter.focus();
            }

            if let Some(drop_down) = self.drop_down_ref.cast::<HtmlElement>() {
                let document = yew::utils::document().document_element().unwrap();
                let viewport_height = document.client_height();
                let client_rect = drop_down.get_bounding_client_rect();

                // Drop down open upward if bottom > viewport_height
                let top = client_rect.top() as i32;
                let bottom = client_rect.bottom() as i32;
                let height = bottom - top;

                if height < top - 70 && bottom > viewport_height - 10 {
                    if let Some(select) = self.select_ref.cast::<HtmlElement>() {
                        let height = select.offset_height();
                        let _ = drop_down.style().set_property("bottom", &format!("{}px", height));
                    }
                }

                // Keep the drop down in the viewport
                let viewport_width = document.client_width();
                let width = client_rect.width() as i32;
                let left = client_rect.left() as i32;
                let right = left + width;

                if right > viewport_width - 20 {
                    let _ = drop_down.style().set_property("right", "0");
                }
            }

            ctx.link().send_message(Msg::Scrolled);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let current_item_name = ctx
            .props()
            .item_db
            .get(&self.current_item)
            .map(|item| item.as_str())
            .unwrap_or_else(|| "Unknown item");

        let item_db = self.filtered_list.as_ref().unwrap_or(&ctx.props().item_db);
        let options = (self.opened && !self.is_opening).then(|| {
            let options = item_db.iter().skip(self.skip).take(self.take).map(|(&key, option)| {
                let selected = key == self.current_item;
                html_nested! {
                    <a
                        class={classes![
                            "flex-1",
                            "px-1",
                            "hover:bg-theme-hover",
                            "active:bg-theme-active",
                            "cursor-pointer",
                            "whitespace-nowrap",
                            selected.then(|| "bg-theme-bg"),
                        ]}
                        onclick={ctx.link().callback(move |_| Msg::Select(key))}
                    >
                        { option }
                    </a>
                }
            });

            html! {
                <div class="flex flex-col"
                    style={format!("will-change: transform; transform: translate3d(0, {}px, 0)", self.skip * self.row_height as usize)}
                >
                    { for options }
                </div>
            }
        });

        let onclick = if !self.opened {
            ctx.link().callback(|_| Msg::Open)
        } else {
            ctx.link().callback(|_| Msg::BlurAll)
        };

        html! {
            <div class="relative flex-auto select-none min-w-0" tabindex="0"
                onfocus={ctx.link().callback(|_| Msg::Focused)}
                onblur={self.opened.then(||ctx.link().callback(|_| Msg::Blurred))}
                ref={self.select_ref.clone()}
            >
                <div class="overflow-hidden">
                    <a class={classes![
                            "block",
                            "bg-theme-bg",
                            "hover:bg-theme-hover",
                            "active:bg-theme-active",
                            "px-1",
                            "cursor-pointer",
                            "select-chevron",
                            "truncate",
                        ]}
                        {onclick}
                    >
                        { current_item_name }
                    </a>
                    <div class={classes![
                            "absolute",
                            "flex",
                            "flex-col",
                            "bg-popup/95",
                            "border",
                            "border-default-border",
                            "min-w-full",
                            "w-[410px]",
                            "max-h-[300px]",
                            "z-20",
                            (!self.opened).then(|| "hidden")
                        ]}
                        ref={self.drop_down_ref.clone()}
                    >
                        <label class="flex items-center gap-1 p-px pr-1">
                            <input type="text" class="flex-auto input" placeholder="<empty>"
                                value={self.filter.clone()}
                                oninput={ctx.link().callback(Msg::Filter)}
                                onfocus={ctx.link().callback(|_| Msg::Focused)}
                                onblur={ctx.link().callback(|_| Msg::Blurred)}
                                ref={self.filter_ref.clone()}
                            />
                            { "Filter" }
                        </label>
                        <hr class="border-t border-default-border" />
                        <div class="p-px overflow-y-auto z-20"
                            onscroll={ctx.link().callback(|_| Msg::Scrolled)}
                            ref={self.scroll_ref.clone()}
                        >
                            <div style={format!("height: {}px;", item_db.len() * self.row_height as usize)}>
                                { for options }
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
