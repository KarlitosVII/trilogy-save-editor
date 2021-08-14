use std::rc::Rc;

use gloo::timers::future::TimeoutFuture;
use indexmap::IndexMap;
use web_sys::HtmlElement;
use yew::{prelude::*, utils::NeqAssign};

use crate::save_data::mass_effect_1_le::item_db::{DbItem, Me1ItemDb};

pub enum Msg {
    Scrolled,
    Open,
    ShouldClose,
    Focused,
    Blurred,
    BlurAll,
    Filter(String),
    Select(DbItem),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub item_db: Rc<Me1ItemDb>,
    pub current_item: DbItem,
    pub onselect: Callback<DbItem>,
}

pub struct ItemSelect {
    props: Props,
    link: ComponentLink<Self>,
    select_ref: NodeRef,
    drop_down_ref: NodeRef,
    scroll_ref: NodeRef,
    filter_ref: NodeRef,
    focused: bool,
    opened: bool,
    check_direction: bool,
    row_height: i32,
    skip: usize,
    take: usize,
    filter: String,
    filtered_list: Option<Me1ItemDb>,
}

impl Component for ItemSelect {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ItemSelect {
            props,
            link,
            select_ref: Default::default(),
            drop_down_ref: Default::default(),
            scroll_ref: Default::default(),
            filter_ref: Default::default(),
            focused: false,
            opened: false,
            check_direction: false,
            row_height: 20,
            skip: 0,
            take: 0,
            filter: Default::default(),
            filtered_list: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Scrolled => {
                if let Some(scroll) = self.scroll_ref.cast::<HtmlElement>() {
                    let scroll_top = scroll.scroll_top();
                    let offset_height = 300;
                    let num_rows = offset_height / self.row_height + 1;
                    let overflow_begin = num_rows / 4;
                    let overflow_end = num_rows / 2;

                    let len = self.filtered_list.as_ref().unwrap_or(&self.props.item_db).len();
                    let start = scroll_top / self.row_height;
                    self.skip = (start - overflow_begin).max(0) as usize;
                    self.take = (num_rows + overflow_end).min(len as i32) as usize;
                }
                true
            }
            Msg::Open => {
                self.opened = true;
                self.check_direction = true;
                self.link.send_message(Msg::Scrolled);
                false
            }
            Msg::ShouldClose => {
                if !self.focused {
                    self.opened = false;
                    if let Some(drop_down) = self.drop_down_ref.cast::<HtmlElement>() {
                        let _ = drop_down.style().set_property("bottom", "auto");
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
                self.link.send_future(async {
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
            Msg::Filter(filter) => {
                if !filter.is_empty() {
                    let filter = filter.to_lowercase();
                    let filtered_list = self
                        .props
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
                self.link.send_message(Msg::Scrolled);
                false
            }
            Msg::Select(key) => {
                self.props.current_item = key.clone();
                self.props.onselect.emit(key);
                self.link.send_message(Msg::BlurAll);
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.neq_assign(props) {
            if !self.opened {
                return true;
            }
            self.link.send_message(Msg::BlurAll);
        }
        false
    }

    fn rendered(&mut self, _first_render: bool) {
        if self.check_direction {
            self.check_direction = false;

            // Drop down open upward if bottom > viewport_height
            if let Some(drop_down) = self.drop_down_ref.cast::<HtmlElement>() {
                let viewport_height =
                    yew::utils::document().document_element().unwrap().client_height();
                let rect = drop_down.get_bounding_client_rect();
                let top = rect.top() as i32;
                let bottom = rect.bottom() as i32;
                let height = bottom - top;

                if height < top - 70 && bottom > viewport_height - 10 {
                    if let Some(select) = self.select_ref.cast::<HtmlElement>() {
                        let height = select.offset_height();
                        let _ = drop_down.style().set_property("bottom", &format!("{}px", height));
                    }
                }
            }
        }
    }

    fn view(&self) -> Html {
        let current_item_name = self
            .props
            .item_db
            .get(&self.props.current_item)
            .map(|i| i.as_str())
            .unwrap_or_else(|| "Unknown item");

        let options = self.opened.then(|| {
            let item_db = self.filtered_list.as_ref().unwrap_or(&self.props.item_db);
            let options = item_db.iter().skip(self.skip).take(self.take).map(|(&key, option)| {
                let selected = key == self.props.current_item;
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
                        onclick={self.link.callback(move |_| Msg::Select(key))}
                    >
                        { option }
                    </a>
                }
            });

            html! {
                <div style={format!("height: {}px;", item_db.len() * self.row_height as usize)}>
                    <div class="flex flex-col"
                        style={format!("will-change: transform; transform: translateY({}px)", self.skip * self.row_height as usize)}
                    >
                        { for options }
                    </div>
                </div>
            }
        });

        let onclick = if !self.opened {
            self.link.callback(|_| Msg::Open)
        } else {
            self.link.callback(|_| Msg::BlurAll)
        };

        html! {
            <div class="relative flex-auto select-none" tabindex="0"
                onfocus={self.link.callback(|_| Msg::Focused)}
                onblur={self.opened.then(||self.link.callback(|_| Msg::Blurred))}
                ref={self.select_ref.clone()}
            >
                <a class={classes![
                        "block",
                        "bg-theme-bg",
                        "hover:bg-theme-hover",
                        "active:bg-theme-active",
                        "px-1",
                        "pr-5",
                        "cursor-pointer",
                        "min-w-full",
                        "select-chevron",
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
                        "max-h-[300px]",
                        "z-20",
                        (!self.opened).then(|| "hidden")
                    ]}
                    ref={self.drop_down_ref.clone()}
                >
                    <label class="flex items-center gap-1 p-px pr-1">
                        <input type="text" class="flex-auto input" placeholder="<empty>"
                            value={self.filter.clone()}
                            oninput={self.link.callback(|data: InputData| Msg::Filter(data.value))}
                            onfocus={self.link.callback(|_| Msg::Focused)}
                            onblur={self.link.callback(|_| Msg::Blurred)}
                            ref={self.filter_ref.clone()}
                        />
                        { "Filter" }
                    </label>
                    <hr class="border-t border-default-border" />
                    <div class="p-px overflow-y-auto z-20"
                        onscroll={self.link.callback(|_| Msg::Scrolled)}
                        ref={self.scroll_ref.clone()}
                    >
                        { for options }
                    </div>
                </div>
            </div>
        }
    }
}
