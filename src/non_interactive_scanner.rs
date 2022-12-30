use crate::cli::{create_progress_bar, Config};
use indicatif::ProgressBar;
use log::{debug, error, info};
use sha256::try_digest;
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub(super) fn scan_files(cfg: Config) {
    let scan_path = cfg.scan_dir.unwrap();
    let report_path = cfg.report.unwrap();

    let mut report_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(report_path.as_path())
        .unwrap();

    let pb = create_progress_bar();
    let mut idx: HashMap<String, PathBuf> = HashMap::new();

    scan_files_in_path(
        scan_path,
        idx.borrow_mut(),
        report_file.borrow_mut(),
        pb.borrow(),
    );

    info!("Final index size {}", idx.len());

    pb.set_length(pb.position());
    let dup_cnt = pb.position() - (idx.len() as u64);
    pb.finish_with_message(format!("Found {} duplicates", dup_cnt));
}

fn scan_files_in_path(
    scan_file_path: PathBuf,
    idx: &mut HashMap<String, PathBuf>,
    result_file: &mut File,
    pb: &ProgressBar,
) {
    let paths = fs::read_dir(scan_file_path).unwrap();

    for path in paths {
        let current_file = path.unwrap().path();
        if current_file.is_dir() {
            debug!("Enter directory {}", current_file.display());
            scan_files_in_path(current_file, idx, result_file, pb);
        } else {
            debug!("Current file {}", current_file.display());
            pb.set_message(format!("file:{}", current_file.display()));
            pb.set_position(pb.position() + 1);
            let f_hash = try_digest(current_file.as_path()).unwrap();
            let found_hash = idx.get_key_value(f_hash.as_str());
            if found_hash.is_some() {
                let entry = found_hash.unwrap();
                println!("{} == {}", entry.1.display(), current_file.display());
                if let Err(e) = writeln!(
                    result_file,
                    "{} == {}",
                    entry.1.display(),
                    current_file.display()
                ) {
                    error!("Couldn't write to file: {}", e);
                }
            } else {
                idx.insert(f_hash, current_file);
            }
        }
        if (idx.len() % 50) == 0 {
            debug!("indexed files {}", idx.len());
        }
    }
}
