use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(filename: &str, days: usize) -> u64 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let peces = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut estanque = [0 as u64; 9];
    for pez in peces {
        estanque[pez as usize] += 1;
    }
    for day in 0..days {
        estanque[(day + 7) % 9] += estanque[day % 9]
    }
    estanque.iter().sum()
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    // Open the file in read-only mode (ignoring errors).
    let part1_result = part1(&filename, 80);
    println!("Result of part 1: {}", part1_result);

    let part2_result = part1(&filename, 256);
    println!("Result of part 2: {}", part2_result);
}
