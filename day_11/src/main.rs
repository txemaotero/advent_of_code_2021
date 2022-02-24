use ndarray::{Array, Array2};
use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let mut total_lights = 0;
    for _ in 0..days {
        total_lights += apply_day(&mut octopus);
    }
    total_lights
}

fn part2(filename: &str) -> i32 {
    let mut octopus = parse_file(filename);
    let elements = (octopus.nrows() * octopus.ncols()) as i32;
    let mut day = 0;
    loop {
        day += 1;
        let lights = apply_day(&mut octopus);
        if lights == elements {
            return day;
        }
    }
}

fn apply_day(matrix: &mut Array2<u8>) -> i32 {
    *matrix += 1;
    let mut remaining: VecDeque<(usize, usize)> = VecDeque::new();
    let (nrows, ncols) = matrix.dim();
    for i in 0..nrows {
        for j in 0..ncols {
            if matrix[[i, j]] > 9 {
                remaining.push_back((i, j));
            }
        }
    }
    let mut changed: HashSet<(usize, usize)> = HashSet::new();
    let mut light: i32 = 0;
    while let Some((i, j)) = remaining.pop_front() {
        matrix[[i, j]] = 0;
        light += 1;
        changed.insert((i, j));

        for (i, j) in get_neighbors(i as isize, j as isize, nrows as isize, ncols as isize) {
            if changed.contains(&(i, j)) || remaining.contains(&(i, j)) {
                continue;
            }
            matrix[[i, j]] += 1;
            if matrix[[i, j]] > 9 {
                remaining.push_back((i, j));
            }
        }
    }
    light
}

fn get_neighbors(i: isize, j: isize, nrows: isize, ncols: isize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    for ind_x in -1..2 {
        for ind_y in -1..2 {
            if ind_x == 0 && ind_y == 0 {
                continue;
            } else if i + ind_x < nrows && j + ind_y < ncols && i + ind_x >= 0 && j + ind_y >= 0 {
                neighbors.push(((i + ind_x) as usize, (j + ind_y) as usize));
            }
        }
    }
    neighbors
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    // Open the file in read-only mode (ignoring errors).
    let part1_result = part1(&filename, 100);
    println!("Result of part 1: {}", part1_result);

    let part2_result = part2(&filename);
    println!("Result of part 2: {}", part2_result);
}
