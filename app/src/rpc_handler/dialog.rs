use std::ffi::OsStr;
use std::path::PathBuf;

use wry::application::window::Window;

use super::command::DialogParams;

pub fn open_save(window: &Window) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_parent(window)
        .set_directory(bioware_dir())
        .add_filter("Mass Effect Trilogy Save", &["pcsav", "xbsav", "ps4sav", "MassEffectSave"])
        .add_filter("All Files", &["*"])
        .pick_file()
}

pub fn save_save(window: &Window, params: DialogParams) -> Option<PathBuf> {
    let DialogParams { path, filters } = params;

    let directory = path
        .parent()
        .and_then(|parent| parent.is_dir().then(|| parent.to_owned()))
        .unwrap_or_else(bioware_dir);
    let file_name = path.file_name().map(OsStr::to_string_lossy).unwrap_or_default();

    let mut dialog = rfd::FileDialog::new()
        .set_parent(window)
        .set_directory(directory)
        .set_file_name(&file_name);

    for (filter, extensions) in filters {
        let extension: Vec<&str> = extensions.iter().map(String::as_str).collect();
        dialog = dialog.add_filter(&filter, &extension);
    }
    dialog.save_file()
}

pub fn import_head_morph(window: &Window) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_parent(window)
        .add_filter("Head Morph", &["ron"])
        .add_filter("All Files", &["*"])
        .pick_file()
}

pub fn export_head_morph(window: &Window) -> Option<PathBuf> {
    rfd::FileDialog::new().set_parent(window).add_filter("Head Morph", &["ron"]).save_file()
}

#[cfg(target_os = "windows")]
fn bioware_dir() -> PathBuf {
    match dirs::document_dir() {
        Some(path) => path.join("BioWare\\"),
        None => PathBuf::default(),
    }
}

// FIXME: Find some nicer way of finding where the game saves are.
// Currently, this should be universal for everyone who has their
// Mass Effect games installed in the default steam library, in
// the user's home directory.
#[cfg(target_os = "linux")]
fn bioware_dir() -> PathBuf {
    match dirs::home_dir() {
        Some(path) => {
            path.join(".steam/root/steamapps/compatdata/1328670/pfx/drive_c/users/steamuser/My Documents/BioWare/")
        }
        None => PathBuf::default(),
    }
}

#[cfg(all(not(target_os = "linux"), not(target_os = "windows")))]
fn bioware_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_default()
}
