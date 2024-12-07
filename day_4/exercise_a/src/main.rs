#![feature(iter_map_windows)]
use std::env;
use std::fs;
use std::path::PathBuf;

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

    let mut input_matrix: Vec<Vec<char>> = Vec::new();
    for report in contents.trim().lines() {
        input_matrix.push(report.chars().collect::<Vec<char>>());
    }

    // X can be the start of XMAS
    // S can be the start of SAMX
    // for every of these we should check matches
    //  horizontally in both directions
    //  vertically in both directions
    //  diagonally in 4 directions

    println!("Number of XMAS hits = {}", xmas_hits);
}
