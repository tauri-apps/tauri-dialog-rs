use tauri_dialog::{DialogBuilder, DialogButtons, DialogSelection, DialogStyle};

fn main() {
  DialogBuilder::new()
    .message("Simple message boxes are very easy to create")
    .title("Basic Example")
    .build()
    .show();

  loop {
    let response = DialogBuilder::new()
      .message("Different buttons may be used, and the user's selection can be checked. Would you like to see this message again?")
      .title("Selection Example")
      .style(DialogStyle::Question)
      .buttons(DialogButtons::YesNo)
      .build()
      .show();
    if response != DialogSelection::Yes {
      break;
    }
  }
}
