use anyhow::Error;
use flume::{Receiver, Sender};
use imgui::{Ui as ImguiUi, *};
use indexmap::IndexMap;
use std::{
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};
use tokio::runtime::Handle;
use wfd::DialogParams;

use crate::{event_handler::MainEvent, save_data::{SaveData,mass_effect_3::Me3SaveGame}};

mod support;

static NOTIFICATION_TIME: f64 = 1.5;

// States
#[derive(Default)]
struct ErrorState {
    errors: Vec<Error>,
    is_opened: bool,
}

#[derive(Default)]
struct NotificationState {
    string: ImString,
    close_time: f64,
}

#[derive(Default)]
struct State {
    errors: ErrorState,
    notification: Option<NotificationState>,
}

// Events
pub enum UiEvent {
    Error(Error),
    Notification(&'static str),
    OpenMassEffect3(Box<Me3SaveGame>),
}

enum SaveGame {
    None,
    MassEffect3(Box<Me3SaveGame>),
}

// UI
pub fn run(event_addr: Sender<MainEvent>, rx: Receiver<UiEvent>, handle: Handle) {
    let mut state = State::default();
    let mut save_game = SaveGame::None;

    // UI
    let system = support::init("Trilogy Save Editor", 1000.0, 700.0);
    system.main_loop(move |_, imgui| {
        handle.block_on(async {
            rx.try_iter().for_each(|ui_event| match ui_event {
                UiEvent::Error(err) => {
                    state.errors.errors.push(err);
                    state.errors.is_opened = true;
                }
                UiEvent::Notification(string) => {
                    state.notification = Some(NotificationState {
                        string: ImString::new(string),
                        close_time: imgui.time() + NOTIFICATION_TIME,
                    })
                }
                UiEvent::OpenMassEffect3(me3_save_game) => {
                    save_game = SaveGame::MassEffect3(me3_save_game)
                }
            });

            let ui = Ui::new(imgui, &event_addr);
            ui.draw(&mut state, &mut save_game).await;
        });
    });
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

    async fn draw(&self, state: &mut State, save_game: &mut SaveGame) {
        let imgui = self.imgui;

        // Main window
        let window = Window::new(im_str!("###main"))
            .size(imgui.io().display_size, Condition::Always)
            .position([0.0, 0.0], Condition::Always)
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .menu_bar(true)
            .collapsible(false);

        // Pop on drop
        let _colors = self
            .style_colors(match save_game {
                SaveGame::None => Theme::None,
                SaveGame::MassEffect3(_) => Theme::MassEffect3,
            })
            .await;
        let _style = imgui.push_style_var(StyleVar::WindowRounding(0.0));

        // Window
        if let Some(_t) = window.begin(imgui) {
            // Main menu bar
            if let Some(_t) = imgui.begin_menu_bar() {
                if imgui.button(im_str!("Open")) {
                    self.open_save().await;
                }
                if imgui.button(im_str!("Save")) {
                    self.save_save(save_game).await;
                }
            }

            // Error popup
            {
                let ErrorState { errors, is_opened } = &mut state.errors;
                if *is_opened {
                    imgui.open_popup(im_str!("Error"));
                }

                if let Some(_t) = PopupModal::new(im_str!("Error###error"))
                    .always_auto_resize(true)
                    .begin_popup(imgui)
                {
                    errors.iter().for_each(|error| {
                        imgui.text(error.to_string());
                    });
                    imgui.separator();

                    if imgui.button_with_size(im_str!("OK"), [70.0, 0.0]) {
                        *is_opened = false;
                        errors.clear();
                        imgui.close_current_popup();
                    }
                }
            }

            // Notification
            self.draw_nofification_overlay(&mut state.notification);

            match save_game {
                SaveGame::None => imgui.text(im_str!("Rien ici")),
                SaveGame::MassEffect3(save_game) => self.draw_mass_effect_3(save_game).await,
            };
        }
    }

    fn draw_nofification_overlay(&self, notification: &mut Option<NotificationState>) {
        if let Some(NotificationState { string, close_time }) = notification {
            let imgui = self.imgui;
            let time = imgui.time();

            let _style = imgui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 0.3]);
            let window = Window::new(im_str!("###notification"))
                .position([imgui.io().display_size[0] / 2.0, 50.0], Condition::Always)
                .title_bar(false)
                .resizable(false)
                .movable(false)
                .always_auto_resize(true);

            if let Some(_t) = window.begin(imgui) {
                imgui.text(&string);

                let remaining = (*close_time - time) / NOTIFICATION_TIME;
                ProgressBar::new(remaining as f32)
                    .overlay_text(&ImString::new("time_bar"))
                    .size([-0.0001, 2.0])
                    .build(imgui);
            }

            if *close_time < time {
                *notification = None;
            }
        }
    }

    async fn draw_mass_effect_3(&self, save_game: &mut Me3SaveGame) {
        let imgui = self.imgui;

        // Tabs
        if let Some(_t) = TabBar::new(im_str!("main-tabs")).begin(imgui) {
            if let Some(_t) = TabItem::new(im_str!("Raw")).begin(imgui) {
                if let Some(_t) = ChildWindow::new("mass_effect_3").size([0.0, 0.0]).begin(imgui) {
                    save_game.draw_raw_ui(self, "Mass Effect 3");
                }
            }
        }
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
            let _width = imgui.push_item_width(100.0);
            imgui.checkbox(&ImString::new(ident), value);
        });
    }

    pub fn draw_edit_i32(&self, ident: &str, value: &mut i32) {
        let imgui = self.imgui;

        self.draw_colored_bg(ident, || {
            let _width = imgui.push_item_width(100.0);
            InputInt::new(imgui, &ImString::new(ident), value).build();
        });
    }

    pub fn draw_edit_f32(&self, ident: &str, value: &mut f32) {
        let imgui = self.imgui;

        self.draw_colored_bg(ident, || {
            let _width = imgui.push_item_width(100.0);
            InputFloat::new(imgui, &ImString::new(ident), value).build();
        });
    }

    pub fn draw_edit_enum(&self, ident: &str, current_item: &mut usize, items: &[&ImStr]) {
        let imgui = self.imgui;

        self.draw_colored_bg(ident, || {
            let _width = imgui.push_item_width(200.0);
            ComboBox::new(&ImString::new(ident)).build_simple_string(imgui, current_item, items);
        });
    }

    pub fn draw_edit_color(&self, ident: &str, color: &mut [f32; 4]) {
        let imgui = self.imgui;

        self.draw_colored_bg(ident, || {
            let _width = imgui.push_item_width(200.0);
            ColorEdit::new(&ImString::new(ident), color).build(imgui);
        });
    }

    // View widgets
    pub fn draw_struct<F>(&self, ident: &str, fields: F)
    where
        F: FnOnce(),
    {
        if let Some(_t) = TreeNode::new(&ImString::new(ident)).push(self.imgui) {
            fields();
        }
    }

    pub fn draw_vec<T>(&self, ident: &str, list: &mut Vec<T>)
    where
        T: SaveData + Default,
    {
        let imgui = self.imgui;

        if let Some(_t) = TreeNode::new(&ImString::new(ident)).push(imgui) {
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
        }
    }

    pub fn draw_bitarray(&self, ident: &str, list: &mut Vec<bool>) {
        if let Some(_t) = TreeNode::new(&ImString::new(ident)).push(self.imgui) {
            if !list.is_empty() {
                let mut clipper = ListClipper::new(list.len() as i32).begin(self.imgui);
                while clipper.step() {
                    for i in clipper.display_start()..clipper.display_end() {
                        list[i as usize].draw_raw_ui(self, &i.to_string());
                    }
                }
            }
        }
    }

    pub fn draw_indexmap<K, V>(&self, ident: &str, list: &mut IndexMap<K, V>)
    where
        K: SaveData + Eq + Hash + Default,
        V: SaveData + Default,
    {
        let imgui = self.imgui;

        if let Some(_t) = TreeNode::new(&ImString::new(ident)).push(imgui) {
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
        }
    }

    // Helpers
    fn draw_colored_bg<F>(&self, id: &str, inner: F)
    where
        F: FnOnce(),
    {
        let _bg = self.bg_colors();
        if let Some(_t) = ChildWindow::new(id).size([0.0, 19.0]).begin(self.imgui) {
            inner();
        }
    }

    fn bg_colors(&self) -> ColorStackToken<'a> {
        let bg_dark = [0.1, 0.1, 0.1, 1.0];
        let bg_light = [0.15, 0.15, 0.15, 1.0];

        let bgs = [bg_dark, bg_light];

        let bg_count = self.bg_count.fetch_add(1, Ordering::AcqRel);
        self.imgui.push_style_color(StyleColor::ChildBg, bgs[bg_count % bgs.len()])
    }

    // Style
    async fn style_colors(&self, game_theme: Theme) -> [ColorStackToken<'a>; 17] {
        let theme = match game_theme {
            Theme::None | Theme::MassEffect3 => ColorTheme {
                bg_color: [0.40, 0.0, 0.0, 1.0],
                color: [0.53, 0.0, 0.0, 1.0],
                active_color: [0.68, 0.0, 0.0, 1.0],
                hover_color: [0.86, 0.0, 0.0, 1.0],
            },
        };

        [
            self.imgui.push_style_color(StyleColor::WindowBg, [0.0, 0.0, 0.0, 1.0]),
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
            self.imgui.push_style_color(StyleColor::PlotHistogram, [1.0, 1.0, 1.0, 1.0]),
        ]
    }

    // Actions
    async fn open_save(&self) {
        let result = wfd::open_dialog(DialogParams {
            file_types: vec![("Mass Effect Save", "*.MassEffectSave;*.pcsav")],
            ..Default::default()
        });

        if let Ok(result) = result {
            let _ =
                self.event_addr.send_async(MainEvent::OpenSave(result.selected_file_path)).await;
        }
    }

    async fn save_save(&self, save_game: &SaveGame) {
        let (save_game, default_ext) = match save_game {
            SaveGame::None => return,
            SaveGame::MassEffect3(save_game) => (save_game, "pcsav"),
        };

        let result = wfd::save_dialog(DialogParams {
            default_extension: default_ext,
            file_types: vec![("Mass Effect Save", "*.MassEffectSave;*.pcsav")],
            ..Default::default()
        });

        if let Ok(result) = result {
            let _ = self
                .event_addr
                .send_async(MainEvent::SaveSave((result.selected_file_path, save_game.clone())))
                .await;
        }
    }
}

enum Theme {
    None,
    MassEffect3,
}

struct ColorTheme {
    bg_color: [f32; 4],
    color: [f32; 4],
    active_color: [f32; 4],
    hover_color: [f32; 4],
}
