use std::{
    cell::{Ref, RefMut},
    rc::Rc,
};

use gloo::{events::EventListener, timers::future::TimeoutFuture, utils};
use indexmap::{map::Entry, IndexMap};
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

use crate::{
    gui::{
        components::{CheckBox, Helper, InputNumber, NumberType},
        raw_ui::RawUi,
    },
    save_data::{shared::plot::RawPlotDb, RcRef},
};

use super::{FloatPlotType, IntPlotType, PlotType};

const LABEL_LIST_MAX_LEN: usize = 10_000_000;

pub enum Msg {
    Scrolled,
    ChangeBool(usize, bool),
    Filter(InputEvent),
    Filtered,
    Add,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub plots: PlotType,
    pub plot_db: Rc<RawPlotDb>,
    #[prop_or_default]
    filter: RcRef<String>,
    #[prop_or_default]
    add_id: RcRef<i32>,
}

impl Props {
    fn filter(&self) -> Ref<'_, String> {
        self.filter.borrow()
    }

    fn filter_mut(&self) -> RefMut<'_, String> {
        self.filter.borrow_mut()
    }

    fn add_id(&self) -> Ref<'_, i32> {
        self.add_id.borrow()
    }
}

pub struct RawPlot {
    _resize_listener: EventListener,
    scroll_ref: NodeRef,
    row_height: i32,
    skip: usize,
    take: usize,
    label_list: Option<IndexMap<usize, Option<String>>>,
    is_filtering: bool,
    pending_filter: Option<InputEvent>,
}

impl Component for RawPlot {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let _resize_listener = {
            let link = ctx.link().clone();
            EventListener::new(&utils::window(), "resize", move |_| {
                link.send_message(Msg::Scrolled)
            })
        };
        ctx.link().send_message(Msg::Scrolled);

        let mut this = RawPlot {
            _resize_listener,
            scroll_ref: Default::default(),
            row_height: 23,
            skip: 0,
            take: 0,
            label_list: None,
            is_filtering: false,
            pending_filter: None,
        };
        this.add_missing_plots(ctx);
        this.update_label_list(ctx);
        this
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Scrolled => {
                if let Some(scroll) = self.scroll_ref.cast::<HtmlElement>() {
                    let scroll_top = scroll.scroll_top();
                    let offset_height = scroll.offset_height();
                    let num_rows = offset_height / self.row_height + 2;

                    let len = self.label_list.as_ref().map(|list| list.len()).unwrap_or_default();
                    let start = scroll_top / self.row_height;
                    self.skip = start.max(0) as usize;
                    self.take = num_rows.min(len as i32) as usize;
                }
                true
            }
            Msg::ChangeBool(idx, value) => {
                if let PlotType::Boolean(ref booleans) = ctx.props().plots {
                    if let Some(mut plot) = booleans.borrow_mut().get_mut(idx) {
                        *plot = value;
                    }
                }
                false
            }
            Msg::Filter(event) => {
                if !self.is_filtering {
                    if let Some(input) = event.target_dyn_into::<HtmlInputElement>() {
                        self.is_filtering = true;

                        *ctx.props().filter_mut() = input.value();

                        ctx.link().send_future(async {
                            TimeoutFuture::new(100).await;
                            Msg::Filtered
                        });

                        self.update_label_list(ctx);

                        ctx.link().send_message(Msg::Scrolled);
                    }
                } else {
                    self.pending_filter = Some(event);
                }
                false
            }
            Msg::Filtered => {
                self.is_filtering = false;
                if let Some(event) = self.pending_filter.take() {
                    ctx.link().send_message(Msg::Filter(event))
                }
                false
            }
            Msg::Add => {
                let new_plot = *ctx.props().add_id() as usize;
                let added = match ctx.props().plots {
                    PlotType::Boolean(ref booleans) => {
                        let mut booleans = booleans.borrow_mut();
                        if new_plot >= booleans.len() {
                            booleans.resize_with(new_plot + 1, Default::default);
                            true
                        } else {
                            false
                        }
                    }
                    PlotType::Int(ref integers) => match integers {
                        IntPlotType::Vec(ref vec) => {
                            let mut vec = vec.borrow_mut();
                            if new_plot >= vec.len() {
                                vec.resize_with(new_plot + 1, Default::default);
                                true
                            } else {
                                false
                            }
                        }
                        IntPlotType::IndexMap(ref index_map) => {
                            match index_map.borrow_mut().entry(new_plot as i32) {
                                Entry::Vacant(plot) => {
                                    plot.insert(Default::default());
                                    true
                                }
                                Entry::Occupied(_) => false,
                            }
                        }
                    },
                    PlotType::Float(ref floats) => match floats {
                        FloatPlotType::Vec(ref vec) => {
                            let mut vec = vec.borrow_mut();
                            if new_plot >= vec.len() {
                                vec.resize_with(new_plot + 1, Default::default);
                                true
                            } else {
                                false
                            }
                        }
                        FloatPlotType::IndexMap(ref index_map) => {
                            match index_map.borrow_mut().entry(new_plot as i32) {
                                Entry::Vacant(plot) => {
                                    plot.insert(Default::default());
                                    true
                                }
                                Entry::Occupied(_) => false,
                            }
                        }
                    },
                };
                if added {
                    self.update_label_list(ctx);
                    ctx.link().send_message(Msg::Scrolled);
                }
                false
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.add_missing_plots(ctx);
        self.update_label_list(ctx);

        if let Some(scroll) = self.scroll_ref.cast::<HtmlElement>() {
            if scroll.scroll_top() != 0 {
                scroll.set_scroll_top(0);
            } else {
                ctx.link().send_message(Msg::Scrolled);
            }
            return false;
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let label_list = self.label_list.as_ref();
        let rows =
            label_list.unwrap().iter().skip(self.skip).take(self.take).map(|(&idx, label)| {
                let label = label
                    .as_ref()
                    .map(|label| format!("{} - {}", idx, label))
                    .unwrap_or_else(|| idx.to_string());

                let row = match ctx.props().plots {
                    PlotType::Boolean(ref booleans) => booleans.borrow().get(idx).map(|plot| {
                        html! {
                            <CheckBox
                                {label}
                                value={RcRef::new(*plot)}
                                onchange={ctx.link().callback(move |value| Msg::ChangeBool(idx, value))}
                            />
                        }
                    }),
                    PlotType::Int(ref integers) => match integers {
                        IntPlotType::Vec(ref vec) => {
                            vec.borrow().get(idx).map(|plot| plot.view(&label))
                        }
                        IntPlotType::IndexMap(ref index_map) => {
                            index_map.borrow().get(&(idx as i32)).map(|plot| plot.view(&label))
                        }
                    },
                    PlotType::Float(ref floats) => match floats {
                        FloatPlotType::Vec(ref vec) => {
                            vec.borrow().get(idx).map(|plot| plot.view(&label))
                        }
                        FloatPlotType::IndexMap(ref index_map) => {
                            index_map.borrow().get(&(idx as i32)).map(|plot| plot.view(&label))
                        }
                    },
                };
                html! {
                    <div class="raw-plot-row">
                        { for row }
                    </div>
                }
            });

        let add_helper = match ctx.props().plots {
            PlotType::Boolean(_)
            | PlotType::Int(IntPlotType::Vec(_))
            | PlotType::Float(FloatPlotType::Vec(_)) => html! {
                <Helper text=
                    "Be careful when adding a new plot.\n\
                    Because of the way the data is stored, adding a new plot will add as many plots as the plot id.\n\
                    For example, if you have `10 000` plots, adding the plot `1 000 000` will add `990 000` plots !\n
                    This can bloat your save and significantly reduce the performance of the filter."
                />
            },
            _ => Html::default(),
        };
        let len = label_list.map(|list| list.len()).unwrap_or_default();
        html! {
            <div class="flex-auto flex flex-col gap-1">
                <div class="flex gap-3 w-2/3">
                    <label class="flex-auto flex items-center gap-1">
                        <input type="text" class="flex-auto input" placeholder="<empty>" value={ctx.props().filter().clone()}
                            oninput={ctx.link().callback(Msg::Filter)}
                        />
                        { "Filter" }
                    </label>
                    <form class="flex gap-1"
                        onsubmit={ctx.link().callback(|e: FocusEvent| {
                            e.prevent_default();
                            Msg::Add
                        })}
                    >
                        <InputNumber label={String::default()} value={NumberType::Int(RcRef::clone(&ctx.props().add_id))} />
                        <input type="submit" class="button" value="Add" />
                        { add_helper }
                    </form>
                </div>
                <hr class="border-t border-default-border" />
                <div class="flex-auto h-0 overflow-y-auto"
                    onscroll={ctx.link().callback(|_| {gloo::console::log!("Scrolled"); Msg::Scrolled})}
                    ref={self.scroll_ref.clone()}
                >
                    <div class="relative w-full border border-default-border raw-plot-bg"
                        style={format!("height: {}px;", len * self.row_height as usize + 2)}
                    >
                        <div class="absolute min-w-[33.333333%]"
                            style={format!("will-change: transform; transform: translate3d(0, {}px, 0)", self.skip * self.row_height as usize)}
                        >
                            { for rows }
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

impl RawPlot {
    fn add_missing_plots(&mut self, ctx: &Context<Self>) {
        let Props { plots, plot_db, .. } = &mut ctx.props();

        match plots {
            PlotType::Boolean(ref booleans) => {
                if let Some(&max) = plot_db.booleans.keys().max() {
                    let mut booleans = booleans.borrow_mut();
                    if max >= booleans.len() {
                        booleans.resize_with(max + 1, Default::default);
                    };
                }
            }
            PlotType::Int(ref integers) => match integers {
                IntPlotType::Vec(ref vec) => {
                    if let Some(&max) = plot_db.integers.keys().max() {
                        let mut vec = vec.borrow_mut();
                        if max >= vec.len() {
                            vec.resize_with(max + 1, Default::default);
                        };
                    }
                }
                IntPlotType::IndexMap(ref index_map) => {
                    for key in plot_db.integers.keys().copied() {
                        index_map.borrow_mut().entry(key as i32).or_default();
                    }
                }
            },
            PlotType::Float(ref floats) => match floats {
                FloatPlotType::Vec(ref vec) => {
                    if let Some(&max) = plot_db.floats.keys().max() {
                        let mut vec = vec.borrow_mut();
                        if max >= vec.len() {
                            vec.resize_with(max + 1, Default::default);
                        };
                    }
                }
                FloatPlotType::IndexMap(ref index_map) => {
                    for key in plot_db.floats.keys().copied() {
                        index_map.borrow_mut().entry(key as i32).or_default();
                    }
                }
            },
        }
    }

    fn update_label_list(&mut self, ctx: &Context<Self>) {
        let Props { plots, plot_db, .. } = &ctx.props();

        let mut label_list: IndexMap<usize, Option<String>> = match plots {
            PlotType::Boolean(ref bitvec) => {
                let len = bitvec.borrow().len().min(LABEL_LIST_MAX_LEN);
                let label_list = plot_db.booleans.iter().map(|(&k, v)| (k, Some(v.clone())));
                (0..len).map(|idx| (idx, None)).chain(label_list).collect()
            }
            PlotType::Int(ref integers) => {
                let label_list = plot_db.integers.iter().map(|(&k, v)| (k, Some(v.clone())));
                match integers {
                    IntPlotType::Vec(ref vec) => {
                        let len = vec.borrow().len().min(LABEL_LIST_MAX_LEN);
                        (0..len).map(|idx| (idx, None)).chain(label_list).collect()
                    }
                    IntPlotType::IndexMap(ref index_map) => index_map
                        .borrow()
                        .keys()
                        .map(|&idx| (idx as usize, None))
                        .chain(label_list)
                        .collect(),
                }
            }
            PlotType::Float(ref floats) => {
                let label_list = plot_db.floats.iter().map(|(&k, v)| (k, Some(v.clone())));
                match floats {
                    FloatPlotType::Vec(ref vec) => {
                        let len = vec.borrow().len().min(LABEL_LIST_MAX_LEN);
                        (0..len).map(|idx| (idx, None)).chain(label_list).collect()
                    }
                    FloatPlotType::IndexMap(ref index_map) => index_map
                        .borrow()
                        .keys()
                        .map(|&idx| (idx as usize, None))
                        .chain(label_list)
                        .collect(),
                }
            }
        };

        label_list.sort_keys();

        let filter = ctx.props().filter();
        let is_number = filter.parse::<usize>().is_ok();
        if !filter.is_empty() {
            let filter_lowercase = filter.to_lowercase();
            label_list.retain(|idx, label| {
                label
                    .as_ref()
                    .map(|label| label.to_lowercase().contains(&filter_lowercase))
                    .unwrap_or_else(|| false)
                    || (is_number && idx.to_string().contains(&*filter))
            });
        }

        self.label_list = Some(label_list);
    }
}
