use crate::cli::{create_progress_bar, Config};
use indicatif::ProgressBar;
use log::{debug, error, info};
use sha256::try_digest;
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub(super) fn scan_files(cfg: Config) {
    let scan_path = cfg.scan_dir.unwrap();

    let pb = create_progress_bar();
    let mut idx: HashMap<String, PathBuf> = HashMap::new();

    scan_files_in_path(scan_path, idx.borrow_mut(), pb.borrow());

    info!("Final index size {}", idx.len());

    pb.set_length(pb.position());
    let dup_cnt = pb.position() - (idx.len() as u64);
    pb.finish_with_message(format!("Found {} duplicates", dup_cnt));
}

fn scan_files_in_path(
    scan_file_path: PathBuf,
    idx: &mut HashMap<String, PathBuf>,
    pb: &ProgressBar,
) {
    let paths = fs::read_dir(scan_file_path).unwrap();

    for path in paths {
        let current_file = path.unwrap().path();
        if current_file.is_dir() {
            debug!("Enter directory {}", current_file.display());
            scan_files_in_path(current_file, idx, pb);
        } else {
            debug!("Current file {}", current_file.display());
            pb.set_message(format!("file:{}", current_file.display()));
            pb.set_position(pb.position() + 1);
            let f_hash = try_digest(current_file.as_path()).unwrap(); // TODO: handle error properly
            let found_hash = idx.get_key_value(f_hash.as_str());
            if found_hash.is_some() {
                let entry = found_hash.unwrap();
                println!("{} == {}", entry.1.display(), current_file.display());
                let current_path = current_file.borrow();
                let mut user_action = choose_action(entry.1, current_path);
                while user_action.is_none() {
                    user_action = choose_action(entry.1, current_path);
                }
                match user_action {
                    Some(0) => println!("Keep both"),
                    Some(1) => {
                        delete_file(entry.1);
                        idx.insert(f_hash, current_file);
                    }
                    Some(2) => delete_file(current_path),
                    _ => panic!("unexpected user input"),
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

fn choose_action(left: &PathBuf, right: &Path) -> Option<u8> {
    println!(
        "Choose an action:\n0 - keep both\n1 - delete {}\n2 - delete {}",
        left.display(),
        right.display()
    );
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {}
        Err(_no_updates_is_fine) => {}
    }

    match input.trim() {
        "0" => return Some(0),
        "1" => return Some(1),
        "2" => return Some(2),
        x => {
            println!("Unexpected input '{}'", x);
            return None;
        }
    }
}

fn delete_file(path: &Path) {
    let path_d = path.display();
    match fs::remove_file(path) {
        Err(_error) => error!("Failed to delete file {} {}", path_d, _error),
        Ok(_) => println!("File deleted {}", path_d),
    }
}
