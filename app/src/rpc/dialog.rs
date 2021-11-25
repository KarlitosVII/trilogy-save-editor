use std::ffi::OsStr;
use std::path::PathBuf;

use wry::application::window::Window;

use super::command::DialogParams;

pub fn open_save(window: &Window, last_dir: bool) -> Option<PathBuf> {
    let mut dialog = rfd::FileDialog::new()
        .add_filter("Mass Effect Trilogy Save", &["pcsav", "xbsav", "ps4sav", "MassEffectSave"])
        .add_filter("All Files", &["*"]);

    dialog = with_parent(dialog, window);

    if !last_dir {
        if let Some(bioware_dir) = bioware_dir() {
            dialog = dialog.set_directory(bioware_dir);
        }
    }

    dialog.pick_file()
}

pub fn save_save(window: &Window, params: DialogParams) -> Option<PathBuf> {
    let DialogParams { path, filters } = params;

    let file_name = path.file_name().map(OsStr::to_string_lossy).unwrap_or_default();

    let mut dialog = rfd::FileDialog::new().set_file_name(&file_name);
    dialog = with_parent(dialog, window);

    for (filter, extensions) in filters {
        let extensions: Vec<&str> = extensions.iter().map(String::as_str).collect();
        dialog = dialog.add_filter(&filter, &extensions);
    }

    let directory = path
        .parent()
        .and_then(|parent| parent.is_dir().then(|| parent.to_owned()))
        .or_else(bioware_dir);

    if let Some(directory) = directory {
        dialog = dialog.set_directory(directory);
    }

    dialog.save_file()
}

pub fn import_head_morph(window: &Window) -> Option<PathBuf> {
    let dialog = rfd::FileDialog::new()
        .add_filter("Head Morph", &["ron", "me2headmorph", "me3headmorph"])
        .add_filter("All Files", &["*"]);

    with_parent(dialog, window).pick_file()
}

pub fn export_head_morph(window: &Window) -> Option<PathBuf> {
    let dialog = rfd::FileDialog::new().add_filter("Head Morph", &["ron"]);
    with_parent(dialog, window).save_file()
}

#[cfg(target_os = "windows")]
fn bioware_dir() -> Option<PathBuf> {
    dirs::document_dir().and_then(|mut path| {
        path.push("BioWare\\");
        path.is_dir().then(|| path)
    })
}

// FIXME: Find some nicer way of finding where the game saves are.
// Currently, this should be universal for everyone who has their
// Mass Effect games installed in the default steam library, in
// the user's home directory.
#[cfg(target_os = "linux")]
fn bioware_dir() -> Option<PathBuf> {
    dirs::home_dir().and_then(|mut path| {
        path.push(".steam/root/steamapps/compatdata/1328670/pfx/drive_c/users/steamuser/My Documents/BioWare/");
        path.is_dir().then(|| path)
    })
}

#[cfg(all(not(target_os = "linux"), not(target_os = "windows")))]
fn bioware_dir() -> Option<PathBuf> {
    None
}

// FIXME: Remove this and set directly `set_parent` when `tao` will implement `raw_window_handle` for linux
#[cfg(not(target_os = "linux"))]
fn with_parent(dialog: rfd::FileDialog, window: &Window) -> rfd::FileDialog {
    dialog.set_parent(window)
}

#[cfg(target_os = "linux")]
fn with_parent(dialog: rfd::FileDialog, _: &Window) -> rfd::FileDialog {
    dialog
}
