use std::cell::RefCell;
use std::rc::Rc;

use anyhow::{Context, Result};
use gloo::{
    events::{EventListener, EventListenerOptions},
    file::{self, callbacks::FileReader, FileList},
};
use gloo_utils as utils;
use wasm_bindgen::JsCast;
use yew::prelude::*;

pub struct DropHandler {
    _drag_over_listener: EventListener,
    _drop_listener: EventListener,
    _file_reader: Rc<RefCell<Option<FileReader>>>,
}

impl DropHandler {
    pub fn new(ondrop: Callback<Result<(String, Vec<u8>)>>) -> Self {
        let document = utils::document();
        let options = EventListenerOptions::enable_prevent_default();

        let drag_over_listener =
            EventListener::new_with_options(&document, "dragover", options, |event| {
                // To allow drop
                event.prevent_default();
            });

        let file_reader = Rc::new(RefCell::new(None));
        let drop_listener = {
            let file_reader = Rc::clone(&file_reader);
            EventListener::new_with_options(&document, "drop", options, move |event| {
                let ondrop = ondrop.clone();
                let file_reader = Rc::clone(&file_reader);
                let handle_drop = move || {
                    event.prevent_default();

                    let drag_event = event.dyn_ref::<DragEvent>()?;
                    let data = drag_event.data_transfer()?;
                    let file_list: FileList = data.files()?.into();

                    if let Some(file) = file_list.first() {
                        let file_name = file.name();
                        let reader = {
                            let file_reader = Rc::clone(&file_reader);
                            file::callbacks::read_as_bytes(file, move |read_file| {
                                let result = read_file
                                    .map(|bytes| (file_name, bytes))
                                    .context("Failed to open the save");
                                ondrop.emit(result);

                                // Drop the reader when finished
                                *(*file_reader).borrow_mut() = None;
                            })
                        };
                        // Keep the reader alive
                        *(*file_reader).borrow_mut() = Some(reader);
                    }
                    Some(())
                };
                let _ = handle_drop();
            })
        };
        DropHandler {
            _drag_over_listener: drag_over_listener,
            _drop_listener: drop_listener,
            _file_reader: file_reader,
        }
    }
}
