#[derive(Debug)]
pub struct Report(pub Vec<i32>);

pub fn is_safe(report: &Report) -> bool {
    if report.0.len() < 2 {
        return true;
    }

    let last_sign = (report.0[0] - report.0[1]).signum();

    for window in report.0.windows(2) {
        let distance = (window[0] - window[1]).abs();
        let sign = (window[0] - window[1]).signum();

        if !(1..=3).contains(&distance) {
            return false;
        }
        
        if last_sign != sign {
            return false;
        }
    }

    true
}
