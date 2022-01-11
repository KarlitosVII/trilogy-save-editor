mod general;
mod plot;
mod plot_variable;
mod raw_plot;

pub use self::{general::*, plot::*, plot_variable::*, raw_plot::*};

use yew::prelude::*;

use crate::{
    gui::{raw_ui::RawUi, shared::Link},
    save_data::{mass_effect_3::plot::PlotTable, RcRef},
};

impl RawUi for RcRef<PlotTable> {
    fn view(&self, _: &str) -> yew::Html {
        html! {
            <Link tab="Raw Plot">{ "Raw Plot" }</Link>
        }
    }
}
