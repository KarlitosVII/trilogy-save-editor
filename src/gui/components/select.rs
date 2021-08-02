use web_sys::HtmlElement;
use yew::prelude::*;
use yewtil::NeqAssign;

pub enum Msg {
    Open,
    Close,
    Blur,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub options: &'static [&'static str],
    pub current_idx: usize,
    pub onselect: Callback<usize>,
}

pub struct Select {
    props: Props,
    link: ComponentLink<Self>,
    select_ref: NodeRef,
    opened: bool,
}

impl Component for Select {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Select { props, link, select_ref: Default::default(), opened: false }
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
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.neq_assign(props) {
            if let Some(select) = self.select_ref.cast::<HtmlElement>() {
                let _ = select.blur();
            }
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let options = self.props.options.iter().enumerate().map(|(idx, option)| {
            html_nested! {
                <a
                    class=classes![
                        "flex-1",
                        "px-1",
                        "hover:bg-theme-hover",
                        "active:bg-theme-active",
                        "cursor-pointer",
                        "whitespace-nowrap",
                        (idx == self.props.current_idx).then(|| "bg-theme-bg"),
                    ]
                    onclick=self.props.onselect.reform(move |_| idx)
                >
                    { option }
                </a>
            }
        });

        let onclick = if !self.opened {
            self.link.callback(|_| Msg::Open)
        } else {
            self.link.callback(|_| Msg::Blur)
        };

        html! {
            <div class="relative w-[200px] select-none" tabindex="0"
                onblur=self.opened.then(||self.link.callback(|_| Msg::Close))
                ref=self.select_ref.clone()
            >
                <a class="block bg-theme-bg hover:bg-theme-hover active:bg-theme-active px-1 cursor-pointer min-w-full relative select-chevron"
                    onclick=onclick
                >
                    { self.props.options[self.props.current_idx] }
                </a>
                <div
                    class=classes![
                        "absolute",
                        "left-0",
                        "flex",
                        "flex-col",
                        "p-px",
                        "bg-popup/90",
                        "border",
                        "border-default-border",
                        "min-w-full",
                        "z-40",
                        (!self.opened).then(|| "hidden" ),
                    ]
                >
                    { for options }
                </div>
            </div>
        }
    }
}
