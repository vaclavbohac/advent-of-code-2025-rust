mod grid;
mod space_calculator;
mod chars;

use crate::grid::Grid;
use crate::space_calculator::count_accessible_paper_rolls;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let grid = Grid::from_str(&input);

    println!("{}", count_accessible_paper_rolls(&grid))
}
