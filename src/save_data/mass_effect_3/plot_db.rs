use indexmap::IndexMap;
use serde::Deserialize;

use crate::save_data::shared::plot::PlotCategory;

#[derive(Deserialize)]
pub struct Me3PlotDb {
    pub general: PlotCategory,
    pub crew: IndexMap<String, PlotCategory>,
    pub romance: IndexMap<String, PlotCategory>,
    pub missions: IndexMap<String, PlotCategory>,
    pub citadel_dlc: IndexMap<String, PlotCategory>,
    pub normandy: IndexMap<String, PlotCategory>,
    pub appearances: IndexMap<String, PlotCategory>,
    pub weapons_powers: IndexMap<String, PlotVariable>,
    pub intel: PlotCategory,
}

#[derive(Deserialize)]
pub struct PlotVariable {
    pub booleans: IndexMap<usize, String>,
    pub variables: IndexMap<String, String>,
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use std::fs;

    use crate::save_data::shared::plot::RawPlotDb;

    use super::*;

    #[test]
    fn deserialize_plot_db() -> Result<()> {
        let input = fs::read_to_string("databases/me3_plot_db.ron")?;
        let _me3_plot_db: Me3PlotDb = ron::from_str(&input)?;

        Ok(())
    }

    #[test]
    fn deserialize_raw_plot_db() -> Result<()> {
        let input = fs::read_to_string("databases/me3_raw_plot_db.ron")?;
        let _me3_raw_plot_db: RawPlotDb = ron::from_str(&input)?;

        Ok(())
    }
}
