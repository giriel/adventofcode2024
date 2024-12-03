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

    let mut counter: u32 = 0;
    for report in contents.trim().lines() {
        let levels: Vec<u32> = report
            .split_whitespace()
            .map(|x| x.trim().parse().expect("cannot parse number"))
            .collect();
        let level_diffs: Vec<i64> = levels
            .iter()
            .map_windows(|[x, y]| {
                i64::try_from(**x).expect("msg") - i64::try_from(**y).expect("msg")
            })
            .collect();

        let report_is_negative = i64::is_negative(*level_diffs.first().expect("msg"));
        let problem = level_diffs.iter().any(|&level_diff| {
            (report_is_negative && i64::is_positive(level_diff))
                || (!report_is_negative && i64::is_negative(level_diff)
                    || level_diff.abs().eq(&0)
                    || level_diff.abs().gt(&3))
        });
        if !problem {
            counter += 1;
        }
    }

    println!("Number of reports = {counter}");
}
