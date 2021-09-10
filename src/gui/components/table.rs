use gloo::timers::future::TimeoutFuture;
use yew::prelude::*;

use crate::gui::components::Helper;

pub enum Msg {
    Toggle,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: Option<String>,
    pub children: Children,
    #[prop_or(true)]
    pub opened: bool,
    pub helper: Option<&'static str>,
}

pub struct Table {
    opened: bool,
}

impl Component for Table {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Table { opened: ctx.props().opened }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle => {
                self.opened = !self.opened;
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.opened = ctx.props().opened;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { title, children, helper, .. } = &ctx.props();
        let opened = title.is_none() || self.opened;

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
                        onclick={ctx.link().callback(|_| Msg::Toggle)}
                    >
                        { title }
                        { for helper }
                    </button>
                </div>
            }
        });

        let chunks = opened.then(|| {
            let mut rows = children.iter().map(|child| {
                html! {
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

#[derive(Properties, PartialEq)]
struct ChunkProps {
    children: Children,
    position: usize,
}

struct RowChunk {
    should_render: bool,
}

impl Component for RowChunk {
    type Message = ChunkMsg;
    type Properties = ChunkProps;

    fn create(ctx: &Context<Self>) -> Self {
        let should_render = ctx.props().position == 0;
        if !should_render {
            let position = ctx.props().position as u32;
            ctx.link().send_future(async move {
                TimeoutFuture::new(17 * position).await;
                ChunkMsg::Render
            });
        }
        RowChunk { should_render }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ChunkMsg::Render => {
                self.should_render = true;
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if ctx.props().position == 0 {
            true
        } else {
            ctx.link().send_future(async {
                TimeoutFuture::new(0).await;
                ChunkMsg::Render
            });
            false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let content = self.should_render.then(|| ctx.props().children.iter().collect::<Html>());
        html! { for content }
    }
}
