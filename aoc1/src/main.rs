use std::fs::read_to_string;

use aoc1::distance;

fn main() {
    let file_path = concat!(std::env!("CARGO_MANIFEST_DIR"), "/", "resources/input_one");
    println!("Opening file {}", file_path);
    let (mut left, mut right) = read_lists_from_file(file_path);
    
    let total_distance = distance::distance::total_distance(&mut left, &mut right);
    println!("Total distance is {}", total_distance);

    let similarity = distance::similarity::similarity(&left, &right);
    println!("Similarity is {}", similarity);
}

fn read_lists_from_file(path: &str) -> (Vec<i32>, Vec<i32>) {
    let file = read_to_string(path).expect("File needs to be readable!");

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in file.lines() {
        let parts: Vec<i32> = line
            .split("   ")
            .map(|part: &str| {
                part.parse::<i32>()
                    .unwrap_or_else(|_| panic!("File content needs to be an integer, was {}", part))
            })
            .collect();

        left.push(parts[0]);
        right.push(parts[1]);
    }

    (left, right)
}
