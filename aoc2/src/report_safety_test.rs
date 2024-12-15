use crate::report_safety::{is_safe, Report};

#[test]
fn valid_report_test() {
    let report = Report(vec![1, 2, 3, 4, 5]);
    let is_safe = is_safe(&report);

    assert!(is_safe);
}

#[test]
fn valid_report_test_two() {
    let report = Report(vec![48, 49, 52, 54, 55, 58, 59]);
    let is_safe = is_safe(&report);

    assert!(is_safe);
}

#[test]
fn invalid_report_distances_too_big_test() {
    let report = Report(vec![1, 4, 8, 9, 10]);
    let is_safe = is_safe(&report);

    assert!(!is_safe);
}

#[test]
fn invalid_report_sign_change() {
    let report = Report(vec![1, 4, 3, 5, 7]);
    let is_safe = is_safe(&report);

    assert!(!is_safe);
}

#[test]
fn report_with_single_entry_test() {
    let report = Report(vec![1]);
    let is_safe = is_safe(&report);

    assert!(is_safe);
}
