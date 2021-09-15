use std::ffi::OsStr;
use std::path::PathBuf;

use anyhow::{Error, Result};
use wry::application::window::Window;

use super::command::DialogParams;

pub fn open_save(window: &Window) -> Result<Option<PathBuf>> {
    native_dialog::FileDialog::new()
        .set_owner(window)
        .set_location(&bioware_dir())
        .add_filter("Mass Effect Trilogy Save", &["pcsav", "xbsav", "ps4sav", "MassEffectSave"])
        .add_filter("All Files", &["*"])
        .show_open_single_file()
        .map_err(Error::from)
}

pub fn save_save(window: &Window, params: DialogParams) -> Result<Option<PathBuf>> {
    let DialogParams { path, filters } = params;

    let directory = path.parent().map(ToOwned::to_owned).unwrap_or_else(bioware_dir);
    let file_name = path.file_name().map(OsStr::to_string_lossy).unwrap_or_default();

    let mut dialog = native_dialog::FileDialog::new()
        .set_owner(window)
        .set_location(&directory)
        .set_filename(&file_name);

    let filters: Vec<(&str, Vec<&str>)> =
        filters.iter().map(|(f, e)| (f.as_str(), e.iter().map(String::as_str).collect())).collect();
    for (filter, extensions) in &filters {
        dialog = dialog.add_filter(filter, extensions);
    }
    dialog.show_save_single_file().map_err(Error::from)
}

pub fn import_head_morph(window: &Window) -> Result<Option<PathBuf>> {
    native_dialog::FileDialog::new()
        .set_owner(window)
        .add_filter("Head Morph", &["ron", "me2headmorph", "me3headmorph"])
        .add_filter("All Files", &["*"])
        .show_open_single_file()
        .map_err(Error::from)
}

pub fn export_head_morph(window: &Window) -> Result<Option<PathBuf>> {
    native_dialog::FileDialog::new()
        .set_owner(window)
        .add_filter("Head Morph", &["ron"])
        .show_save_single_file()
        .map_err(Error::from)
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
