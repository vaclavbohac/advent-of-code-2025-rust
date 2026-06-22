mod space_calculator;

use std::io::{self, BufRead};
use crate::space_calculator::count_accessible_paper_rolls;

fn main() {
    let stdin = io::stdin();

    let grid: Vec<Vec<char>> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    println!("{}", count_accessible_paper_rolls(grid))
}
