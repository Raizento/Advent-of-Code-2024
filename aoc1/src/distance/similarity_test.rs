use std::collections::HashMap;
use crate::distance::similarity;

use super::similarity::similarity;

#[test]
fn occurence_map_test() {
    let list = vec![1, 2, 2, 2, 3];
    
    let expected_occurence_map = HashMap::from([(1, 1), (2, 3), (3, 1)]);

    let actual_occurence_map = similarity::occurence_map(&list);

    assert_eq!(expected_occurence_map, actual_occurence_map);
}

#[test]
fn occurence_map_test_unsorted_list() {
    let list = vec![2, 1, 3, 2, 2];
    
    let expected_occurence_map = HashMap::from([(1, 1), (2, 3), (3, 1)]);

    let actual_occurence_map = similarity::occurence_map(&list);

    assert_eq!(expected_occurence_map, actual_occurence_map);
}

#[test]
fn similarity_test() {
    let left = vec![1, 2, 3];
    let right = vec![1, 2, 2, 2, 3];

    let expected_similarity = 1 + 6 + 3;

    let actual_similarity = similarity(&left, &right);

    assert_eq!(expected_similarity, actual_similarity)
}
