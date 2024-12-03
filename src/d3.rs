use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

pub(crate) fn run() {
    let file = File::open("d3-input.txt").expect("Failed to open input.txt");
    let reader = io::BufReader::new(file);

    let mut operations: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
        for cap in re.find_iter(&line) {
            operations.push(cap.as_str().to_string());
        }
    }

    println!("Day 3:");

    let mut result: u32 = 0;

    for operation in operations {
        let nums: Vec<u32> = operation
            .split(|c| c == '(' || c == ')' || c == ',')
            .filter_map(|s| s.parse().ok())
            .collect();
        result += mul(nums[0], nums[1]);
    }

    println!("{}", result);
}

fn mul(a: u32, b: u32) -> u32 {
    a * b
}