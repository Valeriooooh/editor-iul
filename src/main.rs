mod editor;
use editor::*;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "editor-iul",
        native_options,
        Box::new(|cc| Box::new(Editor::new(cc))),
    );
}
