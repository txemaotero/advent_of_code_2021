use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

fn part1(filename: &str) -> i32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut total_entries: u32 = 0;
    let mut counter_vec = vec::Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let entry = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as f32)
            .collect::<Vec<f32>>();

        if index == 0 {
            counter_vec = vec![0 as f32; entry.len()];
        }
        counter_vec = entry
            .iter()
            .zip(counter_vec.iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<f32>>();
        total_entries += 1;
    }
    let gamma: String = counter_vec
        .iter()
        .map(|x| (((x / total_entries as f32) > 0.5) as u32).to_string())
        .collect();
    let epsilon: String = counter_vec
        .iter()
        .map(|x| (((x / total_entries as f32) < 0.5) as u32).to_string())
        .collect();
    let gamma_int = i32::from_str_radix(&gamma, 2).expect("Not a binary number!");
    let epsilon_int = i32::from_str_radix(&epsilon, 2).expect("Not a binary number!");
    gamma_int * epsilon_int
}

fn part2(filename: &str) -> i32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let all_entries = reader.lines().map(|x| x.unwrap().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let oxygen = find_entry(&all_entries, ToMatch::MOST, 0);
    let co2 = find_entry(&all_entries, ToMatch::LEAST, 0);
    let oxygen_int = i32::from_str_radix(&oxygen, 2).expect("Not a binary number!");
    let co2_int = i32::from_str_radix(&co2, 2).expect("Not a binary number!");
    oxygen_int * co2_int
}

fn find_entry(entries: &Vec<Vec<char>>, to_match: ToMatch, index: i32) -> String {
    if entries.len() == 1 {
        return entries[0].iter().collect::<String>();
    }
    if index == entries.len() as i32 {
        return "".to_string();
    }
    let mut g0: Vec<Vec<char>> = vec::Vec::new();
    let mut g1: Vec<Vec<char>> = vec::Vec::new();
    for entry in entries {
        if entry[index as usize] == '1' {
            g1.push(entry.clone());
        } else {
            g0.push(entry.clone());
        }
    }
    match to_match {
        ToMatch::MOST => {
            if g0.len() > g1.len() {
                return find_entry(&g0, to_match, index + 1);
            } else if g0.len() < g1.len() {
                return find_entry(&g1, to_match, index + 1);
            } else {
                return find_entry(&g1, to_match, index + 1);
            }
        },
        ToMatch::LEAST => {
            if g0.len() < g1.len() {
                return find_entry(&g0, to_match, index + 1);
            } else if g0.len() > g1.len() {
                return find_entry(&g1, to_match, index + 1);
            } else {
                return find_entry(&g0, to_match, index + 1);
            }
        }
    }
}

enum ToMatch {
    MOST,
    LEAST
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    // Open the file in read-only mode (ignoring errors).
    let part1_result = part1(&filename);
    println!("Result of part 1: {}", part1_result);

    let part2_result = part2(&filename);
    println!("Result of part 2: {}", part2_result);
}
