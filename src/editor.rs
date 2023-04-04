mod app;
mod file;
mod shortcuts;
mod syntax_highlighting;
use std::path::PathBuf;

use eframe::egui;

use self::syntax_highlighting::CodeTheme;

pub struct Settings {
    pub is_startup: bool,
    pub font_size: f32,
    pub dark_mode: bool,
    pub theme: CodeTheme,
}
pub struct Editor {
    pub lang: String,
    pub picked_path: String,
    pub project_path: Option<String>,
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
            project_path: None,
            code: String::from(""),
            settings: Settings {
                is_startup: true,
                font_size: 15.,
                theme: CodeTheme::dark(),
                dark_mode: true,
            },
        }
    }
}

#[allow(dead_code, unused_variables)]
impl Editor {
    fn startup(&mut self, ctx: &egui::Context) {
        if self.settings.is_startup {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "JetBrains".to_owned(),
                egui::FontData::from_static(include_bytes!(
                    "fonts/JetBrainsMonoRegularNerdFontComplete.ttf"
                )),
            ); // .ttf and .otf supported
            fonts.font_data.insert(
                "Icons".to_owned(),
                egui::FontData::from_static(include_bytes!(
                    "fonts/Symbols-1000-emNerdFontCompleteMono.ttf"
                )),
            );

            let mut style = (*ctx.style()).clone();
            style.text_styles = [
                (
                    egui::TextStyle::Heading,
                    egui::FontId::new(25.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Body,
                    egui::FontId::new(18.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Monospace,
                    egui::FontId::new(17.0, egui::FontFamily::Monospace),
                ),
                (
                    egui::TextStyle::Button,
                    egui::FontId::new(16.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Small,
                    egui::FontId::new(20.0, egui::FontFamily::Proportional),
                ),
            ]
            .into();
            fonts
                .families
                .get_mut(&egui::FontFamily::Monospace)
                .unwrap()
                .push("JetBrains".to_owned());
            fonts
                .families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .push("JetBrains".to_owned());
            ctx.set_fonts(fonts);
            ctx.set_style(style);
            self.settings.is_startup = false;
        }
    }

    pub fn new(cc: &eframe::CreationContext<'_>, path: Option<PathBuf>) -> Self {
        match path {
            Some(a) => {
                let mut ed = Self {
                    picked_path: String::from(a.into_os_string().into_string().unwrap()),
                    ..Default::default()
                };
                let pick = ed.picked_path.clone();
                file::file_open(&mut ed, pick);
                ed
            }
            None => Self::default(),
        }
    }
}
