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

fn search_xmas(
    letter: char,
    input_matrix: &Vec<Vec<char>>,
    row_index: usize,
    column_index: usize,
) -> i32 {
    return 1;
}
fn search_samx(
    letter: char,
    input_matrix: &Vec<Vec<char>>,
    row_index: usize,
    column_index: usize,
) -> i32 {
    return 1;
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

    let mut hits_heatmap = vec![vec![0; input_matrix.len()]; input_matrix[0].len()];
    for row_index in 0..input_matrix.len() {
        let row = &input_matrix[row_index];
        for column_index in 0..row.len() {
            let letter = row[column_index];
            if letter == 'X' {
                hits_heatmap[row_index][column_index] =
                    search_xmas(letter, &input_matrix, row_index, column_index);
            } else if letter == 'S' {
                hits_heatmap[row_index][column_index] =
                    search_samx(letter, &input_matrix, row_index, column_index);
            }
        }
    }

    for row in &hits_heatmap {
        println!("{:?}", row);
    }

    println!(
        "Number of XMAS hits = {}",
        hits_heatmap
            .iter()
            .map(|x| -> i32 { x.iter().sum() })
            .sum::<i32>()
    );
}
