use crate::extraction::{extract_multiplication_pairs_from_text, MultiplicationPair};

#[test]
fn extract_single_multiplication_pair_test() {
    let text = "mul(1,2)";

    let expected_pair = MultiplicationPair(1, 2);

    let multiplication_pairs = extract_multiplication_pairs_from_text(text);

    assert_eq!(expected_pair, multiplication_pairs[0]);
}

#[test]
fn extract_multiple_multiplication_pairs_test() {
    let text = "mul(1,2)mul(3,4)";
    let expected_pairs = vec![MultiplicationPair(1, 2), MultiplicationPair(3, 4)];

    let multiplication_pairs = extract_multiplication_pairs_from_text(text);

    assert_eq!(expected_pairs, multiplication_pairs);
}

#[test]
fn extract_multiple_multiplication_pairs_with_garbage_text_test() {
    let text = "mul(1,2)wiofnjavnkjvnmul(3,4)";
    let expected_pairs = vec![MultiplicationPair(1, 2), MultiplicationPair(3, 4)];

    let multiplication_pairs = extract_multiplication_pairs_from_text(text);

    assert_eq!(expected_pairs, multiplication_pairs);
}
