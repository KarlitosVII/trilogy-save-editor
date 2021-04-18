use indexmap::IndexMap;
use serde::Deserialize;

use crate::save_data::common::plot::KnownPlot;

#[derive(Deserialize)]
pub struct Me1KnownPlot {
    pub player_squad: IndexMap<String, KnownPlot>,
    pub missions: IndexMap<String, KnownPlot>,
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
            let mut file = File::open("plot/Me1KnownPlot.ron")?;
            file.read_to_string(&mut input)?;
        }

        let _me1_known_plot: Me1KnownPlot = ron::from_str(&input)?;

        Ok(())
    }
}
