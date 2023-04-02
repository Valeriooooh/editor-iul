use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cli")]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    pub file_path: Option<PathBuf>,
}
