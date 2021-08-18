use std::cell::{Ref, RefMut};
use std::rc::Rc;

use gloo::{events::EventListener, timers::future::TimeoutFuture};
use indexmap::{map::Entry, IndexMap};
use web_sys::{HtmlElement, HtmlInputElement};
use yew::{prelude::*, utils::NeqAssign};

use super::{FloatPlotType, IntPlotType, PlotType};
use crate::gui::{
    components::{CheckBox, Helper, InputNumber, NumberType},
    raw_ui::RawUi,
    RcUi,
};
use crate::save_data::shared::plot::RawPlotDb;

pub enum Msg {
    Scrolled,
    ChangeBool(usize, bool),
    Filter(InputEvent),
    Filtered,
    Add,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub plots: PlotType,
    pub plot_db: Rc<RawPlotDb>,
    #[prop_or_default]
    filter: RcUi<String>,
    #[prop_or_default]
    add_id: RcUi<i32>,
}

impl Props {
    fn filter(&self) -> Ref<'_, String> {
        self.filter.borrow()
    }

    fn filter_mut(&mut self) -> RefMut<'_, String> {
        self.filter.borrow_mut()
    }

    fn add_id(&self) -> Ref<'_, i32> {
        self.add_id.borrow()
    }
}

pub struct RawPlot {
    props: Props,
    link: ComponentLink<Self>,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let _resize_listener = {
            let link = link.clone();
            EventListener::new(&web_sys::window().unwrap(), "resize", move |_| {
                link.send_message(Msg::Scrolled)
            })
        };
        link.send_message(Msg::Scrolled);

        let mut this = RawPlot {
            props,
            link,
            _resize_listener,
            scroll_ref: Default::default(),
            row_height: 23,
            skip: 0,
            take: 0,
            label_list: None,
            is_filtering: false,
            pending_filter: None,
        };
        this.add_missing_plots();
        this.update_label_list();
        this
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
                if let PlotType::Boolean(ref mut booleans) = self.props.plots {
                    if let Some(mut plot) = booleans.borrow_mut().get_mut(idx) {
                        *plot = value;
                    }
                }
                false
            }
            Msg::Filter(event) => {
                if !self.is_filtering {
                    self.is_filtering = true;

                    let input: HtmlInputElement = event.target_unchecked_into();
                    *self.props.filter_mut() = input.value();

                    self.link.send_future(async {
                        TimeoutFuture::new(100).await;
                        Msg::Filtered
                    });

                    self.update_label_list();

                    self.link.send_message(Msg::Scrolled);
                } else {
                    self.pending_filter = Some(event);
                }
                false
            }
            Msg::Filtered => {
                self.is_filtering = false;
                if let Some(event) = self.pending_filter.take() {
                    self.link.send_message(Msg::Filter(event))
                }
                false
            }
            Msg::Add => {
                let new_plot = *self.props.add_id() as usize;
                let added = match self.props.plots {
                    PlotType::Boolean(ref mut booleans) => {
                        let mut booleans = booleans.borrow_mut();
                        if new_plot >= booleans.len() {
                            booleans.resize_with(new_plot + 1, Default::default);
                            true
                        } else {
                            false
                        }
                    }
                    PlotType::Integer(ref mut integers) => match integers {
                        IntPlotType::Vec(ref mut vec) => {
                            let mut vec = vec.borrow_mut();
                            if new_plot >= vec.len() {
                                vec.resize_with(new_plot + 1, Default::default);
                                true
                            } else {
                                false
                            }
                        }
                        IntPlotType::IndexMap(ref mut index_map) => {
                            match index_map.borrow_mut().entry(new_plot as i32) {
                                Entry::Vacant(plot) => {
                                    plot.insert(Default::default());
                                    true
                                }
                                Entry::Occupied(_) => false,
                            }
                        }
                    },
                    PlotType::Float(ref mut floats) => match floats {
                        FloatPlotType::Vec(ref mut vec) => {
                            let mut vec = vec.borrow_mut();
                            if new_plot >= vec.len() {
                                vec.resize_with(new_plot + 1, Default::default);
                                true
                            } else {
                                false
                            }
                        }
                        FloatPlotType::IndexMap(ref mut index_map) => {
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
                    self.update_label_list();
                    self.link.send_message(Msg::Scrolled);
                }
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.neq_assign(props) {
            self.add_missing_plots();
            self.update_label_list();

            if let Some(scroll) = self.scroll_ref.cast::<HtmlElement>() {
                if scroll.scroll_top() != 0 {
                    scroll.set_scroll_top(0);
                } else {
                    self.link.send_message(Msg::Scrolled);
                }
                return false;
            }
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let label_list = self.label_list.as_ref();
        let rows =
            label_list.unwrap().iter().skip(self.skip).take(self.take).map(|(&idx, label)| {
                let label = label
                    .as_ref()
                    .map(|label| format!("{} - {}", idx, label))
                    .unwrap_or_else(|| idx.to_string());

                let row = match self.props.plots {
                    PlotType::Boolean(ref booleans) => booleans.borrow().get(idx).map(|plot| {
                        html! {
                            <CheckBox
                                {label}
                                value={RcUi::new(*plot)}
                                onchange={self.link.callback(move |value| Msg::ChangeBool(idx, value))}
                            />
                        }
                    }),
                    PlotType::Integer(ref integers) => match integers {
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
                html_nested! {
                    <div class="raw-plot-row">
                        { for row }
                    </div>
                }
            });

        let add_helper = match self.props.plots {
            PlotType::Boolean(_)
            | PlotType::Integer(IntPlotType::Vec(_))
            | PlotType::Float(FloatPlotType::Vec(_)) => html! {
                <Helper text=
                    "Be careful when adding a new plot.\n\
                    Because of the way the data is stored, adding a new plot will add as many plots as the plot id.\n\
                    For example, if you have `40 000` plots, adding the plot `1 000 000` will add `960 000` plots !\n
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
                        <input type="text" class="flex-auto input" placeholder="<empty>" value={self.props.filter().clone()}
                            oninput={self.link.callback(Msg::Filter)}
                        />
                        { "Filter" }
                    </label>
                    <form class="flex gap-1"
                        onsubmit={self.link.callback(|e: FocusEvent| {
                            e.prevent_default();
                            Msg::Add
                        })}
                    >
                        <InputNumber label={String::default()} value={NumberType::Integer(RcUi::clone(&self.props.add_id))} />
                        <input type="submit" class="button" value="Add" />
                        { add_helper }
                    </form>
                </div>
                <hr class="border-t border-default-border" />
                <div class="flex-auto h-0 overflow-y-auto"
                    onscroll={self.link.callback(|_| Msg::Scrolled)}
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
    fn add_missing_plots(&mut self) {
        let Props { plots, plot_db, .. } = &mut self.props;

        match plots {
            PlotType::Boolean(ref mut booleans) => {
                if let Some(&max) = plot_db.booleans.keys().max() {
                    let mut booleans = booleans.borrow_mut();
                    if max >= booleans.len() {
                        booleans.resize_with(max + 1, Default::default);
                    };
                }
            }
            PlotType::Integer(ref mut integers) => match integers {
                IntPlotType::Vec(ref mut vec) => {
                    if let Some(&max) = plot_db.integers.keys().max() {
                        let mut vec = vec.borrow_mut();
                        if max >= vec.len() {
                            vec.resize_with(max + 1, Default::default);
                        };
                    }
                }
                IntPlotType::IndexMap(ref mut index_map) => {
                    for key in plot_db.integers.keys().copied() {
                        index_map.borrow_mut().entry(key as i32).or_default();
                    }
                }
            },
            PlotType::Float(ref mut floats) => match floats {
                FloatPlotType::Vec(ref mut vec) => {
                    if let Some(&max) = plot_db.floats.keys().max() {
                        let mut vec = vec.borrow_mut();
                        if max >= vec.len() {
                            vec.resize_with(max + 1, Default::default);
                        };
                    }
                }
                FloatPlotType::IndexMap(ref mut index_map) => {
                    for key in plot_db.floats.keys().copied() {
                        index_map.borrow_mut().entry(key as i32).or_default();
                    }
                }
            },
        }
    }

    fn update_label_list(&mut self) {
        let Props { plots, plot_db, .. } = &self.props;

        let mut label_list: IndexMap<usize, Option<String>> = match plots {
            PlotType::Boolean(ref bitvec) => {
                let label_list = plot_db.booleans.iter().map(|(&k, v)| (k, Some(v.clone())));
                (0..bitvec.borrow().len()).map(|idx| (idx, None)).chain(label_list).collect()
            }
            PlotType::Integer(ref integers) => {
                let label_list = plot_db.integers.iter().map(|(&k, v)| (k, Some(v.clone())));
                match integers {
                    IntPlotType::Vec(ref vec) => {
                        (0..vec.borrow().len()).map(|idx| (idx, None)).chain(label_list).collect()
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
                        (0..vec.borrow().len()).map(|idx| (idx, None)).chain(label_list).collect()
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

        let filter = self.props.filter();
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
