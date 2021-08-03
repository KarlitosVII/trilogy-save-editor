use indexmap::IndexMap;

use crate::{gui::RcUi, save_data::shared::plot::BitVec};

mod bonus_powers;
mod plot_category;
mod raw_plot;

pub use self::{bonus_powers::*, plot_category::*, raw_plot::*};

#[derive(Clone)]
pub enum IntegerPlotType {
    Vec(RcUi<Vec<RcUi<i32>>>),
    IndexMap(RcUi<IndexMap<i32, RcUi<i32>>>),
}

impl PartialEq for IntegerPlotType {
    fn eq(&self, other: &IntegerPlotType) -> bool {
        match self {
            IntegerPlotType::Vec(vec) => match other {
                IntegerPlotType::Vec(other) => vec == other,
                _ => false,
            },
            IntegerPlotType::IndexMap(index_map) => match other {
                IntegerPlotType::IndexMap(other) => index_map == other,
                _ => false,
            },
        }
    }
}

#[derive(Clone)]
pub enum FloatPlotType {
    Vec(RcUi<Vec<RcUi<f32>>>),
    IndexMap(RcUi<IndexMap<i32, RcUi<f32>>>),
}

impl PartialEq for FloatPlotType {
    fn eq(&self, other: &FloatPlotType) -> bool {
        match self {
            FloatPlotType::Vec(vec) => match other {
                FloatPlotType::Vec(other) => vec == other,
                _ => false,
            },
            FloatPlotType::IndexMap(index_map) => match other {
                FloatPlotType::IndexMap(other) => index_map == other,
                _ => false,
            },
        }
    }
}

#[derive(Clone)]
pub enum PlotType {
    Boolean(RcUi<BitVec>),
    Integer(IntegerPlotType),
    Float(FloatPlotType),
}

impl PartialEq for PlotType {
    fn eq(&self, other: &PlotType) -> bool {
        match self {
            PlotType::Boolean(boolean) => match other {
                PlotType::Boolean(other) => boolean == other,
                _ => false,
            },
            PlotType::Integer(integer) => match other {
                PlotType::Integer(other) => integer == other,
                _ => false,
            },
            PlotType::Float(float) => match other {
                PlotType::Float(other) => float == other,
                _ => false,
            },
        }
    }
}
