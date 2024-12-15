use std::fs::read_to_string;

use aoc2::report_safety::{is_safe, Report};

fn main() {
    let reports = read_reports_from_file(concat!(std::env!("CARGO_MANIFEST_DIR"), "/", "resources/input_one"));
    let safe_reports = reports.iter().map(is_safe).filter(|is_safe| *is_safe);
    println!("{} reports were safe", safe_reports.count());

}

fn read_reports_from_file(path: &str) -> Vec<Report> {
    let file = read_to_string(path).unwrap_or_else(|_| panic!("File {} needs to be redable", path));
    let mut reports: Vec<Report> = vec![];

    for line in file.lines() {
        let parts = line.split(" ");
        let mut levels: Vec<i32> = vec![];
        for part in parts {
            let level = part.parse::<i32>().expect("Needs to be an i32");
            levels.push(level);
        }
        reports.push(Report(levels));
    }

    reports
}
