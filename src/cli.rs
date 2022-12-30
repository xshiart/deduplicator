use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Search for duplicate files in directory.",
    long_about = "Search for duplicate files in directory."
)]
pub struct Config {
    #[arg(
        short,
        long,
        required = true,
        value_name = "FILE",
        help = "path to directory for scan"
    )]
    pub scan_dir: Option<PathBuf>,

    #[arg(short, long, help = "use interactive mode")]
    pub interactive: bool,

    #[arg(
        short,
        long,
        required_unless_present = "interactive",
        value_name = "FILE",
        help = "path to report file"
    )]
    pub report: Option<PathBuf>,
}

pub fn parse_config() -> Config {
    Config::parse()
}

pub fn create_progress_bar() -> ProgressBar {
    let pb = ProgressBar::new(0);
    let spinner_style =
        ProgressStyle::with_template("[{elapsed_precise}][scanned:{pos}] {wide_msg}").unwrap();
    pb.set_style(spinner_style);
    pb
}
