use flume::{Receiver, Sender};
use imgui::{Ui as ImguiUi, *};
use std::fmt::Debug;

use crate::{event_handler::MainEvent, mass_effect_3::Me3SaveGame};

mod support;

// Events
pub enum UiEvent {
    MassEffect3(Me3SaveGame),
}

enum SaveGame {
    MassEffect3(Me3SaveGame),
    None,
}

// UI
pub fn run<F>(event_addr: Sender<MainEvent>, rx: Receiver<UiEvent>, exit_fn: F)
where
    F: Fn() + 'static,
{
    let mut save_game = SaveGame::None;

    // UI
    let system = support::init("Trilogy Save Editor", 1000.0, 700.0);
    system.main_loop(
        move |_, imgui| {
            rx.try_iter().for_each(|ui_event| match ui_event {
                UiEvent::MassEffect3(me3_save_game) => save_game = SaveGame::MassEffect3(me3_save_game),
            });

            let ui = Ui::new(imgui, &event_addr);
            ui.draw(&mut save_game);
        },
        exit_fn,
    );
}

struct Ui<'a> {
    imgui: &'a ImguiUi<'a>,
    event_addr: Sender<MainEvent>,
}

impl<'a> Ui<'a> {
    fn new(imgui: &'a ImguiUi<'a>, event_addr: &Sender<MainEvent>) -> Self {
        Self {
            imgui,
            event_addr: Sender::clone(event_addr),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn draw(&self, save_game: &mut SaveGame) {
        let imgui = &self.imgui;
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
                    self.open_mass_effect_3();
                }
                menu_bar.end();
            }

            // Error popup
            // {
            //     let ErrorState { errors, is_opened } = error_state;
            //     if *is_opened {
            //         imgui.open_popup(im_str!("Erreur"));
            //     }

            //     PopupModal::new(im_str!("Erreur"))
            //         .always_auto_resize(true)
            //         .build(imgui, || {
            //             errors.iter().for_each(|error| {
            //                 imgui.text(error.to_string());
            //             });
            //             imgui.separator();

            //             if imgui.button_with_size(im_str!("OK"), [70.0, 0.0]) {
            //                 *is_opened = false;
            //                 errors.clear();
            //                 imgui.close_current_popup();
            //             }
            //         });
            // }

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
        let imgui = &self.imgui;

        // Tabs
        TabBar::new(im_str!("main-tabs")).build(imgui, || {
            TabItem::new(im_str!("Raw")).build(imgui, || {
                TreeNode::new(im_str!("Mass Effect 3")).leaf(true).build(imgui, || {
                    let Me3SaveGame {
                        seconds_played,
                        difficulty,
                        end_game_state,
                        conversation_mode,
                        timestamp,
                        ..
                    } = save_game;

                    TreeNode::new(im_str!("General")).default_open(true).build(imgui, || {
                        self.draw_edit_f32("Second Played", seconds_played);
                        self.draw_enum(difficulty);
                        self.draw_enum(end_game_state);
                        self.draw_enum(conversation_mode);
                        self.draw_enum(timestamp);
                    });
                });
            });
        });
    }

    fn draw_edit_i32(&self, text: &str, value: &mut i32) {
        let imgui = &self.imgui;

        let width = imgui.push_item_width(100.0);
        imgui.input_int(&im_str!("{}", text), value).build();
        width.pop(imgui);
    }

    fn draw_edit_f32(&self, text: &str, value: &mut f32) {
        let imgui = &self.imgui;

        let width = imgui.push_item_width(100.0);
        imgui.input_float(&im_str!("{}", text), value).build();
        width.pop(imgui);
    }

    fn draw_enum<T: Debug>(&self, value: &T) {
        let imgui = &self.imgui;

        imgui.text(im_str!("{:?}", value));
    }

    // fn draw_log(&self, log: &IndexMap<String, Vec<UiEvent>>) {
    //     let imgui = &self.imgui;
    //     ChildWindow::new(im_str!("log")).build(imgui, || {
    //         if !log.is_empty() {
    //             log.iter().for_each(|(grab_id, grab_log)| {
    //                 if CollapsingHeader::new(&ImString::new(grab_id))
    //                     .default_open(true)
    //                     .build(imgui)
    //                 {
    //                     self.draw_log_grab(grab_id, grab_log);
    //                 }
    //             });
    //         }
    //         if imgui.scroll_y() >= imgui.scroll_max_y() {
    //             imgui.set_scroll_here_y();
    //         }
    //     });
    // }

    // fn draw_log_grab(&self, id: &str, grab_log: &[UiEvent]) {
    //     let imgui = &self.imgui;
    //     if !grab_log.is_empty() {
    //         let mut clipper = ListClipper::new(grab_log.len() as i32).begin(imgui);
    //         while clipper.step() {
    //             for line in clipper.display_start()..clipper.display_end() {
    //                 ChildWindow::new(&im_str!("{}-{}", id, line))
    //                     .size([0.0, 19.0])
    //                     .build(imgui, || match &grab_log[line as usize] {
    //                         UiEvent::Error(e) => imgui.text_colored([1.0, 0.0, 0.0, 1.0], im_str!("Erreur : {}", e)),
    //                         UiEvent::Message(msg) => imgui.text_colored([0.0, 1.0, 0.0, 1.0], msg),
    //                         UiEvent::Download(path, progress) => {
    //                             ProgressBar::new(*progress)
    //                                 .overlay_text(&ImString::new(path))
    //                                 .size([-38.0, 0.0])
    //                                 .build(imgui);
    //                             imgui.same_line();
    //                             imgui.text(im_str!("{}%", (*progress * 100.0) as i32));
    //                         }
    //                         UiEvent::Finish => imgui.separator(),
    //                         UiEvent::Start => unreachable!(),
    //                     });
    //             }
    //         }
    //     }
    // }

    // Style
    fn style_colors(&self) -> Vec<ColorStackToken> {
        const RED_BG: [f32; 4] = [0.40, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [0.53, 0.0, 0.0, 1.0];
        const RED_ACTIVE: [f32; 4] = [0.68, 0.0, 0.0, 1.0];
        const RED_HOVER: [f32; 4] = [0.86, 0.0, 0.0, 1.0];

        vec![
            self.imgui.push_style_color(StyleColor::TitleBgActive, RED_ACTIVE),
            self.imgui.push_style_color(StyleColor::FrameBg, RED_BG),
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

    // Actions
    fn open_mass_effect_3(&self) {
        let _ = self.event_addr.send(MainEvent::OpenMassEffect3);
    }
}
