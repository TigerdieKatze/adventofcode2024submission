use std::fs::File;
use std::io::{self, BufRead};

pub(crate) fn run() {
    let file = File::open("d2-input.txt").expect("Failed to open d2-input.txt");
    let reader = io::BufReader::new(file);

    let reports: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| {
            line.expect("Failed to read line")
                .split_whitespace()
                .map(|s| s.parse().expect("Failed to parse number"))
                .collect()
        })
        .collect();

    println!("Day 2:");

    find_safe_reports(&reports, false);
    find_safe_reports(&reports, true);
}

fn find_safe_reports(reports: &[Vec<i32>], enable_problem_dampener: bool) {
    let mut safe_reports = 0;

    for report in reports {
        if is_report_safe(report) {
            safe_reports += 1;
        } else if enable_problem_dampener && can_be_saved_by_problem_dampner(report) {
            safe_reports += 1;
        }
    }

    println!("{}", safe_reports);
}

fn is_report_safe(report: &[i32]) -> bool {
    is_report_safe_with_optional_skip(report, None)
}

fn can_be_saved_by_problem_dampner(report: &[i32]) -> bool {
    for i in 0..report.len() {
        if is_report_safe_with_optional_skip(report, Some(i)) {
            return true;
        }
    }
    false
}

fn is_report_safe_with_optional_skip(report: &[i32], skip_idx: Option<usize>) -> bool {
    if report.len() - skip_idx.map_or(0, |_| 1) < 2 {
        // there are less than two elements, so it is safe by default
        return true;
    }

    let mut increasing = true;
    let mut decreasing = true;
    let mut prev_value = None;

    for (idx, &value) in report.iter().enumerate() {
        if skip_idx == Some(idx) {
            continue;
        }
        if let Some(prev) = prev_value {
            if value > prev {
                decreasing = false;
            } else if value < prev {
                increasing = false;
            } else {
                increasing = false;
                decreasing = false;
            }
            if !increasing && !decreasing {
                return false;
            }
        }
        prev_value = Some(value);
    }

    const MIN_DIFF: i32 = 1;
    const MAX_DIFF: i32 = 3;
    prev_value = None;

    for (idx, &value) in report.iter().enumerate() {
        if skip_idx == Some(idx) {
            continue;
        }
        if let Some(prev) = prev_value {
            let diff = (prev - value).abs();
            if diff < MIN_DIFF || diff > MAX_DIFF {
                return false;
            }
        }
        prev_value = Some(value);
    }

    true
}
