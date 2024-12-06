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

fn test_levels_validity(levels: &Vec<i64>) -> bool {
    let level_diff_evolution: Vec<i64> = levels
        .iter()
        .map_windows(|[x, y]| {
            return *x - *y;
        })
        .collect();
    let report_is_negative = i64::is_negative(*level_diff_evolution.first().expect("msg"));
    return !level_diff_evolution.iter().any(|&level_diff| {
        (report_is_negative && i64::is_positive(level_diff))
            || (!report_is_negative && i64::is_negative(level_diff)
                || level_diff.abs().eq(&0)
                || level_diff.abs().gt(&3))
    });
}

fn attempt_single_fix(levels: Vec<i64>, checker: fn(&Vec<i64>) -> bool) -> bool {
    for index in 0..levels.len() {
        let mut levels_copy = levels.clone();
        levels_copy.remove(index);
        if checker(&levels_copy) {
            println!("Fix = {:?}", levels_copy);
            return true;
        }
    }

    return false;
}

fn process_file(file_path: PathBuf) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut counter: u32 = 0;
    for report in contents.trim().lines() {
        let levels: Vec<i64> = report
            .split_whitespace()
            .map(|x| x.trim().parse().expect("cannot parse number"))
            .collect();

        if test_levels_validity(&levels) {
            counter += 1;
            println!("Perfect report = {report}");
            continue;
        } else if attempt_single_fix(levels, test_levels_validity) {
            counter += 1;
            println!("Fixed report = {report}");
            continue;
        }

        println!("Bad reports = {report}");
    }

    println!("Number of reports = {counter}");
}
