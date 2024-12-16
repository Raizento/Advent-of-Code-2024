use regex::{Regex, RegexBuilder};

#[derive(Debug, PartialEq, Eq)]
pub struct MultiplicationPair(pub i32, pub i32);

impl MultiplicationPair {
    pub fn multiply(pair: &MultiplicationPair) -> i32 {
        pair.0 * pair.1
    }
}

const MULTIPLICATION_PAIR_REGEX: &str = r"mul\((\d+)\,(\d+)\)";
const ACTIVATED_MULTIPLICATION_REGEX: &str = r"(don't\(\).*?do\(\))";

pub fn extract_multiplication_pairs_from_text(text: &str) -> Vec<MultiplicationPair> {
    let re = Regex::new(MULTIPLICATION_PAIR_REGEX).expect("Need to use functioning Regex");

    let mut multiplication_pairs = vec![];
    for (_, [left, right]) in re.captures_iter(text).map(|c| c.extract()) {
        let multiplication_pair = MultiplicationPair(
            left.parse::<i32>()
                .expect("Regex should only match digits; error in Regex"),
            right
                .parse::<i32>()
                .expect("Regex should only match digits; error in Regex"),
        );

        multiplication_pairs.push(multiplication_pair);
    }

    multiplication_pairs
}

pub fn extract_activated_multiplications(text: &str) -> String {
    let re = RegexBuilder::new(ACTIVATED_MULTIPLICATION_REGEX)
        .dot_matches_new_line(true)
        .build()
        .expect("Need to use functioning Regex");
    re.replace_all(text, "").to_string()
}
