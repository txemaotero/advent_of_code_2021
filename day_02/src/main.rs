use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn part1(filename: &str) -> i32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut initial_depth: i32 = 0;
    let mut initial_distance: i32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let information = line.split_whitespace().take(2).collect::<Vec<&str>>();
        let direction = information[0]; 
        let amount = information[1].parse::<i32>().unwrap();
        if direction == "forward" {
            initial_distance += amount;
        } else if direction == "up" {
            initial_depth -= amount;
        } else if direction == "down" {
            initial_depth += amount;
        }
    }
    initial_distance * initial_depth
}

fn part2(filename: &str) -> i32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut initial_depth: i32 = 0;
    let mut initial_distance: i32 = 0;
    let mut aim: i32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let information = line.split_whitespace().take(2).collect::<Vec<&str>>();
        let direction = information[0]; 
        let amount = information[1].parse::<i32>().unwrap();
        if direction == "forward" {
            initial_distance += amount;
            initial_depth += amount * aim;
        } else if direction == "up" {
            aim -= amount;
        } else if direction == "down" {
            aim += amount;
        }
    }
    initial_distance * initial_depth
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    // Open the file in read-only mode (ignoring errors).
    let part1_result = part1(&filename);
    println!("Result of part 1: {}", part1_result);

    let part2_result = part2(&filename);
    println!("Result of part 2: {}", part2_result);
}
