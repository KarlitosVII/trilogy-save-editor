use anyhow::Error;
use flume::{Receiver, Sender};
use imgui::{Ui as ImguiUi, *};
use std::{
    fmt::Debug,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{event_handler::MainEvent, mass_effect_3::Me3SaveGame};

mod support;

// States
#[derive(Default)]
struct ErrorState {
    errors: Vec<Error>,
    is_opened: bool,
}

// Events
pub enum UiEvent {
    Error(Error),
    MassEffect3(Me3SaveGame),
}

enum SaveGame {
    None,
    MassEffect3(Me3SaveGame),
}

// UI
pub fn run<F>(event_addr: Sender<MainEvent>, rx: Receiver<UiEvent>, exit_fn: F)
where
    F: Fn() + 'static,
{
    let mut error_state = ErrorState::default();
    let mut save_game = SaveGame::None;

    // UI
    let system = support::init("Trilogy Save Editor", 1000.0, 700.0);
    system.main_loop(
        move |_, imgui| {
            rx.try_iter().for_each(|ui_event| match ui_event {
                UiEvent::MassEffect3(me3_save_game) => save_game = SaveGame::MassEffect3(me3_save_game),
                UiEvent::Error(err) => {
                    error_state.errors.push(err);
                    error_state.is_opened = true;
                }
            });

            let ui = Ui::new(imgui, &event_addr);
            ui.draw(&mut error_state,&mut save_game);
        },
        exit_fn,
    );
}

struct Ui<'a> {
    imgui: &'a ImguiUi<'a>,
    event_addr: Sender<MainEvent>,
    bg_count: AtomicUsize,
}

impl<'a> Ui<'a> {
    fn new(imgui: &'a ImguiUi<'a>, event_addr: &Sender<MainEvent>) -> Self {
        Self {
            imgui,
            event_addr: Sender::clone(event_addr),
            bg_count: AtomicUsize::new(0),
        }
    }

    fn draw(&self, error_state: &mut ErrorState, save_game: &mut SaveGame) {
        let imgui = self.imgui;

        // Main window
        let window = Window::new(im_str!("main"))
            .size([1000.0, 700.0], Condition::Always)
            .position([0.0, 0.0], Condition::Always)
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .menu_bar(true)
            .collapsible(false);

        let mut colors = self.style_colors();
        let style = imgui.push_style_var(StyleVar::WindowRounding(0.0));
        window.build(imgui, || {
            // Main menu bar
            if let Some(menu_bar) = imgui.begin_menu_bar() {
                if imgui.button(im_str!("Open")) {
                    let _ = self.event_addr.send(MainEvent::OpenMassEffect3);
                }
                menu_bar.end();
            }

            // Error popup
            {
                let ErrorState { errors, is_opened } = error_state;
                if *is_opened {
                    imgui.open_popup(im_str!("Error"));
                }

                PopupModal::new(im_str!("Error"))
                    .always_auto_resize(true)
                    .build(imgui, || {
                        errors.iter().for_each(|error| {
                            imgui.text(error.to_string());
                        });
                        imgui.separator();

                        if imgui.button_with_size(im_str!("OK"), [70.0, 0.0]) {
                            *is_opened = false;
                            errors.clear();
                            imgui.close_current_popup();
                        }
                    });
            }

            match save_game {
                SaveGame::None => imgui.text(im_str!("Rien ici")),
                SaveGame::MassEffect3(save_game) => self.draw_mass_effect_3(save_game),
            };
        });

        // Style
        for color in colors.drain(..) {
            color.pop();
        }
        style.pop();
    }

    fn draw_mass_effect_3(&self, save_game: &mut Me3SaveGame) {
        let imgui = self.imgui;

        // Tabs
        TabBar::new(im_str!("main-tabs")).build(imgui, || {
            TabItem::new(im_str!("Raw")).build(imgui, || {
                if CollapsingHeader::new(im_str!("General"))
                    .default_open(true)
                    .build(imgui)
                {
                    let Me3SaveGame {
                        seconds_played,
                        difficulty,
                        end_game_state,
                        conversation_mode,
                        timestamp,
                        ..
                    } = save_game;

                    self.draw_edit_f32("Second Played", seconds_played);
                    self.draw_enum("Difficulty", difficulty);
                    self.draw_enum("EndGameState", end_game_state);
                    self.draw_enum("ConversationMode", conversation_mode);
                    TreeNode::new(im_str!("Timestamp")).default_open(true).build(imgui, || {
                        self.draw_edit_i32("Second since midnight", &mut timestamp.seconds_since_midnight);
                        self.draw_edit_i32("Day", &mut timestamp.day);
                        self.draw_edit_i32("Month", &mut timestamp.month);
                        self.draw_edit_i32("Year", &mut timestamp.year);
                    });
                }
            });
        });
    }

    fn draw_edit_i32(&self, text: &str, value: &mut i32) {
        let imgui = self.imgui;

        self.draw_colored_bg(text, || {
            let width = imgui.push_item_width(100.0);
            Drag::new(&im_str!("{}", text)).build(imgui, value);
            Self::show_help_marker(imgui, "Drag or double-click to edit");
            width.pop(imgui);
        });
    }

    fn draw_edit_f32(&self, text: &str, value: &mut f32) {
        let imgui = self.imgui;

        self.draw_colored_bg(text, || {
            let width = imgui.push_item_width(100.0);
            Drag::new(&im_str!("{}", text)).build(imgui, value);
            Self::show_help_marker(imgui, "Drag or double-click to edit");
            width.pop(imgui);
        });
    }

    fn draw_enum<T: Debug>(&self, text: &str, value: &T) {
        let imgui = self.imgui;

        self.draw_colored_bg(text, || {
            imgui.align_text_to_frame_padding();
            imgui.text(im_str!("{:?}", value));
            imgui.same_line_with_pos(200.0);
            imgui.text(text);
        });
    }

    fn draw_colored_bg<F>(&self, id: &str, inner: F)
    where
        F: FnOnce() -> (),
    {
        let bg = self.bg_colors();
        ChildWindow::new(id).size([0.0, 19.0]).build(self.imgui, || {
            inner();
        });
        bg.pop();
    }

    fn bg_colors(&self) -> ColorStackToken {
        const BG_DARK: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
        const BG_LIGHT: [f32; 4] = [0.15, 0.15, 0.15, 1.0];

        let bg = [BG_DARK, BG_LIGHT];

        let bg_count = self.bg_count.fetch_add(1, Ordering::AcqRel);
        self.imgui
            .push_style_color(StyleColor::ChildBg, bg[bg_count % bg.len()])
    }

    fn show_help_marker(imgui: &ImguiUi, desc: &str) {
        imgui.same_line();
        imgui.text_disabled(im_str!("(?)"));
        if imgui.is_item_hovered() {
            imgui.tooltip(|| {
                imgui.text(desc);
            });
        }
    }

    // Style
    fn style_colors(&self) -> Vec<ColorStackToken> {
        const RED_BG: [f32; 4] = [0.40, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [0.53, 0.0, 0.0, 1.0];
        const RED_ACTIVE: [f32; 4] = [0.68, 0.0, 0.0, 1.0];
        const RED_HOVER: [f32; 4] = [0.86, 0.0, 0.0, 1.0];

        vec![
            self.imgui.push_style_color(StyleColor::TitleBgActive, RED_ACTIVE),
            self.imgui.push_style_color(StyleColor::FrameBg, RED_BG),
            self.imgui.push_style_color(StyleColor::FrameBgActive, RED_ACTIVE),
            self.imgui.push_style_color(StyleColor::FrameBgHovered, RED_HOVER),
            self.imgui.push_style_color(StyleColor::TextSelectedBg, RED_ACTIVE),
            self.imgui.push_style_color(StyleColor::Button, RED),
            self.imgui.push_style_color(StyleColor::ButtonActive, RED_ACTIVE),
            self.imgui.push_style_color(StyleColor::ButtonHovered, RED_HOVER),
            self.imgui.push_style_color(StyleColor::Tab, RED),
            self.imgui.push_style_color(StyleColor::TabActive, RED_ACTIVE),
            self.imgui.push_style_color(StyleColor::TabHovered, RED_HOVER),
            self.imgui.push_style_color(StyleColor::Header, RED_BG),
            self.imgui.push_style_color(StyleColor::HeaderActive, RED_ACTIVE),
            self.imgui.push_style_color(StyleColor::HeaderHovered, RED_HOVER),
            self.imgui
                .push_style_color(StyleColor::PlotHistogram, [1.0, 0.62, 0.24, 1.0]), // Progress bar
        ]
    }
}
