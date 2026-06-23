use crate::worksheet::parse_worksheet;
use std::io;
use std::io::Read;

mod worksheet;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let worksheet = parse_worksheet(&input);

    println!("{}", worksheet.calculate())
}
