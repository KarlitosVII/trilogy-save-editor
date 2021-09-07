use std::cell::RefMut;

use yew::prelude::*;

use super::IntPlotType;
use crate::gui::{
    components::{CheckBox, Table},
    raw_ui::RawUi,
    RcUi,
};
use crate::save_data::shared::plot::{BitVec, PlotCategory as PlotCategoryDb};

pub enum Msg {
    ChangeBool(usize, bool),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: Option<String>,
    pub booleans: RcUi<BitVec>,
    pub integers: IntPlotType,
    pub category: PlotCategoryDb,
    #[prop_or(false)]
    pub me3_imported_me1: bool,
}

impl Props {
    fn booleans_mut(&self) -> RefMut<'_, BitVec> {
        self.booleans.borrow_mut()
    }
}

pub struct PlotCategory;

impl Component for PlotCategory {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut this = PlotCategory {};
        this.add_missing_plots(ctx);
        this
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeBool(idx, value) => {
                if let Some(mut plot) = ctx.props().booleans_mut().get_mut(idx) {
                    *plot = value;
                }
                false
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.add_missing_plots(ctx);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { title, booleans, integers, category, me3_imported_me1 } = &ctx.props();
        let PlotCategoryDb { booleans: bool_db, integers: int_db } = category;

        let booleans = bool_db.iter().map(|(idx, label)| {
            let mut idx = *idx;
            if *me3_imported_me1 {
                idx += 10_000;
            }
            match booleans.borrow().get(idx) {
                Some(value) => html! {
                    <CheckBox
                        label={label.clone()}
                        value={RcUi::new(*value)}
                        onchange={ctx.link().callback(move |value| Msg::ChangeBool(idx, value))}
                    />
                },
                None => Html::default(),
            }
        });

        let integers = int_db.iter().map(|(idx, label)| {
            let mut idx = *idx;
            if *me3_imported_me1 {
                idx += 10_000;
            }
            let value = match integers {
                IntPlotType::Vec(vec) => vec.borrow().get(idx).map(RcUi::clone),
                IntPlotType::IndexMap(index_map) => {
                    index_map.borrow().get(&(idx as i32)).map(RcUi::clone)
                }
            };
            match value {
                Some(value) => value.view(label),
                None => Html::default(),
            }
        });

        html! {
            <Table title={title.clone()} opened={title.is_none()}>
                { for booleans }
                { for integers }
            </Table>
        }
    }
}

impl PlotCategory {
    fn add_missing_plots(&mut self, ctx: &Context<Self>) {
        let Props { booleans, integers, category, me3_imported_me1, .. } = &mut ctx.props();
        let PlotCategoryDb { booleans: bool_db, integers: int_db } = &category;

        // Booleans
        if let Some(&(mut max)) = bool_db.keys().max() {
            if *me3_imported_me1 {
                max += 10_000;
            }

            let mut booleans = booleans.borrow_mut();
            if max >= booleans.len() {
                booleans.resize_with(max + 1, Default::default);
            };
        }

        // Ints
        match integers {
            IntPlotType::Vec(ref vec) => {
                if let Some(&(mut max)) = int_db.keys().max() {
                    if *me3_imported_me1 {
                        max += 10_000;
                    }

                    let mut vec = vec.borrow_mut();
                    if max >= vec.len() {
                        vec.resize_with(max + 1, Default::default);
                    };
                }
            }
            IntPlotType::IndexMap(ref index_map) => {
                for mut key in int_db.keys().copied() {
                    if *me3_imported_me1 {
                        key += 10_000;
                    }
                    index_map.borrow_mut().entry(key as i32).or_default();
                }
            }
        }
    }
}
