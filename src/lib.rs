//! This crate is no longer maintained. Tauri now uses [rfd](https://github.com/PolyMeilex/rfd).

use tauri_dialog_sys::show_dialog;
pub use tauri_dialog_sys::{DialogButtons, DialogSelection, DialogStyle};

#[derive(Default)]
pub struct DialogBuilder<'a> {
    message: Option<&'a str>,
    title: Option<&'a str>,
    style: Option<DialogStyle>,
    buttons: Option<DialogButtons>,
}

impl<'a> DialogBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn message(mut self, message: &'a str) -> Self {
        self.message = Some(message);
        self
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn style(mut self, style: DialogStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn buttons(mut self, buttons: DialogButtons) -> Self {
        self.buttons = Some(buttons);
        self
    }

    pub fn build(self) -> Dialog<'a> {
        Dialog {
            message: self.message.unwrap_or(""),
            title: self.title.unwrap_or(""),
            style: self.style.unwrap_or(DialogStyle::Info),
            buttons: self.buttons.unwrap_or(DialogButtons::Ok),
        }
    }
}

pub struct Dialog<'a> {
    message: &'a str,
    title: &'a str,
    style: DialogStyle,
    buttons: DialogButtons,
}

impl<'a> Dialog<'a> {
    pub fn show(self) -> DialogSelection {
        show_dialog(self.message, self.title, self.style, self.buttons)
    }
}
