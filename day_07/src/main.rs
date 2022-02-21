use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(filename: &str) -> Vec<i64> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn calc_median(numbers: &Vec<i64>) -> i64 {
    let len = numbers.len();
    if len % 2 == 0 {
        (numbers[len / 2] + numbers[len / 2 - 1]) / 2
    } else {
        numbers[len / 2]
    }
}

fn cost(mid_point: i64, points: &Vec<i64>) -> i64 {
    let average = (points.iter().sum::<i64>() as f32 / points.len() as f32).round() as i64;
    let n = points.len() as i64;
    2 * n * (average - mid_point)
        + points
            .iter()
            .map(|x| {
                if (x - mid_point) < 0 {
                    -1
                } else if (x - mid_point) > 0 {
                    1
                } else {
                    0
                }
            })
            .sum::<i64>() as i64
}

fn find_root_integers(
    numbers: &Vec<i64>,
    cost_function: fn(i64, &Vec<i64>) -> i64,
    optimize_function: fn(i64, &Vec<i64>) -> i64,
    left: i64,
    right: i64,
) -> i64 {
    let (left, right) = if left > right {
        (right, left)
    } else {
        (left, right)
    };
    let f_left = cost_function(left, numbers);
    let f_right = cost_function(right, numbers);
    if f_left * f_right > 0 {
        panic!("Root can not be found in this interval");
    }
    if (left - right).abs() == 1 {
        let opt_left = optimize_function(left, numbers);
        let opt_right = optimize_function(right, numbers);
        if opt_left.abs() < opt_right.abs() {
            return left;
        } else {
            return right;
        }
    }
    let middle = (left + right) / 2;
    let f_middle = cost_function(middle, numbers);
    if f_middle * f_left < 0 {
        return find_root_integers(numbers, cost_function, optimize_function, left, middle);
    } else {
        return find_root_integers(numbers, cost_function, optimize_function, middle, right);
    }
}

fn part1(filename: &str) -> i64 {
    let mut positions = read_input(filename);
    positions.sort();
    let median = calc_median(&positions);
    positions.iter().map(|x| (x - median).abs()).sum()
}

fn part2(filename: &str) -> i64 {
    let mut positions = read_input(filename);
    positions.sort();
    let median = calc_median(&positions);
    let average = positions.iter().sum::<i64>() as f64 / positions.len() as f64;
    let mid_point =
        find_root_integers(&positions, cost, fuel_part2, median, average.round() as i64);
    fuel_part2(mid_point, &positions)
}

fn fuel_part2(mid_point: i64, positions: &Vec<i64>) -> i64 {
    let mut fuel = 0;
    for position in positions {
        let diff = (position - mid_point).abs();
        fuel += (diff * (diff + 1)) / 2;
    }
    fuel
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    // Open the file in read-only mode (ignoring errors).
    let part1_result = part1(&filename);
    println!("Result of part 1: {}", part1_result);

    let part2_result = part2(&filename);
    println!("Result of part 2: {}", part2_result);
}
