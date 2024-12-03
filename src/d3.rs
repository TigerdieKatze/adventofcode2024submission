use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

pub(crate) fn run() {
    let file = File::open("d3-input.txt").expect("Failed to open d3-input.txt");
    let reader = io::BufReader::new(file);

    let mut result: u32 = 0;
    let mut operations_enabled = true;

    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        for cap in re.captures_iter(&line) {
            if let Some(a_match) = cap.get(1) {
                if operations_enabled {
                    let a: u32 = a_match.as_str().parse().expect("Failed to parse number");
                    let b: u32 = cap[2].parse().expect("Failed to parse number");
                    result += a * b;
                }
            } else {
                match cap.get(0).unwrap().as_str() {
                    "do()" => operations_enabled = true,
                    "don't()" => operations_enabled = false,
                    _ => {}
                }
            }
        }
    }

    println!("Day 3:");
    println!("{}", result);
}