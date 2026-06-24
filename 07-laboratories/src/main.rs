use crate::tachyon_manifold::{count_quantum_timelines, count_splits};
use std::io;
use std::io::Read;

mod tachyon_manifold;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", count_splits(&input));
    println!("{}", count_quantum_timelines(&input));
}
