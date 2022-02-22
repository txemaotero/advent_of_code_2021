use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(filename: &str) -> Vec<Vec<Vec<String>>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut result = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        result.push(parse_line(line))
    }
    result
}

fn parse_line(line: String) -> Vec<Vec<String>> {
    let mut partial = Vec::new();
    for element in line.split("|") {
        let element = element.trim();
        let mut part2 = Vec::new();
        for word in element.split_whitespace() {
            part2.push(word.to_string());
        }
        partial.push(part2);
    }
    partial
}

fn part1(filename: &str) -> i64 {
    let lines = read_input(filename);
    lines
        .iter()
        .map(|line| {
            line[1]
                .iter()
                .map(|word| {
                    if [2, 4, 3, 7].contains(&word.len()) {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i64>()
        })
        .sum::<i64>()
}

fn part2(filename: &str) -> i64 {
    let lines = read_input(filename);
    let mut result = 0;
    for line in lines {
        let decoder = Decoder::new(&line[0]);
        let mut partial_num = String::new();
        for word in &line[1] {
            partial_num += &decoder.decode(word).to_string();
        }
        result += partial_num.parse::<i64>().unwrap();
    }
    result
}

struct Decoder {
    numbers: Vec<HashSet<char>>,
}

impl Decoder {
    fn new(input_info: &Vec<String>) -> Decoder {
        let mut input_info = input_info.clone();
        input_info.sort_by(|a, b| a.len().cmp(&b.len()));

        let mut numbers: Vec<HashSet<char>> = vec![HashSet::new(); 10];

        for digit in input_info {
            if digit.len() == 2 {
                numbers[1] = HashSet::from_iter(digit.chars());
            } else if digit.len() == 3 {
                numbers[7] = HashSet::from_iter(digit.chars());
            } else if digit.len() == 4 {
                numbers[4] = HashSet::from_iter(digit.chars());
            } else if digit.len() == 7 {
                numbers[8] = HashSet::from_iter(digit.chars());
            } else if digit.len() == 5 {
                let aux_set = HashSet::from_iter(digit.chars());
                if numbers[1].is_subset(&aux_set) {
                    numbers[3] = aux_set;
                } else {
                    let aux_4_minus_1: HashSet<_> = numbers[4]
                        .clone()
                        .difference(&numbers[1])
                        .map(|x| x.clone())
                        .collect();
                    if aux_4_minus_1.is_subset(&aux_set) {
                        numbers[5] = aux_set;
                    } else {
                        numbers[2] = aux_set;
                    }
                }
            } else if digit.len() == 6 {
                let aux_set = HashSet::from_iter(digit.chars());
                if numbers[1].is_subset(&aux_set) {
                    let aux_4_minus_1: HashSet<_> = numbers[4]
                        .clone()
                        .difference(&numbers[1])
                        .map(|x| x.clone())
                        .collect();
                    if aux_4_minus_1.is_subset(&aux_set) {
                        numbers[9] = aux_set;
                    } else {
                        numbers[0] = aux_set;
                    }
                } else {
                    numbers[6] = aux_set;
                }
            }
        }
        Decoder { numbers }
    }
    
    fn decode(&self, code: &str) -> i64 {
        let char_set = HashSet::from_iter(code.chars());
        for (index, number) in self.numbers.iter().enumerate() {
            if number == &char_set {
                return index as i64;
            }
        }
        panic!("No number found for {}", code);
    }
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    // Open the file in read-only mode (ignoring errors).
    let part1_result = part1(&filename);
    println!("Result of part 1: {}", part1_result);

    let part2_result = part2(&filename);
    println!("Result of part 2: {}", part2_result);
}
