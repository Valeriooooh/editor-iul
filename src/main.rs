mod cli;
mod editor;
use editor::*;
use structopt::StructOpt;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let args = cli::Cli::from_args();
    println!("{:?}", args);
    let _ = eframe::run_native(
        "editor-iul",
        native_options,
        Box::new(|cc| Box::new(Editor::new(cc, args.file_path))),
    );
}
