use std::cmp::Ordering;
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

    let mut first_column: Vec<u32> = Vec::new();
    let mut second_column: Vec<u32> = Vec::new();
    for line in contents.trim().lines() {
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
    let mut second_iter = second_column.iter().peekable();
    let mut similarity: Vec<u64> = Vec::new();
    let mut prev_first: u32 = 0;
    let mut counter: u64 = 0;
    for first in first_column.iter() {
        if first.eq(&prev_first) {
            similarity.push(<u32 as Into<u64>>::into(*first) * counter);
            continue;
        } else {
            prev_first = *first;
            counter = 0;
        }

        while let Some(second) = second_iter.peek() {
            match first.cmp(second) {
                Ordering::Less => {
                    similarity.push(<u32 as Into<u64>>::into(*first) * counter);
                    break;
                }
                Ordering::Equal => {
                    second_iter.next();
                    counter += 1;
                }
                Ordering::Greater => {
                    second_iter.next();
                }
            }
        }
    }
    let total_difference: u64 = similarity.iter().sum();
    println!("Total difference = {total_difference}");
}
