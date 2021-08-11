use web_sys::HtmlElement;
use yew::{prelude::*, utils::NeqAssign};

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
    props: Props,
    link: ComponentLink<Self>,
    select_ref: NodeRef,
    drop_down_ref: NodeRef,
    opened: bool,
}

impl Component for Select {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Select {
            props,
            link,
            select_ref: Default::default(),
            drop_down_ref: Default::default(),
            opened: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
                self.props.current_idx = idx;
                self.props.onselect.emit(idx);
                self.link.send_message(Msg::Blur);
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.neq_assign(props) {
            if !self.opened {
                return true;
            }
            self.link.send_message(Msg::Blur);
        }
        false
    }

    fn rendered(&mut self, _first_render: bool) {
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

    fn view(&self) -> Html {
        let options = self.props.options.iter().enumerate().map(|(idx, option)| {
            let selected = idx == self.props.current_idx;
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
                    onclick={self.link.callback(move |_| Msg::Select(idx))}
                >
                    { option }
                </a>
            }
        });

        let size = if self.props.sized { "w-[200px]" } else { "min-w-[60px]" };

        let onclick = if !self.opened {
            self.link.callback(|_| Msg::Open)
        } else {
            self.link.callback(|_| Msg::Blur)
        };

        html! {
            <div tabindex="0"
                class={classes![
                    "relative",
                    "select-none",
                    size,
                ]}
                onblur={self.opened.then(||self.link.callback(|_| Msg::Close))}
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
                    { self.props.options[self.props.current_idx] }
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
                    { for options }
                </div>
            </div>
        }
    }
}
