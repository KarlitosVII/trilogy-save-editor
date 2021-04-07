use anyhow::Error;
use flume::{Receiver, Sender};
use imgui::{Ui as ImguiUi, *};
use indexmap::IndexMap;
use std::{
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{event_handler::MainEvent, mass_effect_3::Me3SaveGame, save_data::SaveData};

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
    MassEffect3(Box<Me3SaveGame>),
}

enum SaveGame {
    None,
    MassEffect3(Box<Me3SaveGame>),
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
                UiEvent::MassEffect3(me3_save_game) => {
                    save_game = SaveGame::MassEffect3(me3_save_game)
                }
                UiEvent::Error(err) => {
                    error_state.errors.push(err);
                    error_state.is_opened = true;
                }
            });

            let ui = Ui::new(imgui, &event_addr);
            ui.draw(&mut error_state, &mut save_game);
        },
        exit_fn,
    );
}

pub struct Ui<'a> {
    imgui: &'a ImguiUi<'a>,
    event_addr: Sender<MainEvent>,
    bg_count: AtomicUsize,
}

impl<'a> Ui<'a> {
    fn new(imgui: &'a ImguiUi<'a>, event_addr: &Sender<MainEvent>) -> Self {
        Self { imgui, event_addr: Sender::clone(event_addr), bg_count: AtomicUsize::new(0) }
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

        let mut colors = self.style_colors(save_game);
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

                PopupModal::new(im_str!("Error")).always_auto_resize(true).build(imgui, || {
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
                ChildWindow::new("mass_effect_3").size([0.0, 0.0]).build(self.imgui, || {
                    save_game.draw_raw_ui(self, "Mass Effect 3");
                });
            });
        });
    }

    // Edit boxes
    pub fn draw_edit_string(&self, ident: &str, value: &mut ImString) {
        self.draw_colored_bg(ident, || {
            self.imgui.input_text(&ImString::new(ident), value).build();
        });
    }

    pub fn draw_edit_bool(&self, ident: &str, value: &mut bool) {
        let imgui = self.imgui;

        self.draw_colored_bg(ident, || {
            let width = imgui.push_item_width(100.0);
            imgui.checkbox(&ImString::new(ident), value);
            width.pop(imgui);
        });
    }

    pub fn draw_edit_i32(&self, ident: &str, value: &mut i32) {
        let imgui = self.imgui;

        self.draw_colored_bg(ident, || {
            let width = imgui.push_item_width(100.0);
            InputInt::new(imgui, &ImString::new(ident), value).build();
            width.pop(imgui);
        });
    }

    pub fn draw_edit_f32(&self, ident: &str, value: &mut f32) {
        let imgui = self.imgui;

        self.draw_colored_bg(ident, || {
            let width = imgui.push_item_width(100.0);
            InputFloat::new(imgui, &ImString::new(ident), value).build();
            width.pop(imgui);
        });
    }

    pub fn draw_edit_enum(&self, ident: &str, current_item: &mut usize, items: &[&ImStr]) {
        let imgui = self.imgui;

        self.draw_colored_bg(ident, || {
            let width = imgui.push_item_width(200.0);
            ComboBox::new(&ImString::new(ident)).build_simple_string(imgui, current_item, items);
            width.pop(imgui);
        });
    }

    pub fn draw_edit_color(&self, ident: &str, color: &mut [f32; 4]) {
        let imgui = self.imgui;

        self.draw_colored_bg(ident, || {
            let width = imgui.push_item_width(200.0);
            ColorEdit::new(&ImString::new(ident), color).build(imgui);
            width.pop(imgui);
        });
    }

    // View widgets
    pub fn draw_struct<F>(&self, ident: &str, fields: F)
    where
        F: FnOnce(),
    {
        TreeNode::new(&ImString::new(ident)).build(self.imgui, fields);
    }

    pub fn draw_vec<T>(&self, ident: &str, list: &mut Vec<T>)
    where
        T: SaveData + Default,
    {
        let imgui = self.imgui;

        TreeNode::new(&ImString::new(ident)).build(imgui, || {
            if !list.is_empty() {
                // Item
                let mut remove = None;
                for (i, item) in list.iter_mut().enumerate() {
                    if imgui.small_button(&im_str!("remove##x-{}", i)) {
                        remove = Some(i);
                    }
                    imgui.same_line();
                    item.draw_raw_ui(self, &i.to_string());
                }

                // Remove
                if let Some(i) = remove {
                    list.remove(i);
                }

                // Add
                if imgui.button(&im_str!("add##add-{}", ident)) {
                    // Ça ouvre automatiquement le tree node de l'élément ajouté
                    TreeNode::new(&ImString::new(&list.len().to_string()))
                        .opened(true, Condition::Always)
                        .build(imgui, || {});

                    list.push(T::default());
                }
            } else {
                imgui.text("Empty");
            }
        });
    }

    pub fn draw_bitarray(&self, ident: &str, list: &mut Vec<bool>) {
        TreeNode::new(&ImString::new(ident)).build(self.imgui, || {
            if !list.is_empty() {
                let mut clipper = ListClipper::new(list.len() as i32).begin(self.imgui);
                while clipper.step() {
                    for i in clipper.display_start()..clipper.display_end() {
                        list[i as usize].draw_raw_ui(self, &i.to_string());
                    }
                }
            } else {
                self.imgui.text("Empty");
            }
        });
    }

    pub fn draw_indexmap<K, V>(&self, ident: &str, list: &mut IndexMap<K, V>)
    where
        K: SaveData + Eq + Hash + Default,
        V: SaveData + Default,
    {
        let imgui = self.imgui;

        TreeNode::new(&ImString::new(ident)).build(imgui, || {
            if !list.is_empty() {
                // Item
                let mut remove = None;
                for i in 0..list.len() {
                    if imgui.small_button(&im_str!("remove##x-{}", i)) {
                        remove = Some(i);
                    }
                    imgui.same_line();

                    TreeNode::new(&ImString::new(i.to_string())).build(imgui, || {
                        if let Some((key, value)) = list.get_index_mut(i) {
                            key.draw_raw_ui(self, "##k");
                            value.draw_raw_ui(self, "##v");
                        }
                    });
                }

                // Remove
                if let Some(i) = remove {
                    list.shift_remove_index(i);
                }

                // Add
                if imgui.button(&im_str!("add##add-{}", ident)) {
                    // Ça ouvre automatiquement le tree node de l'élément ajouté
                    TreeNode::new(&ImString::new(&list.len().to_string()))
                        .opened(true, Condition::Always)
                        .build(imgui, || {});

                    // FIXME: Ajout d'un nouvel élément si K = 0i32 déjà présent
                    list.entry(K::default()).or_default();
                }
            } else {
                imgui.text("Empty");
            }
        });
    }

    // Helpers
    fn draw_colored_bg<F>(&self, id: &str, inner: F)
    where
        F: FnOnce(),
    {
        let bg = self.bg_colors();
        ChildWindow::new(id).size([0.0, 19.0]).build(self.imgui, inner);
        bg.pop();
    }

    fn bg_colors(&self) -> ColorStackToken {
        let bg_dark = [0.1, 0.1, 0.1, 1.0];
        let bg_light = [0.15, 0.15, 0.15, 1.0];

        let bgs = [bg_dark, bg_light];

        let bg_count = self.bg_count.fetch_add(1, Ordering::AcqRel);
        self.imgui.push_style_color(StyleColor::ChildBg, bgs[bg_count % bgs.len()])
    }

    // Style
    fn style_colors(&self, save_game: &SaveGame) -> Vec<ColorStackToken> {
        let theme = match save_game {
            SaveGame::None | SaveGame::MassEffect3(_) => Theme {
                bg_color: [0.40, 0.0, 0.0, 1.0],
                color: [0.53, 0.0, 0.0, 1.0],
                active_color: [0.68, 0.0, 0.0, 1.0],
                hover_color: [0.86, 0.0, 0.0, 1.0],
            },
        };

        vec![
            self.imgui.push_style_color(StyleColor::TitleBgActive, theme.active_color),
            self.imgui.push_style_color(StyleColor::FrameBg, theme.bg_color),
            self.imgui.push_style_color(StyleColor::FrameBgActive, theme.active_color),
            self.imgui.push_style_color(StyleColor::FrameBgHovered, theme.hover_color),
            self.imgui.push_style_color(StyleColor::TextSelectedBg, theme.active_color),
            self.imgui.push_style_color(StyleColor::Button, theme.bg_color),
            self.imgui.push_style_color(StyleColor::ButtonActive, theme.active_color),
            self.imgui.push_style_color(StyleColor::ButtonHovered, theme.hover_color),
            self.imgui.push_style_color(StyleColor::Tab, theme.color),
            self.imgui.push_style_color(StyleColor::TabActive, theme.active_color),
            self.imgui.push_style_color(StyleColor::TabHovered, theme.hover_color),
            self.imgui.push_style_color(StyleColor::Header, theme.bg_color),
            self.imgui.push_style_color(StyleColor::HeaderActive, theme.active_color),
            self.imgui.push_style_color(StyleColor::HeaderHovered, theme.hover_color),
            self.imgui.push_style_color(StyleColor::CheckMark, [1.0, 1.0, 1.0, 1.0]),
        ]
    }
}

struct Theme {
    bg_color: [f32; 4],
    color: [f32; 4],
    active_color: [f32; 4],
    hover_color: [f32; 4],
}
