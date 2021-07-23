use std::time::Duration;
use web_sys::HtmlElement;
use yew::{
    prelude::*,
    services::{resize::ResizeTask, timeout::TimeoutTask, ResizeService, TimeoutService},
};
use yewtil::NeqAssign;

pub enum Msg {
    Throttle,
    Scrolled,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub list: Vec<i32>,
}

pub struct Clipper {
    props: Props,
    link: ComponentLink<Self>,
    _resize_task: ResizeTask,
    scroll_ref: NodeRef,
    content_ref: NodeRef,
    throttle: Option<TimeoutTask>,
    queued_scroll: bool,
    row_height: i32,
    start: usize,
    end: usize,
}

impl Component for Clipper {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let _resize_task = ResizeService::register(link.callback(|_| Msg::Scrolled));
        link.send_message(Msg::Scrolled);
        Clipper {
            props,
            link,
            _resize_task,
            scroll_ref: Default::default(),
            content_ref: Default::default(),
            throttle: None,
            queued_scroll: false,
            row_height: 23,
            start: 0,
            end: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        const TRHOTTLE: Duration = Duration::from_millis(15);
        match msg {
            Msg::Scrolled => {
                if self.throttle.is_none() {
                    if let Some(scroll) = self.scroll_ref.cast::<HtmlElement>() {
                        let scroll_top = scroll.scroll_top();
                        let offset_height = scroll.offset_height();
                        let num_rows = offset_height / self.row_height + 1;
                        let overflow = num_rows / 4;

                        let len = self.props.list.len() as i32;
                        let start = scroll_top / self.row_height;
                        let end = start + num_rows;
                        self.start = (start - overflow).max(0) as usize;
                        self.end = (end + overflow).min(len) as usize;

                        self.throttle = Some(TimeoutService::spawn(
                            TRHOTTLE,
                            self.link.callback(|_| Msg::Throttle),
                        ));

                        return true;
                    }
                } else {
                    self.queued_scroll = true;
                }
                false
            }
            Msg::Throttle => {
                self.throttle = None;
                if self.queued_scroll {
                    self.queued_scroll = false;
                    self.link.send_message(Msg::Scrolled);
                }
                false
            }
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        if let Some(content) = self.content_ref.cast::<HtmlElement>() {
            let _ = content
                .style()
                .set_property("top", &format!("{}px", self.start * self.row_height as usize));
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let rows = self.props.list[self.start..self.end].iter().map(|child| {
            html_nested! {
                <div class="clipper-row">
                    <input class="input" type="number" value=child.to_string() />
                </div>
            }
        });

        html! {
            <div class="flex-auto h-0 overflow-y-auto"
                onscroll=self.link.callback(|_| Msg::Scrolled)
                ref=self.scroll_ref.clone()
            >
                <div class="relative w-full border border-default-border clipper-bg"
                    style=format!("height: {}px;", self.props.list.len() as i32 * self.row_height + 2)
                >
                    <div class="absolute w-full" ref=self.content_ref.clone()>
                        {for rows}
                    </div>
                </div>
            </div>
        }
    }
}
