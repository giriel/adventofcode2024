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

    let mut first_column: Vec<i32> = Vec::new();
    let mut second_column: Vec<i32> = Vec::new();
    for line in contents.trim().lines() {
        println!("{line}");
        let mut exploded_string = line.split_whitespace();
        match exploded_string.next() {
            Some(first_number) => {
                first_column.push(first_number.trim().parse().expect("cannot parse number"))
            }
            None => println!("skipping line"),
        }
        match exploded_string.next() {
            Some(second_number) => {
                second_column.push(second_number.trim().parse().expect("cannot parse number"))
            }
            None => println!("skipping line"),
        }
    }

    first_column.sort();
    second_column.sort();
    let mut diff: Vec<u32> = Vec::new();
    for (first, second) in first_column.iter().zip(second_column.iter()) {
        diff.push(first.abs_diff(*second));
    }
    let total_difference: u32 = diff.iter().sum();
    println!("Total difference = {total_difference}");
}
