use gloo::timers::callback::Timeout;
use yew::{prelude::*, utils::NeqAssign};

use crate::gui::components::Helper;

pub enum Msg {
    Toggle,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: Option<String>,
    pub children: Children,
    #[prop_or(true)]
    pub opened: bool,
    pub helper: Option<&'static str>,
}

pub struct Table {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for Table {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Table { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                self.props.opened = !self.props.opened;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let Props { title, children, opened, helper } = &self.props;
        let opened = title.is_none() || *opened;

        let title = title.as_ref().map(|title| {
            let chevron = if opened { "table-chevron-down" } else { "table-chevron-right" };
            let helper = helper.as_ref().map(|&helper| {
                html! {
                    <Helper text={helper} />
                }
            });

            html! {
                <div class="flex-1 bg-table-odd p-px">
                    <button
                        class={classes![
                            "flex",
                            "items-center",
                            "gap-1",
                            "rounded-none",
                            "hover:bg-theme-hover",
                            "active:bg-theme-active",
                            "px-1",
                            "w-full",
                            "text-left",
                            "pl-6",
                            chevron,
                        ]}
                        onclick={self.link.callback(|_| Msg::Toggle)}
                    >
                        { title }
                        { for helper }
                    </button>
                </div>
            }
        });

        let chunks = opened.then(|| {
            let mut rows = children.iter().map(|child| {
                html_nested! {
                    <div class={classes![
                        "table-row",
                        title.is_some().then(|| "!pl-6"),
                    ]}>
                        {child}
                    </div>
                }
            });

            const CHUNK_SIZE: usize = 40;
            let chunks = (0..children.len() / CHUNK_SIZE + 1).map(|i| {
                html! {
                    <RowChunk position={i}>
                        { for rows.by_ref().take(CHUNK_SIZE) }
                    </RowChunk>
                }
            });

            html! { for chunks }
        });

        html! {
            <div class="flex flex-col border border-default-border">
                { for title }
                { for chunks }
            </div>
        }
    }
}

enum ChunkMsg {
    Render,
}

#[derive(Properties, Clone, PartialEq)]
struct ChunkProps {
    children: Children,
    position: usize,
}

struct RowChunk {
    props: ChunkProps,
    link: ComponentLink<Self>,
    should_render: bool,
}

impl Component for RowChunk {
    type Message = ChunkMsg;
    type Properties = ChunkProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let should_render = props.position == 0;
        if !should_render {
            let link = link.clone();
            Timeout::new(17 * props.position as u32, move || link.send_message(ChunkMsg::Render))
                .forget();
        }
        RowChunk { props, link, should_render }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ChunkMsg::Render => {
                self.should_render = true;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.neq_assign(props) {
            if self.props.position != 0 {
                let link = self.link.clone();
                Timeout::new(0, move || link.send_message(ChunkMsg::Render)).forget();
            } else {
                return true;
            }
        }
        false
    }

    fn view(&self) -> Html {
        let content = self.should_render.then(|| self.props.children.iter().collect::<Html>());
        html! { for content }
    }
}
