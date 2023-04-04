use eframe::egui;

use crate::editor::file;

use super::Editor;
#[macro_export]
macro_rules! keybind {
    ($ui:tt, ctrl, $key:tt, $($code:tt),*) => {
        let short = egui::KeyboardShortcut {
            modifiers: egui::Modifiers {
                ctrl: true,
                ..Default::default()
            },
            key: egui::Key::$key,
        };
        if $ui.input_mut(|i| i.consume_shortcut(&short)) {
            $($code),*
        }
    };

    ($ui:tt, alt, $key:tt, $($code:tt),*) => {
        let short = egui::KeyboardShortcut {
            modifiers: egui::Modifiers {
                alt: true,
                ..Default::default()
            },
            key: egui::Key::$key,
        };
        if $ui.input_mut(|i| i.consume_shortcut(&short)) {
            $($code),*
        }
    };
}

pub fn set_default_shortcuts(ui: &mut egui::Ui, ed: &mut Editor, frame: &mut eframe::Frame) {
    keybind!(ui, ctrl, S, {
        if ed.picked_path == Editor::default().picked_path {
            file::file_save(ed)
        } else {
            match file::file_write(ed.picked_path.clone(), ed.code.clone()) {
                Ok(_) => ed.saved = true,
                Err(e) => println!("Error: {:?}", e),
            };
        }
    });

    keybind!(ui, ctrl, O, {
        file::file_picker(ed);
    });

    keybind!(ui, alt, Q, {
        frame.close();
    });

    keybind!(ui, alt, P, {
        if !(ed.settings_panel) {
            ed.left_panel = !ed.left_panel;
        }
        ed.settings_panel = false;
    });

    keybind!(ui, ctrl, P, {
        file::project_picker(ed);
    });
}
