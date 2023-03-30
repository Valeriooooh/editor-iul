mod file;
mod syntax_highlighting;
use std::fs;

use eframe::{egui, epaint::FontId};

use self::{
    file::{file_write, scan_dir},
    syntax_highlighting::CodeTheme,
};

pub struct Settings {
    pub font_size: f32,
    pub theme: CodeTheme,
}
pub struct Editor {
    pub lang: String,
    pub picked_path: String,
    pub left_panel: bool,
    pub settings_panel: bool,
    pub code: String,
    pub saved: bool,
    pub settings: Settings,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            lang: String::from("txt"),
            left_panel: false,
            settings_panel: false,
            saved: false,
            picked_path: "untitled.txt".to_string(),
            code: String::from(""),
            settings: Settings {
                font_size: 15.,
                theme: CodeTheme::dark(),
            },
        }
    }
}

impl Editor {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

// macro_rules! menu_button {
//     ($text:expr,($($code:tt)) => {
//         if ui.button($text).clicked() {
//             $code
//             ui.close_menu();
//         }

//     };
//     (_) => {};
// }

impl eframe::App for Editor {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        file::file_picker(self);
                        ui.close_menu();
                    }
                    if ui.button("Save").clicked() {
                        if self.picked_path == Editor::default().picked_path {
                            file::file_save(self)
                        } else {
                            match file_write(self.picked_path.clone(), self.code.clone()) {
                                Ok(_) => {}
                                Err(e) => println!("Error: {:?}", e),
                            };
                        }
                        ui.close_menu();
                    }
                    if ui.button("Save as").clicked() {
                        file::file_save(self);
                        ui.close_menu();
                    }
                    if ui.button("Quit").clicked() {
                        frame.close()
                    }
                });

                ui.separator();
                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {}
                    if ui.button("Redo").clicked() {}
                    if ui.button("Preferences").clicked() {
                        self.left_panel = true;
                        self.settings_panel = true;
                    }
                });
                ui.separator();
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {}
                    if ui.button("Licence").clicked() {
                        egui::Window::new("Licence").show(ctx, |ui| {
                            ui.label("teste");
                        });
                    }
                });
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.left_panel {
                // TODO fazer função que retorna a arvore de ficheiros para saber quais são pastas e ficheiros:
                // possivel implementação fic(stream) json (usar serde)
                egui::SidePanel::left("side_panel").show_inside(ui, |ui| {
                    if !(self.settings_panel) {
                        ui.heading("Project Tree\t\t\t");
                        ui.horizontal(|ui| {
                            ui.collapsing("Files", |ui| {
                                scan_dir(".".to_string(), ui, self);
                            });
                        });
                    } else {
                        ui.heading("Preferences\t\t\t");
                        ui.group(|ui| {});
                    }
                });
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                let theme = syntax_highlighting::CodeTheme::dark();
                let mut layouter = |ui: &egui::Ui, _string: &str, _wrap_width: f32| {
                    let mut layout_job =
                        syntax_highlighting::highlight(ui.ctx(), &theme, _string, &self.lang);
                    layout_job.wrap.max_width = _wrap_width;
                    ui.fonts(|f| f.layout_job(layout_job))
                };
                // let mut style = (*ctx.style()).clone();
                // style.text_styles = [
                //     (egui::TextStyle::Heading, FontId::new(30.0, Proportional)),
                //     (egui::TextStyle::Body, FontId::new(18.0, Proportional)),
                //     (egui::TextStyle::Monospace, FontId::new(14.0, Proportional)),
                //     (egui::TextStyle::Button, FontId::new(14.0, Proportional)),
                //     (egui::TextStyle::Small, FontId::new(10.0, Proportional)),
                // ]
                // .into();
                // ctx.set_style(style);
                ui.add(
                    egui::TextEdit::multiline(&mut self.code)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .lock_focus(true)
                        .desired_rows(70)
                        .desired_width(f32::INFINITY)
                        .layouter(&mut layouter)
                        .id("CodeEditor".into()),
                );
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("show directory").clicked() {
                    if !(self.settings_panel) {
                        self.left_panel = !self.left_panel;
                    }
                    self.settings_panel = false;
                }

                ui.separator();
                ui.label(format!("File: {}", self.picked_path));
                ui.separator();
                ui.label(format!("Lang: {}", self.lang));
            });
        });
    }
}

fn my_memoized_highlighter(s: &str) -> egui::text::LayoutJob {
    Default::default()
}
