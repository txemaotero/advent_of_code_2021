use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use ndarray::{Array, Array2};
use std::collections::VecDeque;

fn parse_file(filename: &str) -> Array2<u8> {
    let mut result = Array::zeros((10, 10));
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for (row_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (col_index, light) in line.chars().enumerate() {
            let light = light.to_digit(10).unwrap() as u8;
            result[[row_index, col_index]] = light;
        }
    }
    result
}

fn part1(filename: &str, days: i32) -> i32 {
    let mut octopus = parse_file(filename);
    dbg!(&octopus);
    for day in 0..days {
        apply_day(&mut octopus);
    }
    dbg!(&octopus);
    0
}

fn apply_day(matrix: &mut Array2<u8>) {
    *matrix += 1;
    let mut remaining: VecDeque<(usize, usize)> = VecDeque::new();
    for i in 0..matrix.nrows() {
        for j in 0..matrix.ncols() {
            if matrix[[i, j]] > 9 {
                remaining.push_back((i, j));
            }
        }
    }
    
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    // Open the file in read-only mode (ignoring errors).
    let part1_result = part1(&filename, 10);
    println!("Result of part 1: {}", part1_result);

    // let part2_result = part2(&filename);
    // println!("Result of part 2: {}", part2_result);
}
