mod bonus_powers;
mod head_morph;
mod link;
mod plot_category;
mod raw_plot;

pub use self::{bonus_powers::*, head_morph::*, link::*, plot_category::*, raw_plot::*};

use indexmap::IndexMap;
use yew::prelude::*;

use crate::save_data::{
    shared::plot::{BitVec, PlotTable},
    RcCell, RcRef,
};

use super::raw_ui::RawUi;

#[derive(Clone)]
pub enum IntPlotType {
    Vec(RcRef<Vec<RcCell<i32>>>),
    IndexMap(RcRef<IndexMap<i32, RcCell<i32>>>),
}

impl PartialEq for IntPlotType {
    fn eq(&self, other: &IntPlotType) -> bool {
        match (self, other) {
            (IntPlotType::Vec(vec), IntPlotType::Vec(other)) => vec == other,
            (IntPlotType::IndexMap(index_map), IntPlotType::IndexMap(other)) => index_map == other,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub enum FloatPlotType {
    Vec(RcRef<Vec<RcCell<f32>>>),
    IndexMap(RcRef<IndexMap<i32, RcCell<f32>>>),
}

impl PartialEq for FloatPlotType {
    fn eq(&self, other: &FloatPlotType) -> bool {
        match (self, other) {
            (FloatPlotType::Vec(vec), FloatPlotType::Vec(other)) => vec == other,
            (FloatPlotType::IndexMap(index_map), FloatPlotType::IndexMap(other)) => {
                index_map == other
            }
            _ => false,
        }
    }
}

#[derive(Clone)]
pub enum PlotType {
    Boolean(RcRef<BitVec>),
    Int(IntPlotType),
    Float(FloatPlotType),
}

impl PartialEq for PlotType {
    fn eq(&self, other: &PlotType) -> bool {
        match (self, other) {
            (PlotType::Boolean(booleans), PlotType::Boolean(other)) => booleans == other,
            (PlotType::Int(integers), PlotType::Int(other)) => integers == other,
            (PlotType::Float(floats), PlotType::Float(other)) => floats == other,
            _ => false,
        }
    }
}

impl RawUi for RcRef<PlotTable> {
    fn view(&self, _: &str) -> yew::Html {
        html! {
            <Link tab="Raw Plot">{ "Raw Plot" }</Link>
        }
    }
}
