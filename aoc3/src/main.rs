use std::fs::read_to_string;

use aoc3::extraction::{extract_multiplication_pairs_from_text, MultiplicationPair};

fn main() {
    let text = read_to_string(concat!(std::env!("CARGO_MANIFEST_DIR"), "/", "resources/input_one"))
        .expect("File needs to be readable");

    let multiplication_pairs = extract_multiplication_pairs_from_text(&text);

    let sum: i32 = multiplication_pairs
        .iter()
        .map(MultiplicationPair::multiply)
        .sum();

    println!("Sum is: {}", sum);
}
