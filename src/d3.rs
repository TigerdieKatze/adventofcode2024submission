use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

pub(crate) fn run() {
    let file = File::open("d3-input.txt").expect("Failed to open input.txt");
    let reader = io::BufReader::new(file);

    let mut operations: Vec<String> = Vec::new();
    let mut operations_enabled = true;
    
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();

        for cap in re.captures_iter(&line) {
            let cap_str = cap.get(0).unwrap().as_str();
            if cap_str == "do()" {
                operations_enabled = true;
                continue;
            } else if cap_str == "don't()" {
                operations_enabled = false;
                continue;
            }

            if !operations_enabled {
                continue;
            }

            operations.push(cap_str.to_string());
        }
    }

    println!("Day 3:");

    let mut result: u32 = 0;

    for operation in operations {
        let nums = operation
            .split(|c| c == '(' || c == ')' || c == ',')
            .filter(|s| !s.is_empty())
            .skip(1)
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect::<Vec<u32>>();
        result += mul(nums[0], nums[1]);
    }

    println!("{}", result);
}

fn mul(a: u32, b: u32) -> u32 {
    a * b
}