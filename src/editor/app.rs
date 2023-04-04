use eframe::egui::{self, Visuals};
// use egui_keybinds::KeyBindWidget;

use super::{
    file::{self, file_write, scan_dir},
    shortcuts, syntax_highlighting, Editor,
};

impl eframe::App for Editor {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.startup(ctx);
        if !self.settings.dark_mode {
            ctx.set_visuals(Visuals::light())
        } else {
            ctx.set_visuals(Visuals::dark())
        }

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("show directory").clicked() {
                    if !(self.settings_panel) {
                        self.left_panel = !self.left_panel;
                    }
                    self.settings_panel = false;
                }

                ui.separator();
                match self.saved {
                    true => {
                        ui.label(format!("File: {}", self.picked_path));
                    }
                    false => {
                        ui.label(format!("File: {}*", self.picked_path));
                    }
                }
                ui.separator();
                ui.label(format!("Lang: {}", self.lang));
            });
        });
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        file::file_picker(self);
                        ui.close_menu();
                    }
                    if ui.button("Open Project").clicked() {
                        file::project_picker(self);
                        ui.close_menu();
                    }
                    if ui.button("Save").clicked() {
                        if self.picked_path == Editor::default().picked_path {
                            file::file_save(self)
                        } else {
                            match file_write(self.picked_path.clone(), self.code.clone()) {
                                Ok(_) => self.saved = true,
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
                egui::SidePanel::left("side_panel")
                    .max_width(250.)
                    .show_inside(ui, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            if !(self.settings_panel) {
                                ui.heading("Project Tree\t\t\t");
                                ui.vertical(|ui| match self.project_path.clone() {
                                    // ui.collapsing("Files", |ui|  {
                                    Some(a) => scan_dir(a.to_string(), ui, self),
                                    None => scan_dir(".".to_string(), ui, self),
                                    // });
                                });
                            } else {
                                ui.heading("Preferences\t\t\t");
                                ui.vertical(|ui| {
                                    ui.add(egui::Checkbox::new(
                                        &mut self.settings.dark_mode,
                                        "Dark Mode",
                                    ));
                                });
                            }
                        });
                    });
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut theme = syntax_highlighting::CodeTheme::dark();
                if !self.settings.dark_mode {
                    theme = syntax_highlighting::CodeTheme::light();
                }
                let mut layouter = |ui: &egui::Ui, _string: &str, _wrap_width: f32| {
                    let mut layout_job =
                        syntax_highlighting::highlight(ui.ctx(), &theme, _string, &self.lang);
                    layout_job.wrap.max_width = _wrap_width;
                    ui.fonts(|f| f.layout_job(layout_job))
                };
                if ui
                    .add(
                        egui::TextEdit::multiline(&mut self.code)
                            .font(egui::TextStyle::Monospace)
                            .code_editor()
                            .lock_focus(true)
                            .desired_rows(48)
                            .desired_width(f32::INFINITY)
                            .layouter(&mut layouter)
                            .id("CodeEditor".into()),
                    )
                    .changed()
                {
                    self.saved = false;
                }
                shortcuts::set_default_shortcuts(ui, self, frame);
            });
        });
    }
}
