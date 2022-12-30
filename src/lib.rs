use crate::cli::Config;

pub mod cli;
mod interactive_scanner;
mod non_interactive_scanner;

pub fn scan(cfg: Config) {
    if cfg.interactive {
        interactive_scanner::scan_files(cfg);
    } else {
        non_interactive_scanner::scan_files(cfg);
    }
}
