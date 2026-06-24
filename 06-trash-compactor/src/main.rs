use crate::worksheet::{parse_worksheet, parse_worksheet_horizontally};
use std::io;
use std::io::Read;

mod worksheet;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", parse_worksheet(&input).calculate());
    println!("{}", parse_worksheet_horizontally(&input).calculate());
}
