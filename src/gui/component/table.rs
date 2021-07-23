use yew::prelude::*;

pub enum Msg {
    Toggle,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or(true)]
    pub opened: bool,
    pub children: Children,
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

    fn change(&mut self, mut props: Self::Properties) -> ShouldRender {
        let Props { title, opened, children } = &mut props;
        if self.props.title != *title || self.props.children != *children {
            *opened = self.props.opened;
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let opened = self.props.title.is_none() || self.props.opened;

        let title = self
            .props
            .title
            .as_ref()
            .map(|title| {
                let chevron = if opened {
                    // Down ▼
                    "table-chevron-down"
                } else {
                    // Right ▶
                    "table-chevron-right"
                };

                html! {
                    <div class="flex-1 bg-table-odd p-px">
                        <button
                            class=classes![
                                "rounded-none",
                                "hover:bg-theme-hover",
                                "active:bg-theme-active",
                                "px-1",
                                "w-full",
                                "text-left",
                                "pl-6",
                                chevron,
                            ]
                            onclick=self.link.callback(|_| Msg::Toggle)
                        >
                            {title}
                        </button>
                    </div>
                }
            })
            .unwrap_or_default();

        let rows = opened
            .then(|| {
                self.props
                    .children
                    .iter()
                    .map(|child| {
                        html_nested! {
                            <div class=classes![
                                "table-row",
                                self.props.title.is_some().then(|| "!pl-6"),
                            ]>
                                {child}
                            </div>
                        }
                    })
                    .collect::<Html>()
            })
            .unwrap_or_default();

        html! {
            <div class="flex flex-col border border-default-border">
                {title}
                {rows}
            </div>
        }
    }
}
