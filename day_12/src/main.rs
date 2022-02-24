use ndarray::{Array, Array2};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Cave {
    Start,
    End,
    Small,
    Large,
}

fn parse_file(filename: &str) -> (Array2<u8>, HashMap<usize, Cave>) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut index_to_type: HashMap<usize, Cave> = HashMap::new();
    let mut name_to_index: HashMap<String, usize> = HashMap::new();
    let mut connections: Vec<Vec<usize>> = Vec::new();
    let mut index = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let elements = line.split('-').map(|s| s.to_string());
        let mut connection: Vec<usize> = Vec::with_capacity(2);
        for s in elements {
            let el_index = if name_to_index.contains_key(&s) {
                name_to_index[&s]
            } else {
                name_to_index.insert(s.clone(), index);
                index += 1;
                index - 1
            };
        
            name_to_index.entry(s.clone()).or_insert(index);
            let cave = if s == "start" {
                Cave::Start
            } else if s == "end" {
                Cave::End
            } else if s.chars().all(|c| c.is_uppercase()) {
                Cave::Large
            } else if s.chars().all(|c| c.is_lowercase()) {
                Cave::Small
            } else {
                panic!("Unknown cave type: {}", s);
            };
            index_to_type.insert(el_index, cave);
            connection.push(el_index);
        }
        connections.push(connection);
    }
    let mut result = Array::zeros((index, index));
    for connection in connections {
        let (i, j) = (connection[0], connection[1]);
        result[[i, j]] = 1;
        result[[j, i]] = 1;
    }
    (result, index_to_type)
}

fn part1(filename: &str) -> i32 {
    let (connection_mat, index_to_type) = parse_file(filename);

    0
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    // Open the file in read-only mode (ignoring errors).
    
    let part1_result = part1(&filename);
    println!("Result of part 1: {}", part1_result);

    // let part2_result = part2(&filename);
    // println!("Result of part 2: {}", part2_result);
}
