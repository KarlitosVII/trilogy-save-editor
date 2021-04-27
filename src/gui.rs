use anyhow::Error;
use flume::{Receiver, Sender};
use imgui::{
    im_str, ColorStackToken, Condition, ImString, PopupModal, ProgressBar, StyleColor, StyleVar,
    Ui, Window,
};
use wfd::DialogParams;

use crate::{
    event_handler::{MainEvent, SaveGame},
    save_data::{
        common::appearance::{HasHeadMorph, HeadMorph},
        mass_effect_1::known_plot::Me1KnownPlot,
        mass_effect_2::known_plot::Me2KnownPlot,
        mass_effect_3::known_plot::Me3KnownPlot,
    },
};

mod backend;
mod imgui_utils;
mod mass_effect_1;
mod mass_effect_2;
mod mass_effect_3;
mod raw_ui;

static NOTIFICATION_TIME: f64 = 1.5;

// States
#[derive(Default)]
struct NotificationState {
    string: ImString,
    close_time: f64,
}

#[derive(Default)]
pub struct KnownPlotsState {
    me1: Option<Me1KnownPlot>,
    me2: Option<Me2KnownPlot>,
    me3: Option<Me3KnownPlot>,
}

#[derive(Default)]
struct State {
    save_game: Option<SaveGame>,
    error: Option<Error>,
    notification: Option<NotificationState>,
    known_plots: KnownPlotsState,
}

// Events
pub enum UiEvent {
    Error(Error),
    Notification(&'static str),
    OpenedSave(SaveGame),
    LoadedMe1KnownPlot(Me1KnownPlot),
    LoadedMe2KnownPlot(Me2KnownPlot),
    LoadedMe3KnownPlot(Me3KnownPlot),
    ImportedHeadMorph(HeadMorph),
}

// UI
pub fn run(event_addr: Sender<MainEvent>, rx: Receiver<UiEvent>) {
    let mut state = State::default();

    let _ = event_addr.send(MainEvent::LoadKnownPlots);

    // UI
    let system = backend::init("Trilogy Save Editor", 1000.0, 700.0);
    system.main_loop(move |_, ui| {
        rx.try_iter().for_each(|ui_event| match ui_event {
            UiEvent::Error(err) => {
                state.error = Some(err);
            }
            UiEvent::Notification(string) => {
                state.notification = Some(NotificationState {
                    string: ImString::new(string),
                    close_time: ui.time() + NOTIFICATION_TIME,
                })
            }
            UiEvent::OpenedSave(opened_save_game) => {
                state.save_game = Some(opened_save_game);
            }
            UiEvent::LoadedMe1KnownPlot(me1_known_plot) => {
                state.known_plots.me1 = Some(me1_known_plot)
            }
            UiEvent::LoadedMe2KnownPlot(me2_known_plot) => {
                state.known_plots.me2 = Some(me2_known_plot)
            }
            UiEvent::LoadedMe3KnownPlot(me3_known_plot) => {
                state.known_plots.me3 = Some(me3_known_plot)
            }
            UiEvent::ImportedHeadMorph(head_morph) => {
                let has_head_morph =
                    HasHeadMorph { has_head_morph: true, head_morph: Some(head_morph) };
                match state.save_game.as_mut() {
                    Some(SaveGame::MassEffect2(savegame)) => {
                        savegame.player.appearance.head_morph = has_head_morph
                    }
                    Some(SaveGame::MassEffect3(savegame)) => {
                        savegame.player.appearance.head_morph = has_head_morph
                    }
                    _ => unreachable!(),
                }
            }
        });

        let ui = Gui::new(ui, &event_addr);
        ui.draw(&mut state);
    });
}

pub struct Gui<'ui> {
    ui: &'ui Ui<'ui>,
    event_addr: Sender<MainEvent>,
}

impl<'ui> Gui<'ui> {
    fn new(ui: &'ui Ui<'ui>, event_addr: &Sender<MainEvent>) -> Self {
        Self { ui, event_addr: Sender::clone(event_addr) }
    }

    fn draw(&self, state: &mut State) -> Option<()> {
        let ui = self.ui;

        // Main window
        let window = Window::new(im_str!("###main"))
            .size(ui.io().display_size, Condition::Always)
            .position([0.0, 0.0], Condition::Always)
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .menu_bar(true)
            .bring_to_front_on_focus(false)
            .collapsible(false);

        // Pop on drop
        let _colors = self.style_colors(match state.save_game {
            None => Theme::MassEffect3,
            Some(SaveGame::MassEffect1(_)) => Theme::MassEffect1,
            Some(SaveGame::MassEffect2(_)) => Theme::MassEffect2,
            Some(SaveGame::MassEffect3(_)) => Theme::MassEffect3,
        });
        let _style = ui.push_style_var(StyleVar::WindowRounding(0.0));

        // Window
        if let Some(_t) = window.begin(ui) {
            // Main menu bar
            if let Some(_t) = ui.begin_menu_bar() {
                if ui.button(im_str!("Open")) {
                    self.open_save();
                }
                if ui.button(im_str!("Save")) {
                    self.save_save(&state.save_game);
                }
            }

            // Error popup
            self.draw_error(&mut state.error);

            // Notification
            self.draw_nofification_overlay(&mut state.notification);

            // Game
            match &mut state.save_game {
                None => ui.text(im_str!("Rien ici")),
                Some(SaveGame::MassEffect1(save_game)) => {
                    self.draw_mass_effect_1(save_game, &state.known_plots)?
                }
                Some(SaveGame::MassEffect2(save_game)) => {
                    self.draw_mass_effect_2(save_game, &state.known_plots)?
                }
                Some(SaveGame::MassEffect3(save_game)) => {
                    self.draw_mass_effect_3(save_game, &state.known_plots)?
                }
            };
        }
        Some(())
    }

    fn draw_error(&self, option_error: &mut Option<Error>) {
        let ui = self.ui;

        if let Some(error) = option_error {
            ui.open_popup(im_str!("Error###error"));

            if let Some(_t) =
                PopupModal::new(im_str!("Error###error")).always_auto_resize(true).begin_popup(ui)
            {
                ui.text(error.to_string());

                let chain = error.chain().skip(1);
                if chain.len() != 0 {
                    ui.separator();
                    for error in chain {
                        ui.text(error.to_string());
                    }
                }
                ui.separator();

                if ui.button_with_size(im_str!("OK"), [70.0, 0.0]) {
                    *option_error = None;
                    ui.close_current_popup();
                }
            }
        }
    }

    fn draw_nofification_overlay(&self, notification: &mut Option<NotificationState>) {
        if let Some(NotificationState { string, close_time }) = notification {
            let ui = self.ui;
            let time = ui.time();

            let _style = ui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 0.3]);
            let window = Window::new(im_str!("###notification"))
                .position([ui.io().display_size[0] / 2.0, 5.0], Condition::Always)
                .title_bar(false)
                .resizable(false)
                .movable(false)
                .always_auto_resize(true);

            if let Some(_t) = window.begin(ui) {
                ui.text(&string);

                let remaining = (*close_time - time) / NOTIFICATION_TIME;
                ProgressBar::new(remaining as f32)
                    .overlay_text(&ImString::new("time_bar"))
                    .size([-0.0001, 2.0])
                    .build(ui);
            }

            if *close_time < time {
                *notification = None;
            }
        }
    }

    fn draw_help_marker(&self, desc: &str) {
        let ui = self.ui;

        ui.text_disabled(im_str!("(?)"));
        if ui.is_item_hovered() {
            let _t = ui.begin_tooltip();
            ui.text(desc);
        }
    }

    // Style
    fn style_colors(&self, game_theme: Theme) -> [ColorStackToken<'ui>; 20] {
        let ui = self.ui;
        let theme = match game_theme {
            Theme::MassEffect1 => ColorTheme {
                bg_color: [0.11, 0.32, 0.43, 1.0],
                color: [0.16, 0.42, 0.58, 1.0],
                active_color: [0.28, 0.55, 0.67, 1.0],
                hover_color: [0.83, 0.43, 0.17, 1.0],
            },
            Theme::MassEffect2 => ColorTheme {
                bg_color: [0.64, 0.32, 0.12, 1.0],
                color: [0.70, 0.37, 0.16, 1.0],
                active_color: [0.85, 0.49, 0.25, 1.0],
                hover_color: [0.22, 0.52, 0.23, 1.0],
            },
            Theme::MassEffect3 => ColorTheme {
                bg_color: [0.40, 0.0, 0.0, 1.0],
                color: [0.53, 0.0, 0.0, 1.0],
                active_color: [0.70, 0.0, 0.0, 1.0],
                hover_color: [0.02, 0.28, 0.43, 1.0],
            },
        };

        [
            ui.push_style_color(StyleColor::WindowBg, [0.05, 0.05, 0.05, 1.0]),
            ui.push_style_color(StyleColor::TitleBgActive, theme.active_color),
            ui.push_style_color(StyleColor::FrameBg, theme.bg_color),
            ui.push_style_color(StyleColor::FrameBgActive, theme.active_color),
            ui.push_style_color(StyleColor::FrameBgHovered, theme.hover_color),
            ui.push_style_color(StyleColor::TextSelectedBg, theme.active_color),
            ui.push_style_color(StyleColor::Button, theme.bg_color),
            ui.push_style_color(StyleColor::ButtonActive, theme.active_color),
            ui.push_style_color(StyleColor::ButtonHovered, theme.hover_color),
            ui.push_style_color(StyleColor::Tab, theme.color),
            ui.push_style_color(StyleColor::TabActive, theme.active_color),
            ui.push_style_color(StyleColor::TabHovered, theme.hover_color),
            ui.push_style_color(StyleColor::Header, theme.bg_color),
            ui.push_style_color(StyleColor::HeaderActive, theme.active_color),
            ui.push_style_color(StyleColor::HeaderHovered, theme.hover_color),
            ui.push_style_color(StyleColor::CheckMark, [1.0, 1.0, 1.0, 1.0]),
            ui.push_style_color(StyleColor::PlotHistogram, [1.0, 1.0, 1.0, 1.0]),
            ui.push_style_color(StyleColor::TableRowBg, [0.07, 0.07, 0.07, 1.0]),
            ui.push_style_color(StyleColor::TableRowBgAlt, [0.1, 0.1, 0.1, 1.0]),
            ui.push_style_color(StyleColor::TableBorderStrong, [0.20, 0.20, 0.20, 1.0]),
        ]
    }

    // Actions
    fn open_save(&self) {
        let result = wfd::open_dialog(DialogParams {
            file_types: vec![("Mass Effect Save", "*.MassEffectSave;*.pcsav")],
            ..Default::default()
        });

        if let Ok(result) = result {
            let _ = self.event_addr.send(MainEvent::OpenSave(result.selected_file_path));
        }
    }

    fn save_save(&self, save_game: &Option<SaveGame>) {
        if let Some(save_game) = save_game {
            let default_ext = match save_game {
                SaveGame::MassEffect1(_) => "MassEffectSave",
                SaveGame::MassEffect2(_) | SaveGame::MassEffect3(_) => "pcsav",
            };

            let result = wfd::save_dialog(DialogParams {
                default_extension: default_ext,
                file_types: vec![("Mass Effect Save", "*.MassEffectSave;*.pcsav")],
                ..Default::default()
            });

            if let Ok(result) = result {
                let _ = self
                    .event_addr
                    .send(MainEvent::SaveSave(result.selected_file_path, save_game.clone()));
            }
        }
    }
}

enum Theme {
    MassEffect1,
    MassEffect2,
    MassEffect3,
}

struct ColorTheme {
    bg_color: [f32; 4],
    color: [f32; 4],
    active_color: [f32; 4],
    hover_color: [f32; 4],
}
