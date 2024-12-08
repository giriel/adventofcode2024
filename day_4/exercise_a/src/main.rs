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

fn search_xmas(input_matrix: &Vec<Vec<char>>, row_index: usize, column_index: usize) -> i32 {
    let mut total = count_horizontal(column_index, &input_matrix[row_index]);
    total += count_vertical(row_index, column_index, &input_matrix);
    total += count_diagonally(row_index, column_index, &input_matrix);
    return total;
}

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

fn count_diagonally(row_index: usize, column_index: usize, input_matrix: &Vec<Vec<char>>) -> i32 {
    let max_rows = input_matrix.len();
    let max_columns = input_matrix[0].len();
    let advance_left_top = [-1, -1];
    let advance_right_top = [1, -1];
    let advance_left_bottom = [-1, 1];
    let advance_right_bottom = [1, 1];

    let mut diagonal_matches = [
        Search {
            valid: true,
            row_index: row_index,
            column_index: column_index,
            advance: advance_left_top,
        },
        Search {
            valid: true,
            row_index: row_index,
            column_index: column_index,
            advance: advance_right_top,
        },
        Search {
            valid: true,
            row_index: row_index,
            column_index: column_index,
            advance: advance_left_bottom,
        },
        Search {
            valid: true,
            row_index,
            column_index,
            advance: advance_right_bottom,
        },
    ];
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
