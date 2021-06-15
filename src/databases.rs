use anyhow::{anyhow, Error, Result};
use lazy_static::lazy_static;
use std::fs;

use crate::save_data::{
    mass_effect_1::{item_db::Me1ItemDb, plot_db::Me1PlotDb},
    mass_effect_2::plot_db::Me2PlotDb,
    mass_effect_3::plot_db::Me3PlotDb,
};

lazy_static! {
    static ref ME1_PLOT_DB: Result<Me1PlotDb> = {
        let input = fs::read_to_string("databases/me1_plot_db.ron")?;
        ron::from_str(&input).map_err(Error::new)
    };
    static ref ME1_ITEM_DB: Result<Me1ItemDb> = {
        let input = fs::read_to_string("databases/me1_item_db.ron")?;
        ron::from_str(&input).map_err(Error::new)
    };
    static ref ME2_PLOT_DB: Result<Me2PlotDb> = {
        let input = fs::read_to_string("databases/me2_plot_db.ron")?;
        ron::from_str(&input).map_err(Error::new)
    };
    static ref ME3_PLOT_DB: Result<Me3PlotDb> = {
        let input = fs::read_to_string("databases/me3_plot_db.ron")?;
        ron::from_str(&input).map_err(Error::new)
    };
}

pub fn initialize() -> Result<()> {
    // ME1
    ME1_PLOT_DB
        .as_ref()
        .map_err(|e| anyhow!(e).context("Failed to parse databases/me1_plot_db.ron"))?;
    ME1_ITEM_DB
        .as_ref()
        .map_err(|e| anyhow!(e).context("Failed to parse databases/me1_item_db.ron"))?;
    // ME2
    ME2_PLOT_DB
        .as_ref()
        .map_err(|e| anyhow!(e).context("Failed to parse databases/me2_plot_db.ron"))?;
    // ME3
    ME3_PLOT_DB
        .as_ref()
        .map_err(|e| anyhow!(e).context("Failed to parse databases/me3_plot_db.ron"))?;
    Ok(())
}

pub struct Database;
impl Database {
    // ME1
    pub fn me1_plot() -> Option<&'static Me1PlotDb> {
        ME1_PLOT_DB.as_ref().ok()
    }
    pub fn me1_item() -> Option<&'static Me1ItemDb> {
        ME1_ITEM_DB.as_ref().ok()
    }
    // ME2
    pub fn me2_plot() -> Option<&'static Me2PlotDb> {
        ME2_PLOT_DB.as_ref().ok()
    }
    // ME3
    pub fn me3_plot() -> Option<&'static Me3PlotDb> {
        ME3_PLOT_DB.as_ref().ok()
    }
}
