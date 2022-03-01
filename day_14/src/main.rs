use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


fn parse_file(filename: &str) -> (Vec<char>, HashMap<(char, char), char>){
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut template = Vec::new();
    let mut rules = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains("->") {
            let cmd: Vec<&str> = line.split(" -> ").collect();
            rules.insert((cmd[0].chars().next().unwrap(), cmd[0].chars().last().unwrap()), cmd[1].chars().next().unwrap());
        } else if line.trim().is_empty() {
            continue;
        } else {
            template = line.chars().collect();
        }

    }
    (template, rules)
}

fn part1(filename: &str, iters: u32) -> u64 {
    let (template, rules) = parse_file(filename);

    let mut pairs_counter: HashMap<(char, char), u64> = HashMap::new();
    let (tem_start, tem_end) = (template.first().unwrap(), template.last().unwrap());
    for (start, end) in get_pairs(&template) {
        *pairs_counter.entry((start, end)).or_insert(0) += 1;
    }
    for _ in 0..iters {
        let mut aux_counter: HashMap<(char, char), u64> = HashMap::new();
        for ((p_start, p_end), val) in pairs_counter.iter() {
            let new_el = rules.get(&(*p_start, *p_end)).unwrap();
            *aux_counter.entry((*p_start, *new_el)).or_insert(0) += val;
            *aux_counter.entry((*new_el, *p_end)).or_insert(0) += val;
        }
        pairs_counter = aux_counter;
    }
    let mut counter: HashMap<char, u64> = HashMap::new();
    for ((start, end), val) in pairs_counter.iter() {
        *counter.entry(*start).or_insert(0) += val;
        *counter.entry(*end).or_insert(0) += val;
    }
    *counter.entry(*tem_start).or_insert(0) += 1;
    *counter.entry(*tem_end).or_insert(0) += 1;


    let max_val = counter.values().max().unwrap();
    let min_val = counter.values().min().unwrap();
    (max_val - min_val) / 2 
}

fn get_pairs(vector: &Vec<char>) -> Vec<(char, char)> {
    let mut pairs = Vec::new();
    for ind in 0..vector.len() - 1 {
        pairs.push((vector[ind], vector[ind + 1]));
    }
    pairs
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    // Open the file in read-only mode (ignoring errors).
    let part1_result = part1(&filename, 10);
    println!("Result of part 1: {}", part1_result);

    let part2_result = part1(&filename, 40);
    println!("Result of part 2: {}", part2_result);
}

