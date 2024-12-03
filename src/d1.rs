use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub(crate) fn run() {
    let file = File::open("d1-input.txt").expect("Failed to open input.txt");
    let reader = io::BufReader::new(file);

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();
        if numbers.len() >= 2 {
            left.push(numbers[0]);
            right.push(numbers[1]);
        }
    }

    println!("Day 1:");

    left.sort();
    right.sort();

    let difference = find_difference(&left, &right);
    println!("{}", difference);

    let similarity_score = calculate_similarity_score(&left, &right);
    println!("{}", similarity_score);
}

fn find_difference(left: &[i32], right: &[i32]) -> u32 {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs() as u32)
        .sum()
}

fn calculate_similarity_score(left: &[i32], right: &[i32]) -> u32 {
    let mut right_counts: HashMap<i32, u32> = HashMap::new();
    for &num in right {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    left.iter()
        .map(|&num| num as u32 * right_counts.get(&num).cloned().unwrap_or(0))
        .sum()
}
