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

struct Search {
    valid: bool,
    row_index: usize,
    column_index: usize,
    advance: [i32; 2],
}

impl Search {
    fn advance(&mut self, max_rows: usize, max_columns: usize) -> bool {
        let new_row_index: usize;
        match update_index(self.row_index, self.advance[0], max_rows) {
            Some(x) => {
                new_row_index = x;
            }
            None => {
                self.valid = false;
                return false;
            }
        }
        let new_column_index: usize;
        match update_index(self.column_index, self.advance[1], max_columns) {
            Some(x) => {
                new_column_index = x;
            }
            None => {
                self.valid = false;
                return false;
            }
        }
        self.row_index = new_row_index;
        self.column_index = new_column_index;
        return true;
    }

    fn is_valid(&self) -> bool {
        return self.valid;
    }

    fn matches_char(&self, input_matrix: &Vec<Vec<char>>, character: char) -> bool {
        return input_matrix[self.row_index][self.column_index] == character;
    }

    fn invalidate(&mut self) {
        self.valid = false;
    }
}

fn search_xmas(input_matrix: &Vec<Vec<char>>, row_index: usize, column_index: usize) -> i32 {
    let max_rows = input_matrix.len();
    let max_columns = input_matrix[0].len();
    let advancements = vec![
        [-1, -1],
        [-1, 0],
        [1, 0],
        [1, -1],
        [-1, 1],
        [1, 1],
        [0, -1],
        [0, 1],
    ];
    let mut diagonal_matches: Vec<Search> = Vec::new();
    for advancement in advancements {
        diagonal_matches.push(Search {
            valid: true,
            row_index,
            column_index,
            advance: advancement,
        });
    }
    for xmas_index in 1..XMAS.len() {
        let xmas_char = XMAS.chars().nth(xmas_index).expect("verified");
        let mut still_matching = false;
        for diagonal_match in &mut diagonal_matches {
            if !diagonal_match.is_valid() {
                continue;
            }
            if !diagonal_match.advance(max_rows, max_columns) {
                continue;
            }

            if diagonal_match.matches_char(input_matrix, xmas_char) {
                still_matching = true;
            } else {
                diagonal_match.invalidate();
            }
        }

        if !still_matching {
            return 0;
        }
    }

    return i32::try_from(diagonal_matches.iter().filter(|x| x.is_valid()).count()).expect("msg");
}

fn update_index(index: usize, update: i32, max: usize) -> Option<usize> {
    match i32::try_from(index) {
        Ok(x) => {
            let updated = x + update;
            let max: i32 = i32::try_from(max).expect("ok");
            let valid_range = 0..max;
            if valid_range.contains(&updated) {
                return usize::try_from(updated).ok();
            }
            None
        }
        Err(_) => None,
    }
}

fn process_file(file_path: PathBuf) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut input_matrix: Vec<Vec<char>> = Vec::new();
    for report in contents.trim().lines() {
        input_matrix.push(report.chars().collect::<Vec<char>>());
    }

    let mut hits_heatmap = vec![vec![0; input_matrix.len()]; input_matrix[0].len()];
    for row_index in 0..input_matrix.len() {
        let row = &input_matrix[row_index];
        for column_index in 0..row.len() {
            let letter = row[column_index];
            if letter == 'X' {
                hits_heatmap[row_index][column_index] =
                    search_xmas(&input_matrix, row_index, column_index);
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
