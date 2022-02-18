use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

fn part1(filename: &str, window: usize) -> u32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total_descends: u32 = 0;
    let mut depth_vec = vec![0; window+1];

    for (index, line) in reader.lines().enumerate() {
        if index < (window) {
            depth_vec[index+1] = line.unwrap().parse().unwrap();
            continue;
        }
        depth_vec.rotate_left(1);
        depth_vec[window] = line.unwrap().parse().unwrap();
        let old_depth = depth_vec[..window].iter().sum::<i32>();
        let new_depth = depth_vec[1..].iter().sum::<i32>();
        if new_depth > old_depth {
            total_descends += 1;
        }
    }
    total_descends
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    // Open the file in read-only mode (ignoring errors).
    let part1_result = part1(&filename, 1);
    println!("Result of part 1: {}", part1_result);

    let part2_result = part1(&filename, 3);
    println!("Result of part 2: {}", part2_result);
}