use super::distance::distance;
use super::distance::{total_distance, tuple_distance};

#[test]
pub fn distance_test() {
    let distance = distance(5, 3);
    assert_eq!(2, distance);
}

#[test]
pub fn tuple_distance_test() {
    let tuple = (&1, &3);
    let distance = tuple_distance(tuple);
    assert_eq!(2, distance);
}

#[test]
pub fn total_distance_test_lists_ordered() {
    let mut left = [1, 3, 7, 8];
    let mut right = [2, 4, 6, 8];

    let total_distance = total_distance(&mut left, &mut right);
    assert_eq!(3, total_distance);
}

#[test]
pub fn total_distance_test_lists_unordered() {
    let mut left = [1, 3, 8, 7];
    let mut right = [2, 4, 6, 8];

    let total_distance = total_distance(&mut left, &mut right);
    assert_eq!(3, total_distance);
}
