#![feature(iter_map_windows)]
use std::env;
use std::fs;
use std::path::PathBuf;

use regex::Regex;

fn main() {
    println!("Please select a file to process:");
    let file = select_file();
    match file {
        Some(file) => process_file(file),
        None => println!("No file selected."),
    }
}

fn select_file() -> Option<PathBuf> {
    return rfd::FileDialog::new()
        .set_directory(env::current_dir().expect("Cannot fetch current directory?"))
        .pick_file();
}

fn process_file(file_path: PathBuf) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut xmas_hits = 0;
    for report in contents.trim().lines() {}

    println!("Number of XMAS hits = {}", xmas_hits);
}
