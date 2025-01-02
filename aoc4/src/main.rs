use std::fs::read_to_string;

use aoc4::occurence_count::{count_occurences_of_cross_mas, count_occurrences_of_xmas};

#[tokio::main]
async fn main() {
    let text = read_to_string(concat!(std::env!("CARGO_MANIFEST_DIR"), "/", "resources/input_one"))
        .expect("File needs to be readable");

    let occurrences = count_occurrences_of_xmas(&text).await;

    println!("Occurrences of XMAS: {}", occurrences);

    let occurrences = count_occurences_of_cross_mas(&text);

    println!("Occurrences of X-MAS: {}", occurrences)
}
