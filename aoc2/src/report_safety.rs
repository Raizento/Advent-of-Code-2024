#[derive(Debug)]
pub struct Report(pub Vec<i32>);

/// Checks if a [Report](Report) is safe.
/// [safety_fuction](safety_function) determines whether or not a given report is safe. It needs to return a boolean.
/// Reports with a length of one are always safe.
pub fn is_safe(report: &Report, safety_function: fn(&Report) -> bool) -> bool {
    if report.0.len() < 2 {
        return true;
    }

    safety_function(report)
}

/// Safety function for use with [is_safe](is_safe).
/// Checks if a [Report](Report) is safe.
/// If an error is encountered, the report will be unsafe.
pub fn is_safe_normal(report: &Report) -> bool {
    let is_increasing = (report.0[0] - report.0[1]).signum();

    for window in report.0.windows(2) {
        if !are_levels_safe(is_increasing, window[0], window[1]) {
            return false;
        }
    }

    true
}

/// Safety function for use with [is_safe](is_safe).
/// Checks if a [Report](Report) is safe.
/// If there is a single error in the report, it will still be safe.
/// Due to this, a report of length 2 is always safe.
pub fn is_safe_dampened(report: &Report) -> bool {
    if report.0.len() == 2 {
        return true; 
    }

    let levels = report.0.clone();

    for i in 0..levels.len() {
        let mut levels = levels.clone();
        levels.remove(i);

        let mut is_safe = true;
        let is_increasing = (levels[0] - levels[1]).signum();
        for window in levels.windows(2) {
            if !are_levels_safe(is_increasing, window[0], window[1]) {
                is_safe = false;
            }
        }

        if is_safe {
            return true
        }
    }

    false
}

/// Two levels are safe if
/// - Their distance is between 1 and 3 (inclusive) and
/// - their value is steadily decreasing in regards to the last level. A valid sequence would be
///     [1, 2, 3], but [1, 3, 2] would be invalid since it first increases and then decreases.
fn are_levels_safe(is_increasing: i32, level_one: i32, level_two: i32) -> bool {
    let difference = level_one - level_two;
    let sign = difference.signum();

    do_levels_have_valid_distance(level_one, level_two) && sign == is_increasing
}

/// Checks whether or not two levels have a valid distance.
fn do_levels_have_valid_distance(level_one: i32, level_two: i32) -> bool {
    let distance = (level_one - level_two).abs();
    (1..=3).contains(&distance)
}
