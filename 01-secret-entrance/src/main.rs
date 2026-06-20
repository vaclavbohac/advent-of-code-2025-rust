mod rotation;
mod dial;

use std::io::{self, BufRead};
use crate::dial::Dial;

fn main() {
    let stdin = io::stdin();

    let mut dial = Dial::new(50);
    let mut pointing_at_zero_count = 0;
    let mut sum_zeros_crossed = 0;

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");

        dial = dial.rotate(line.trim());

        if dial.pointing_at_zero() {
            pointing_at_zero_count += 1;
        }

        sum_zeros_crossed += dial.zeros_crossed()
    }

    println!("{}", pointing_at_zero_count);
    println!("{}", sum_zeros_crossed)
}
