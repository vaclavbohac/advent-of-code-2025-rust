mod battery;

use std::io::{self, BufRead};
use crate::battery::{get_joltage, parse_battery};

fn main() {
    let stdin = io::stdin();

    let mut joltage = 0;
    let mut joltage2 = 0;

    for line in stdin.lock().lines() {
        let battery = parse_battery(line.unwrap().trim());

        joltage += get_joltage(battery.clone(), 2);
        joltage2 += get_joltage(battery.clone(), 12);
    }

    println!("{}", joltage);
    println!("{}", joltage2);
}
