use eframe::{egui, NativeOptions};

fn main() {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Editor Pandion",
        native_options,
        Box::new(|cc| Box::new(Editor::new(cc, "java".to_string(), Editor::default().code))),
    );
}

struct Editor {
    lang: String,
    code: String,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            lang: String::from("java"),
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
    fn new(cc: &eframe::CreationContext<'_>, lang: String, code: String) -> Self {
        Self { lang, code }
    }
}

impl eframe::App for Editor {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let ed = "java";
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        // frame.close()
                    }
                    if ui.button("Save").clicked() {
                        // frame.close()
                    }
                    if ui.button("Save as").clicked() {
                        // frame.close()
                    }
                    if ui.button("Quit").clicked() {
                        frame.close()
                    }
                })
            })
        });
        let label = "teste";
        egui::SidePanel::left("side_panel").show(ctx, move |ui| {
            ui.heading("Side Panel");

            let mut value: f32 = 0.;
            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut Editor::default().lang);
            });
            ui.add(egui::Slider::new(&mut value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                value += 1.0;
            }
        });
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.label(ed);
            });
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {}
}
