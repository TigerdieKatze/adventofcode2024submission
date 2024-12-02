use std::fs::File;
use std::io::{self, BufRead};

pub(crate) fn run() {
    let file = File::open("d2-input.txt").expect("Failed to open input.txt");
    let reader = io::BufReader::new(file);

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();
        reports.push(report);
    }

    println!("Day 2:");

    find_safe_reports(&reports, false);
    find_safe_reports(&reports, true);

}

fn find_safe_reports(reports: &Vec<Vec<i32>>, enable_problem_dampner: bool) {
    let mut safe_reports: u32 = 0;
    let mut unsafe_reports: Vec<Vec<i32>> = Vec::new();

    for i in 0..reports.len() {
        if !is_increasing_or_decreasing(&reports[i]) {
            unsafe_reports.push(reports[i].clone());
            continue;
        }

        if !are_adjacent_level_differences_correct(&reports[i]) {
            unsafe_reports.push(reports[i].clone());
            continue;
        }

        safe_reports += 1;
    }

    if enable_problem_dampner {
        for i in 0..unsafe_reports.len() {
            if can_be_saved_by_problem_dampner(&unsafe_reports[i]) {
                safe_reports += 1;
            }      
        }
    }
    

    println!("{}", safe_reports);
}

fn can_be_saved_by_problem_dampner(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let mut report_copy = report.clone();
        report_copy.remove(i);

        if is_increasing_or_decreasing(&report_copy) && are_adjacent_level_differences_correct(&report_copy) {
            return true;
        }
    }

    false
}

fn is_increasing_or_decreasing(report: &Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..report.len() - 1 {
        if report[i] > report[i + 1] {
            increasing = false;
        } else if report[i] < report[i + 1] {
            decreasing = false;
        }
    }

    increasing || decreasing
}

fn are_adjacent_level_differences_correct(report: &Vec<i32>) -> bool {
    const MIN_DIFF: i32 = 1;
    const MAX_DIFF: i32 = 3;

    for i in 0..report.len() - 1 {
        if (report[i] - report[i + 1]).abs() < MIN_DIFF || (report[i] - report[i + 1]).abs() > MAX_DIFF {
            return false;
        }
    }

    return true;
}