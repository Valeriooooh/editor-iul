mod file;
mod syntax_highlighting;
use eframe::egui;
use rfd;

use self::file::file_save;

pub struct Settings {
    pub font_size: u32,
    pub theme: String,
}
pub struct Editor {
    pub lang: String,
    pub picked_path: String,
    pub left_panel: bool,
    pub code: String,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            lang: String::from("java"),
            left_panel: false,
            picked_path: "untitled.txt".to_string(),
            code: String::from(
                "public class Test{
    public static void main(String[] args){
        System.out.println(\"hello world\")
    }
}",
            ),
        }
    }
}

impl Editor {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for Editor {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        match rfd::FileDialog::new().pick_file() {
                            Some(path) => {
                                let path = Some(path.display().to_string());
                                match path {
                                    Some(a) => {
                                        self.picked_path = a;
                                        match file::file_read(self.picked_path.clone()) {
                                            Ok(a) => self.code = a,
                                            Err(e) => println!("Error: {:?}", e),
                                        };
                                    }
                                    None => {}
                                }
                            }
                            _ => (),
                        }
                    }
                    if ui.button("Save").clicked() {
                        if self.picked_path == Editor::default().picked_path {
                            match rfd::FileDialog::new().save_file() {
                                Some(path) => {
                                    let path = Some(path.display().to_string());
                                    match path {
                                        Some(a) => {
                                            self.picked_path = a;
                                            match file_save(
                                                self.picked_path.clone(),
                                                self.code.clone(),
                                            ) {
                                                Ok(_) => {}
                                                Err(e) => println!("Error: {:?}", e),
                                            };
                                        }
                                        None => {}
                                    }
                                }
                                _ => (),
                            }
                        } else {
                            match file_save(self.picked_path.clone(), self.code.clone()) {
                                Ok(_) => {}
                                Err(e) => println!("Error: {:?}", e),
                            };
                        }
                    }
                    if ui.button("Save as").clicked() {
                        match rfd::FileDialog::new().save_file() {
                            Some(path) => {
                                let path = Some(path.display().to_string());
                                match path {
                                    Some(a) => {
                                        self.picked_path = a;
                                        match file_save(self.picked_path.clone(), self.code.clone())
                                        {
                                            Ok(_) => {}
                                            Err(e) => println!("Error: {:?}", e),
                                        };
                                    }
                                    None => {}
                                }
                            }
                            _ => (),
                        }
                        // frame.close()
                    }
                    if ui.button("Quit").clicked() {
                        frame.close()
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        // frame.close()
                    }
                    if ui.button("Redo").clicked() {
                        // frame.close()
                    }
                    if ui.button("Preferences").clicked() {
                        // frame.close()
                    }
                });
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // let mut theme = syntax_highlighting::CodeTheme::from_memory(u);
            // ui.collapsing("Theme", |ui| {
            //     ui.group(|ui| {
            //         theme.ui(ui);
            //         theme.clone().store_in_memory(ui);
            //     });
            // });
            if self.left_panel {
                egui::SidePanel::left("side_panel").show_inside(ui, |ui| {
                    ui.heading("Project Tree\t\t");
                    ui.horizontal(|ui| {});
                });
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                let theme = syntax_highlighting::CodeTheme::dark();
                let codeclone = &self.code.to_owned();
                let mut layouter = |ui: &egui::Ui, _string: &str, _wrap_width: f32| {
                    let mut layout_job =
                        syntax_highlighting::highlight(ui.ctx(), &theme, codeclone, &self.lang);
                    layout_job.wrap.max_width = f32::INFINITY;
                    ui.fonts(|f| f.layout_job(layout_job))
                };

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
                    self.left_panel = !self.left_panel;
                }

                ui.label(format!("Lang: {}", self.lang));
                ui.label(format!("File: {}", self.picked_path));
            });
        });
    }
}

fn my_memoized_highlighter(s: &str) -> egui::text::LayoutJob {
    Default::default()
}
