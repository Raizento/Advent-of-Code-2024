use crate::report_safety::{is_safe, is_safe_dampened, is_safe_normal, Report};

#[test]
fn valid_report_test() {
    let report = Report(vec![1, 2, 3, 4, 5]);
    let is_safe = is_safe(&report, is_safe_normal);

    assert!(is_safe);
}

#[test]
fn valid_report_test_two() {
    let report = Report(vec![48, 49, 52, 54, 55, 58, 59]);
    let is_safe = is_safe(&report, is_safe_normal);

    assert!(is_safe);
}

#[test]
fn invalid_report_distances_too_big_test() {
    let report = Report(vec![1, 4, 8, 9, 10]);
    let is_safe = is_safe(&report, is_safe_normal);

    assert!(!is_safe);
}

#[test]
fn invalid_report_sign_change() {
    let report = Report(vec![1, 4, 3, 5, 7]);
    let is_safe = is_safe(&report, is_safe_normal);

    assert!(!is_safe);
}

#[test]
fn report_with_single_entry_test() {
    let report = Report(vec![1]);
    let is_safe = is_safe(&report, is_safe_normal);

    assert!(is_safe);
}

pub mod dampened {
    use crate::report_safety::{is_safe, is_safe_dampened, Report};

    #[test]
    fn is_safe_dampened_with_valid_report_no_errors_test() {
        let report = Report(vec![1, 2, 3, 4, 5]);
        let is_safe = is_safe(&report, is_safe_dampened);

        assert!(is_safe);
    }

    #[test]
    fn is_safe_dampened_with_valid_report_one_error_test() {
        let report = Report(vec![1, 2, 3, 5, 4]);
        let is_safe = is_safe(&report, is_safe_dampened);

        assert!(is_safe);
    }

    #[test]
    fn is_safe_dampened_with_invalid_report_ascending_test() {
        let report = Report(vec![1, 2, 7, 8, 9]);
        let is_safe = is_safe(&report, is_safe_dampened);

        assert!(!is_safe);
    }

    #[test]
    fn is_safe_dampened_with_invalid_report_descending_test() {
        let report = Report(vec![9, 8, 7, 2, 1]);
        let is_safe = is_safe(&report, is_safe_dampened);

        assert!(!is_safe);
    }

    #[test]
    fn is_safe_dampened_valid_report_first_element_erroneous_test() {
        let report = Report(vec![19, 19, 17, 15, 13, 12]);
        let is_safe = is_safe(&report, is_safe_dampened);

        assert!(is_safe);
    }

    #[test]
    fn is_safe_dampened_sign_change_at_start_test() {
        let report = Report(vec![7, 9, 6, 5, 4, 3]);
        let is_safe = is_safe(&report, is_safe_dampened);

        assert!(is_safe);
    }
}
