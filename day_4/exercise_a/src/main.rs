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

const XMAS: &str = "XMAS";
const SAMX: &str = "SAMX";

fn count_horizontal(column_index: usize, input_matrix: &Vec<char>) -> i32 {
    let mut count = 0;
    if column_index + XMAS.len() < input_matrix.len()
        && XMAS
            == input_matrix[column_index..(column_index + XMAS.len())]
                .iter()
                .collect::<String>()
    {
        count += 1;
    }
    if column_index >= SAMX.len()
        && SAMX
            == input_matrix[(column_index - SAMX.len() + 1)..column_index + 1]
                .iter()
                .collect::<String>()
    {
        count += 1;
    }

    return count;
}

fn search_xmas(
    letter: char,
    input_matrix: &Vec<Vec<char>>,
    row_index: usize,
    column_index: usize,
) -> i32 {
    let mut total = count_horizontal(column_index, &input_matrix[row_index]);
    total += count_vertical(row_index, column_index, &input_matrix);
    return total;
}

fn count_vertical(row_index: usize, column_index: usize, input_matrix: &Vec<Vec<char>>) -> i32 {
    let can_check_downwards = row_index + 1 < input_matrix.len() - XMAS.len();
    let can_check_upwards = row_index + 1 > XMAS.len();

    let mut matches = 0;
    if can_check_downwards {
        matches += 1;
    }
    if can_check_upwards {
        matches += 1;
    }
    if can_check_upwards {
        for index in 1..XMAS.len() {
            let row_upwards = row_index.checked_sub(index).expect("checked this up front");
            if input_matrix[row_upwards][column_index]
                != XMAS.chars().nth(index).expect("checked up front")
            {
                matches -= 1;
                break;
            }
        }
    }

    if can_check_downwards {
        for index in 1..XMAS.len() {
            let row_downwards = row_index.checked_add(index).expect("checked this up front");
            if input_matrix[row_downwards][column_index]
                != XMAS.chars().nth(index).expect("checked up front")
            {
                matches -= 1;
                break;
            }
        }
    }

    return matches;
}

fn process_file(file_path: PathBuf) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut input_matrix: Vec<Vec<char>> = Vec::new();
    for report in contents.trim().lines() {
        input_matrix.push(report.chars().collect::<Vec<char>>());
    }

    // let's start from X as the start of XMAS
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
