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

    let mult_regex = Regex::new(r#"mul\((?<lhs>[1-9][0-9]*),(?<rhs>[1-9][0-9]*)\)"#).unwrap();

    let mut multiplications: Vec<i64> = Vec::new();
    for report in contents.trim().lines() {
        let mut multi: Vec<i64> = mult_regex
            .captures_iter(report)
            .map(|caps| {
                let lhs: i64 = caps
                    .name("lhs")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("no number");
                let rhs: i64 = caps
                    .name("rhs")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("no number");
                lhs * rhs
            })
            .collect();
        multiplications.append(&mut multi);
    }

    let sum: i64 = multiplications.iter().sum();
    println!("Multiplication result = {}", sum);
}
