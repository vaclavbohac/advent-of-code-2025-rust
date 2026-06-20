mod series_parser;
mod simple_validator;
mod advanced_validator;

use crate::simple_validator::is_invalid_id as is_invalid_id_simple;
use crate::advanced_validator::is_invalid_id as is_invalid_id_advanced;
use series_parser::parse_series;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");

        let mut sum_simples = 0;
        let mut sum_advanced = 0;

        for series in line.trim().split(",") {
            for id in parse_series(series) {
                if is_invalid_id_simple(id) {
                    sum_simples += id
                }
                if is_invalid_id_advanced(id) {
                    sum_advanced += id
                }
            }
        }

        println!("{} {}", sum_simples, sum_advanced);
    }
}
