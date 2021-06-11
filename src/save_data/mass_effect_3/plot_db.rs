use indexmap::IndexMap;
use serde::Deserialize;

use crate::save_data::{mass_effect_1::plot_db::Me1PlotDb, shared::plot::PlotCategory};

#[derive(Deserialize)]
pub struct Me3PlotDb {
    pub general: PlotCategory,
    pub appearances: IndexMap<String, PlotCategory>,
    pub crew: IndexMap<String, PlotCategory>,
    pub romance: IndexMap<String, PlotCategory>,
    pub missions: IndexMap<String, PlotCategory>,
    pub citadel_dlc: IndexMap<String, PlotCategory>,
    pub normandy: IndexMap<String, PlotCategory>,
    pub intel: PlotCategory,
    pub weapons_powers: IndexMap<String, PlotVariable>,
    pub me1_imported: Me1PlotDb,
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
    fn deserialize_plot_db() -> Result<()> {
        let mut input = String::new();
        {
            let mut file = File::open("databases/me3_plot_db.ron")?;
            file.read_to_string(&mut input)?;
        }

        let _me3_plot_db: Me3PlotDb = ron::from_str(&input)?;

        Ok(())
    }
}
