use indexmap::IndexMap;
use serde::Deserialize;

use crate::save_data::shared::plot::PlotCategory;

#[derive(Deserialize)]
pub struct Me2PlotDb {
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
    use std::fs;

    use super::*;

    #[test]
    fn deserialize_plot_db() -> Result<()> {
        let input = fs::read_to_string("databases/me2_plot_db.ron")?;
        let _me2_plot_db: Me2PlotDb = ron::from_str(&input)?;

        Ok(())
    }
}
