mod inventory;
mod range;

use crate::inventory::parse_inventory;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let inventory = parse_inventory(&input);

    println!("{}", inventory.count_fresh_ingredients());
    println!(
        "{}",
        inventory.count_unique_ids_in_fresh_ingredient_ranges()
    );
}
