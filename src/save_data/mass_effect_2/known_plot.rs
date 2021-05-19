use indexmap::IndexMap;
use serde::Deserialize;

use crate::save_data::shared::plot::PlotCategory;

#[derive(Deserialize)]
pub struct Me2KnownPlot {
    pub player: PlotCategory,
    pub crew: IndexMap<String, PlotCategory>,
    pub romance: IndexMap<String, PlotCategory>,
    pub missions: IndexMap<String, PlotCategory>,
    pub loyalty_missions: IndexMap<String, PlotCategory>,
    pub research_upgrades: IndexMap<String, PlotCategory>,
    pub rewards: PlotCategory,
    pub captains_cabin: PlotCategory,
    pub imported_me1: IndexMap<String, PlotCategory>,
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn deserialize_know_plot() -> Result<()> {
        let mut input = String::new();
        {
            let mut file = File::open("plot/me2_known_plot.ron")?;
            file.read_to_string(&mut input)?;
        }

        let _me2_known_plot: Me2KnownPlot = ron::from_str(&input)?;

        Ok(())
    }
}
