mod chars;
mod grid;
mod space_calculator;

use crate::grid::Grid;
use crate::space_calculator::{count_accessible_paper_rolls, count_removable_paper_rolls};
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let grid = Grid::from_str(&input);

    println!("{}", count_accessible_paper_rolls(&grid));
    println!("{}", count_removable_paper_rolls(&grid))
}
