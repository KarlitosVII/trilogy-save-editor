use web_sys::HtmlElement;
use yew::prelude::*;

pub enum Msg {
    Open,
    Close,
    Blur,
    Select(usize),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub options: &'static [&'static str],
    pub current_idx: usize,
    pub onselect: Callback<usize>,
    #[prop_or(true)]
    pub sized: bool,
}

pub struct Select {
    select_ref: NodeRef,
    drop_down_ref: NodeRef,
    current_idx: usize,
    opened: bool,
}

impl Component for Select {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Select {
            select_ref: Default::default(),
            drop_down_ref: Default::default(),
            current_idx: ctx.props().current_idx,
            opened: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Open => {
                self.opened = true;
                true
            }
            Msg::Close => {
                self.opened = false;
                true
            }
            Msg::Blur => {
                if let Some(select) = self.select_ref.cast::<HtmlElement>() {
                    let _ = select.blur();
                }
                false
            }
            Msg::Select(idx) => {
                self.current_idx = idx;
                ctx.props().onselect.emit(idx);
                ctx.link().send_message(Msg::Blur);
                false
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.current_idx = ctx.props().current_idx;
        if self.opened {
            ctx.link().send_message(Msg::Blur);
            false
        } else {
            true
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
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
            } else {
                let _ = drop_down.style().set_property("bottom", "auto");
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let drop_down = self.opened.then(|| {
            let options = ctx.props().options.iter().enumerate().map(|(idx, option)| {
                let selected = idx == self.current_idx;
                html! {
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
                        onclick={ctx.link().callback(move |_| Msg::Select(idx))}
                    >
                        { option }
                    </a>
                }
            });
            html! { for options }
        });

        let size = if ctx.props().sized { "w-[200px]" } else { "min-w-[60px]" };

        let onclick = if !self.opened {
            ctx.link().callback(|_| Msg::Open)
        } else {
            ctx.link().callback(|_| Msg::Blur)
        };

        html! {
            <div tabindex="0"
                class={classes![
                    "relative",
                    "select-none",
                    size,
                ]}
                onblur={self.opened.then(||ctx.link().callback(|_| Msg::Close))}
                ref={self.select_ref.clone()}
            >
                <a class={classes![
                        "block",
                        "bg-theme-bg",
                        "hover:bg-theme-hover",
                        "active:bg-theme-active",
                        "px-1",
                        "cursor-pointer",
                        "min-w-full",
                        "select-chevron",
                    ]}
                    {onclick}
                >
                    { ctx.props().options[self.current_idx] }
                </a>
                <div
                    class={classes![
                        "absolute",
                        "flex",
                        "flex-col",
                        "bg-popup/95",
                        "border",
                        "border-default-border",
                        "p-px",
                        "min-w-full",
                        "z-20",
                        (!self.opened).then(|| "hidden" ),
                    ]}
                    ref={self.drop_down_ref.clone()}
                >
                    { for drop_down }
                </div>
            </div>
        }
    }
}
