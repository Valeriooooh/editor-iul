use std::{
    ffi::OsStr,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use eframe::egui;

use super::Editor;

#[derive(Debug)]
pub enum FileError {
    WriteError(String),
    ReadError(String),
}

pub fn file_read(path: String) -> Result<String, FileError> {
    let mut str = String::new();
    let file = match File::open(path) {
        Ok(a) => a,
        Err(e) => return Err(FileError::ReadError(e.to_string())),
    };
    let mut buf = BufReader::new(file);
    match buf.read_to_string(&mut str) {
        Ok(_) => {}
        Err(e) => return Err(FileError::ReadError(e.to_string())),
    };
    Ok(str)
}

pub fn file_write(path: String, content: String) -> Result<(), FileError> {
    match std::fs::write(path, content) {
        Ok(_) => {}
        Err(e) => return Err(FileError::WriteError(e.to_string())),
    };
    Ok(())
}

pub fn file_open(ed: &mut Editor, path: String) {
    let prevpath = ed.picked_path.clone();
    ed.picked_path = path;
    match file_read(ed.picked_path.clone()) {
        Ok(a) => {
            ed.code = a;
            ed.saved = true;
            let extension = match Path::new(ed.picked_path.as_str())
                .extension()
                .and_then(OsStr::to_str)
            {
                Some(a) => a.to_string(),
                None => ed.lang.clone(),
            };
            ed.lang = extension;
        }
        Err(e) => {
            ed.picked_path = prevpath;
            println!("Error: {:?}", e);
        }
    };
}

pub fn file_picker(ed: &mut Editor) {
    match rfd::FileDialog::new().pick_file() {
        Some(path) => {
            let path = Some(path.display().to_string());
            match path {
                Some(a) => file_open(ed, a),
                None => {}
            }
        }
        _ => (),
    }
}

pub fn project_picker(ed: &mut Editor) {
    match rfd::FileDialog::new().pick_folder() {
        Some(path) => {
            let path = Some(path.display().to_string());
            match path {
                Some(a) => ed.project_path = Some(a),
                None => {}
            }
        }
        _ => (),
    }
}
pub fn file_save(ed: &mut Editor) {
    match rfd::FileDialog::new().save_file() {
        Some(path) => {
            let path = Some(path.display().to_string());
            match path {
                Some(a) => {
                    match file_write(ed.picked_path.clone(), ed.code.clone()) {
                        Ok(_) => {
                            ed.picked_path = a;
                            ed.saved = true
                        }
                        Err(e) => println!("Error: {:?}", e),
                    };
                }
                None => {}
            }
        }
        _ => (),
    }
}

pub fn scan_dir(begin: String, ui: &mut egui::Ui, ed: &mut Editor) {
    let _ = match std::fs::read_dir(begin) {
        Ok(a) => {
            for i in a {
                match i {
                    Ok(file) => match file.path().file_name() {
                        Some(a) => {
                            if file.path().is_dir() {
                                egui::CollapsingHeader::new(format!(
                                    "🗁 {}",
                                    match a.to_str() {
                                        Some(a) => a,
                                        None => {
                                            ""
                                        }
                                    }
                                ))
                                .show(ui, |ui| scan_dir(file.path().display().to_string(), ui, ed));
                            } else {
                                if (ui.button(format!(
                                    "{}{}",
                                    get_lang_icon({
                                        match file.path().extension().and_then(OsStr::to_str) {
                                            Some(a) => a,
                                            None => " ",
                                        }
                                    }),
                                    match a.to_str() {
                                        Some(a) => a,
                                        None => {
                                            ""
                                        }
                                    }
                                )))
                                .clicked()
                                {
                                    file_open(ed, file.path().display().to_string());
                                }
                            }
                        }
                        None => {}
                    },
                    Err(_) => {}
                }
            }
        }
        Err(_) => todo!(),
    };
}

fn get_lang_icon(extension: &str) -> &str {
    match extension {
        "rs" => "🦀",
        "java" => "🍵",
        "kt" => "",
        _ => " ",
    }
}
