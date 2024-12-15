use std::fs::read_to_string;

use aoc2::report_safety::{is_safe, is_safe_dampened, is_safe_normal, Report};

fn main() {
    let reports = read_reports_from_file(concat!(std::env!("CARGO_MANIFEST_DIR"), "/", "resources/input_one"));
    let safe_reports = reports.iter().map(|report| is_safe(report, is_safe_normal)).filter(|is_safe| *is_safe);
    println!("{} reports were safe", safe_reports.count());

    let reports = reports.iter().zip(reports.iter());
    let filtered_reports = reports.map(|reports| (reports.0, is_safe(reports.1, is_safe_dampened)));
    let filtered_reports: Vec<(&Report, bool)> = filtered_reports.filter(|reports| reports.1).collect();

    println!("{}", filtered_reports.len());

    
}

fn read_reports_from_file(path: &str) -> Vec<Report> {
    let file = read_to_string(path).unwrap_or_else(|_| panic!("File {} needs to be readable", path));
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
