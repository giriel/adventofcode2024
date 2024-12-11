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

    let rules_regex = Regex::new(r#"^(?<lhs>[1-9][0-9])\|(?<rhs>[1-9][0-9])$"#).unwrap();
    let input_regex = Regex::new(r#"(?<entry>[1-9][0-9]),?"#).unwrap();

    let mut rules: Vec<(i64, i64)> = Vec::new();
    let mut input: Vec<Vec<i64>> = Vec::new();
    let mut is_rules = true;
    for line in contents.trim().lines() {
        if line.trim().is_empty() {
            is_rules = false;
            continue;
        }

        if is_rules {
            let mut rule: Vec<(i64, i64)> = rules_regex
                .captures_iter(line)
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
                    (lhs, rhs)
                })
                .collect();
            rules.append(&mut rule);
        } else {
            let inputline: Vec<i64> = input_regex
                .captures_iter(line)
                .map(|caps| {
                    let entry: i64 = caps
                        .name("entry")
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("no number");
                    entry
                })
                .collect();
            input.push(inputline);
        }
    }

    println!("{:?}", rules);
    println!("{:?}", input);
    println!("Result = {}", 0);
}
