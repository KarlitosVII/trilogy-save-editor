use indexmap::IndexMap;
use serde::Deserialize;

use crate::save_data::common::plot::KnownPlot;

#[derive(Deserialize)]
pub struct Me2KnownPlot {
    pub player: KnownPlot,
    pub crew: IndexMap<String, KnownPlot>,
    pub romance: IndexMap<String, KnownPlot>,
    pub missions: IndexMap<String, KnownPlot>,
    pub loyalty_missions: IndexMap<String, KnownPlot>,
    pub research_upgrades: IndexMap<String, KnownPlot>,
    pub rewards: KnownPlot,
    pub captains_cabin: KnownPlot,
}

#[cfg(test)]
mod test {
    use anyhow::*;
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn deserialize_know_plot() -> Result<()> {
        let mut input = String::new();
        {
            let mut file = File::open("plot/Me2KnownPlot.ron")?;
            file.read_to_string(&mut input)?;
        }

        let _me2_known_plot: Me2KnownPlot = ron::from_str(&input)?;

        Ok(())
    }
}
