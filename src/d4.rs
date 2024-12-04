use std::fs::File;
use std::io::{self, BufRead};

pub(crate) fn run() {
    let file = File::open("d4-input.txt").expect("Failed to open d4-input.txt");
    let reader = io::BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    println!("Day 4:");

    let mut result: u32 = 0;

    let nrows = grid.len();
    if nrows == 0 {
        println!("0");
        return;
    }
    let ncols = grid[0].len();

    let word = ['X', 'M', 'A', 'S'];
    let directions = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (-1, -1), // up-left
        (1, -1),  // down-left
        (-1, 1),  // up-right
    ];

    for i in 0..nrows {
        for j in 0..ncols {
            for &(dx, dy) in &directions {
                let mut found = true;
                for k in 0..4 {
                    let k_isize = k as isize;
                    let x = i as isize + k_isize * dx;
                    let y = j as isize + k_isize * dy;
                    if x < 0 || x >= nrows as isize || y < 0 || y >= ncols as isize {
                        found = false;
                        break;
                    }
                    if grid[x as usize][y as usize] != word[k] {
                        found = false;
                        break;
                    }
                }
                if found {
                    result += 1;
                }
            }
        }
    }

    println!("{}", result);
}
