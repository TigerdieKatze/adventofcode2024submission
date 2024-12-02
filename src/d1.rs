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
        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    println!("Day 1:");

    left.sort();
    right.sort();

    find_difference(&left, &right);
    calculate_similarity_score(&left, &right);
}

fn find_difference(left: &Vec<i32>, right: &Vec<i32>) {
    let mut difference: u32 = 0;

    for i in 0..left.len() {
        difference += (left[i] - right[i]).abs() as u32;
    }

    println!("{}", difference);
}

fn calculate_similarity_score(left: &Vec<i32>, right: &Vec<i32>) {
    let mut score: u32 = 0;

    for i in 0..left.len() {
        let mut count: u32 = 0;
        for j in 0..right.len() {
            if left[i] == right[j] {
                count += 1;
            }
        }

        score += left[i] as u32 * count;
    }

    println!("{}", score);
}