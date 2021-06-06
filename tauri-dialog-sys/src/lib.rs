//! This crate is no longer maintained. Tauri now uses [rfd](https://github.com/PolyMeilex/rfd).

use std::ffi::CString;

#[repr(C)]
pub enum DialogStyle {
  Info,
  Warning,
  Error,
  Question,
}

#[repr(C)]
pub enum DialogButtons {
  Ok,
  OkCancel,
  YesNo,
  Quit,
}

#[repr(C)]
#[derive(PartialEq)]
pub enum DialogSelection {
  Ok,
  Cancel,
  Yes,
  No,
  Quit,
  None,
  Error,
}

mod ffi {
  use crate::{DialogButtons, DialogSelection, DialogStyle};
  use std::os::raw::*;

  extern "C" {
    pub fn boxerShow(
      message: *const c_char,
      title: *const c_char,
      style: DialogStyle,
      buttons: DialogButtons,
    ) -> DialogSelection;
  }
}

pub fn show_dialog(
  message: &str,
  title: &str,
  style: DialogStyle,
  buttons: DialogButtons,
) -> DialogSelection {
  let c_message = CString::new(message).expect("No nul bytes in parameter message");
  let c_title = CString::new(title).expect("No nul bytes in parameter title");
  unsafe { ffi::boxerShow(c_message.as_ptr(), c_title.as_ptr(), style, buttons) }
}
