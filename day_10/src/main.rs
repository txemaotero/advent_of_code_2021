use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn penalization() -> HashMap<char, i32> {
    let mut map = HashMap::new();
    map.insert(')', 3);
    map.insert(']', 57);
    map.insert('}', 1197);
    map.insert('>', 25137);
    map
}

fn open_symbol() -> HashMap<char, i32> {
    let mut map = HashMap::new();
    map.insert('(', 1);
    map.insert('[', 2);
    map.insert('{', 3);
    map.insert('<', 4);
    map
}

fn close_symbol() -> HashMap<char, i32> {
    let mut map = HashMap::new();
    map.insert(')', 1);
    map.insert(']', 2);
    map.insert('}', 3);
    map.insert('>', 4);
    map
}

fn part1(filename: &str) -> i64 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut final_result = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let result = parse_line(line);
        match result {
            Err(value) => {
                final_result += value as i64;
            }
            Ok(_) => {}
        }
    }
    final_result
}

fn part2(filename: &str) -> i64 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut final_result: Vec<i64> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let result = parse_line(line);
        match result {
            Err(_) => {}
            Ok(OkLine::Incomplete(penal)) => {
                final_result.push(penal);
            }
            Ok(_) => {}
        }
    }
    final_result.sort();
    let index = (final_result.len() - 1) / 2;
    final_result[index]
}

fn parse_line(line: String) -> Result<OkLine, i32> {
    let mut aux_open_symbol: VecDeque<i32> = VecDeque::new();
    let open = open_symbol();
    let close = close_symbol();
    let penal = penalization();
    for char in line.chars() {
        match open.get(&char) {
            Some(value) => {
                aux_open_symbol.push_back(*value);
                continue;
            }
            None => {}
        };
        match close.get(&char) {
            Some(value) => {
                let last = aux_open_symbol.pop_back().unwrap();
                if last != *value {
                    let penalty = match penal.get(&char) {
                        Some(value) => *value,
                        None => panic!("No penalty for closing symbol {}", char),
                    };
                    return Err(penalty);
                }
                continue;
            }
            None => {
                panic!("Unknown character: {}", char);
            }
        };
    }
    if aux_open_symbol.len() > 0 {
        let mut score: i64 = 0;
        while let Some(last_num) = aux_open_symbol.pop_back() {
            score *= 5;
            score += last_num as i64;
        }
        return Ok(OkLine::Incomplete(score));
    }
    return Ok(OkLine::Complete);
}

enum OkLine {
    Complete,
    Incomplete(i64),
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    // Open the file in read-only mode (ignoring errors).
    let part1_result = part1(&filename);
    println!("Result of part 1: {}", part1_result);

    let part2_result = part2(&filename);
    println!("Result of part 2: {}", part2_result);
}
