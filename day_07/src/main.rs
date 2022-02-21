use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn part1(filename: &str) -> i32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut position: Vec<i32> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    position.sort();
    let total_half = (position.len() as f32 / 2.).floor() as usize;
    let median = if position.len() % 2 == 0 {
        ((position[total_half - 1] + position[total_half]) as f32 / 2.) as i32
    } else {
        position[total_half]
    };
    position.iter().map(|x| (x - median).abs()).sum()
}


fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    // Open the file in read-only mode (ignoring errors).
    
    let part1_result = part1(&filename);
    println!("Result of part 1: {}", part1_result);

    // let part2_result = part1(&filename);
    // println!("Result of part 2: {}", part2_result);
}
