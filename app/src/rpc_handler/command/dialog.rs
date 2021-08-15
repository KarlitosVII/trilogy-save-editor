use std::path::PathBuf;

use wry::application::window::Window;

pub fn open_save(window: &Window) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_parent(window)
        .set_directory(document_dir())
        .add_filter("Mass Effect Trilogy Save", &["pcsav", "ps4sav", "MassEffectSave"])
        .add_filter("All Files", &["*"])
        .pick_file()
}

pub fn save_save(window: &Window, path: PathBuf) -> Option<PathBuf> {
    let directory = path.parent().map(ToOwned::to_owned).unwrap_or_default();
    let file_name =
        path.file_name().map(ToOwned::to_owned).unwrap_or_default().to_string_lossy().into_owned();

    // TODO: Filter by game
    rfd::FileDialog::new()
        .set_parent(window)
        .set_directory(directory)
        .set_file_name(&file_name)
        .add_filter("Mass Effect Trilogy Save", &["pcsav", "ps4sav", "MassEffectSave"])
        .add_filter("All Files", &["*"])
        .save_file()
}

pub fn import_head_morph(window: &Window) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_parent(window)
        .add_filter("Head Morph", &["ron"])
        .add_filter("All Files", &["*"])
        .pick_file()
}

pub fn export_head_morph(window: &Window) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_parent(window)
        .add_filter("Head Morph", &["ron"])
        .add_filter("All Files", &["*"])
        .save_file()
}

#[cfg(target_os = "windows")]
fn document_dir() -> PathBuf {
    match dirs::document_dir() {
        Some(mut path) => {
            path.push("BioWare\\");
            path
        }
        None => PathBuf::default(),
    }
}

// FIXME: Find some nicer way of finding where the game saves are.
// Currently, this should be universal for everyone who has their
// Mass Effect games installed in the default steam library, in
// the user's home directory.
#[cfg(target_os = "linux")]
fn document_dir() -> PathBuf {
    match dirs::home_dir() {
        Some(mut path) => {
            path.push(".steam/root/steamapps/compatdata/1328670/pfx/drive_c/users/steamuser/My Documents/BioWare/");
            path
        }
        None => PathBuf::default(),
    }
}

#[cfg(all(not(target_os = "linux"), not(target_os = "windows")))]
fn document_dir() -> PathBuf {
    PathBuf::default()
}
