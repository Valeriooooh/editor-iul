mod editor;
use editor::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Editor Pandion",
        native_options,
        Box::new(|cc| Box::new(Editor::new(cc))),
    );
}
