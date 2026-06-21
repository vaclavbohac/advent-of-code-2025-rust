mod battery;

use std::io::{self, BufRead};
use crate::battery::{get_joltage, parse_battery};

fn main() {
    let stdin = io::stdin();

    let mut joltage2 = 0;
    let mut joltage12 = 0;

    for line in stdin.lock().lines() {
        let battery = parse_battery(line.unwrap().trim());

        joltage2 += get_joltage(&battery, 2);
        joltage12 += get_joltage(&battery, 12);
    }

    println!("{}", joltage2);
    println!("{}", joltage12);
}
