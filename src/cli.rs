use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cli")]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    pub path: Option<PathBuf>,

    #[structopt(short, long, parse(from_os_str))]
    pub project: Option<PathBuf>,
}
