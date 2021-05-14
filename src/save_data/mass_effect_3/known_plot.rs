use indexmap::IndexMap;
use serde::Deserialize;

use crate::save_data::{shared::plot::PlotCategory, mass_effect_1::known_plot::Me1KnownPlot};

#[derive(Deserialize)]
pub struct Me3KnownPlot {
    pub general: PlotCategory,
    pub appearances: IndexMap<String, PlotCategory>,
    pub crew: IndexMap<String, PlotCategory>,
    pub romance: IndexMap<String, PlotCategory>,
    pub missions: IndexMap<String, PlotCategory>,
    pub citadel_dlc: IndexMap<String, PlotCategory>,
    pub normandy: IndexMap<String, PlotCategory>,
    pub intel: PlotCategory,
    pub weapons_powers: IndexMap<String, PlotVariable>,
    pub me1_imported: Me1KnownPlot,
}

#[derive(Deserialize)]
pub struct PlotVariable {
    pub booleans: IndexMap<usize, String>,
    pub variables: IndexMap<String, String>,
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
            let mut file = File::open("plot/Me3KnownPlot.ron")?;
            file.read_to_string(&mut input)?;
        }

        let _me3_known_plot: Me3KnownPlot = ron::from_str(&input)?;

        Ok(())
    }
}
