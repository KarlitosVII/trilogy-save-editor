use anyhow::{bail, Result};
use flume::{Receiver, Sender};
use indexmap::IndexMap;
use ron::ser::PrettyConfig;
use serde::Serialize;
use tokio::fs;

use crate::{
    event_handler::{MainEvent, SaveGame},
    gui::UiEvent,
    save_data::shared::plot::BoolVec,
};

pub async fn compare(
    event_addr: Sender<MainEvent>, rx: Receiver<UiEvent>, src: &str, cmp: &str,
) -> Result<()> {
    let _ = event_addr.send_async(MainEvent::OpenSave(src.to_owned())).await;
    let _ = event_addr.send_async(MainEvent::OpenSave(cmp.to_owned())).await;

    let mut src_save = None;
    let mut cmp_save = None;

    // Wait until the 2 saves are opened
    while let Ok(event) = rx.recv_async().await {
        match event {
            UiEvent::OpenedSave(save_game) => {
                // < 2 saves opened
                if src_save.is_none() || cmp_save.is_none() {
                    let opened_path = match save_game {
                        SaveGame::MassEffect1 { ref file_path, .. }
                        | SaveGame::MassEffect1Le { ref file_path, .. }
                        | SaveGame::MassEffect1LePs4 { ref file_path, .. }
                        | SaveGame::MassEffect2 { ref file_path, .. }
                        | SaveGame::MassEffect2Le { ref file_path, .. }
                        | SaveGame::MassEffect3 { ref file_path, .. } => file_path,
                    };

                    if opened_path == src {
                        src_save = Some(save_game);
                    } else {
                        cmp_save = Some(save_game);
                    }
                }
                // 2 saves opened
                if src_save.is_some() && cmp_save.is_some() {
                    break;
                }
            }
            UiEvent::Error(err) => {
                bail!(err);
            }
            _ => {}
        }
    }

    let (mut src_bools, mut src_ints, mut src_floats) = get_plot_from_save_game(src_save.unwrap());
    let (mut cmp_bools, mut cmp_ints, mut cmp_floats) = get_plot_from_save_game(cmp_save.unwrap());

    // Booleans
    let mut booleans = IndexMap::new();

    let max_len = src_bools.len().max(cmp_bools.len());
    for i in 0..max_len {
        // Add plot if missing
        if i >= src_bools.len() {
            src_bools.resize(i + 1, Default::default());
        }
        if i >= cmp_bools.len() {
            cmp_bools.resize(i + 1, Default::default());
        }
        let src = *src_bools.get(i).unwrap();
        let cmp = *cmp_bools.get(i).unwrap();

        if src != cmp {
            booleans.insert(i, BoolDifference { src, cmp });
        }
    }

    // Integers
    let mut integers = IndexMap::new();

    let mut keys: Vec<i32> = src_ints.keys().copied().collect();
    keys.extend(cmp_ints.keys());
    keys.sort_unstable();
    keys.dedup();

    for &k in &keys {
        let src = *src_ints.entry(k).or_default();
        let cmp = *cmp_ints.entry(k).or_default();

        if src != cmp {
            integers.insert(k, IntegerDifference { src, cmp });
        }
    }

    // Floats
    let mut floats = IndexMap::new();

    let mut keys: Vec<i32> = src_floats.keys().copied().collect();
    keys.extend(cmp_floats.keys());
    keys.sort_unstable();
    keys.dedup();

    for &k in &keys {
        let src = *src_floats.entry(k).or_default();
        let cmp = *cmp_floats.entry(k).or_default();

        // src != cmp
        if (src - cmp).abs() > f32::EPSILON {
            floats.insert(k, FloatDifference { src, cmp });
        }
    }

    // Output
    let differences = Differences { booleans, integers, floats };

    // Serialize
    let pretty_config = PrettyConfig::new().with_new_line(String::from('\n'));
    let export = ron::ser::to_string_pretty(&differences, pretty_config)?;

    fs::write("compare_result.ron", export.as_bytes()).await?;
    Ok(())
}

#[derive(Serialize)]
struct Differences {
    booleans: IndexMap<usize, BoolDifference>,
    integers: IndexMap<i32, IntegerDifference>,
    floats: IndexMap<i32, FloatDifference>,
}

#[derive(Serialize)]
struct BoolDifference {
    src: bool,
    cmp: bool,
}

#[derive(Serialize)]
struct IntegerDifference {
    src: i32,
    cmp: i32,
}

#[derive(Serialize)]
struct FloatDifference {
    src: f32,
    cmp: f32,
}

fn get_plot_from_save_game(
    save_game: SaveGame,
) -> (BoolVec, IndexMap<i32, i32>, IndexMap<i32, f32>) {
    match save_game {
        SaveGame::MassEffect1 { save_game, .. } => {
            let plot = save_game.state.plot;
            (
                plot.booleans,
                plot.integers.into_iter().enumerate().map(|(k, v)| (k as i32, v)).collect(),
                plot.floats.into_iter().enumerate().map(|(k, v)| (k as i32, v)).collect(),
            )
        }
        SaveGame::MassEffect1Le { save_game, .. } => {
            let plot = save_game.save_data.plot;
            (
                plot.booleans,
                plot.integers.into_iter().enumerate().map(|(k, v)| (k as i32, v)).collect(),
                plot.floats.into_iter().enumerate().map(|(k, v)| (k as i32, v)).collect(),
            )
        }
        SaveGame::MassEffect1LePs4 { save_game, .. } => {
            let plot = save_game.plot;
            (
                plot.booleans,
                plot.integers.into_iter().enumerate().map(|(k, v)| (k as i32, v)).collect(),
                plot.floats.into_iter().enumerate().map(|(k, v)| (k as i32, v)).collect(),
            )
        }
        SaveGame::MassEffect2 { save_game, .. } => {
            let plot = save_game.plot;
            (
                plot.booleans,
                plot.integers.into_iter().enumerate().map(|(k, v)| (k as i32, v)).collect(),
                plot.floats.into_iter().enumerate().map(|(k, v)| (k as i32, v)).collect(),
            )
        }
        SaveGame::MassEffect2Le { save_game, .. } => {
            let plot = save_game.plot;
            (
                plot.booleans,
                plot.integers.into_iter().enumerate().map(|(k, v)| (k as i32, v)).collect(),
                plot.floats.into_iter().enumerate().map(|(k, v)| (k as i32, v)).collect(),
            )
        }
        SaveGame::MassEffect3 { save_game, .. } => {
            let plot = save_game.plot;
            (plot.booleans, plot.integers, plot.floats)
        }
    }
}
