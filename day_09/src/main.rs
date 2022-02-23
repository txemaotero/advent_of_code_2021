use std::collections::{VecDeque, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(filename: &str) -> i32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut three_lines: VecDeque<Vec<i32>> = VecDeque::new();
    let mut result = 0;
    for (index, line) in reader.lines().enumerate() {
        let line = line
            .unwrap()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>();
        let len_lin = line.len();
        if index == 0 {
            let aux_vec = vec![10; len_lin];
            three_lines.push_front(aux_vec);
            three_lines.push_front(line);
            continue;
        } else if index == 1 {
            three_lines.push_front(line);
        } else {
            three_lines.push_front(line);
            three_lines.pop_back();
        }
        let minima = find_minima_three(&three_lines);
        result += minima.iter().sum::<i32>() + minima.len() as i32;
    }

    let aux_vec = vec![10; three_lines[0].len()];
    three_lines.push_front(aux_vec);
    three_lines.pop_back();
    let minima = find_minima_three(&three_lines);
    result += minima.iter().sum::<i32>() + minima.len() as i32;

    result
}

fn find_minima_three(three_lines: &VecDeque<Vec<i32>>) -> Vec<i32> {
    let mut minima: Vec<i32> = Vec::new();
    let len_line = three_lines[1].len();
    for (index, number) in three_lines[1].iter().enumerate() {
        let cond1 = number < &three_lines[0][index] && number < &three_lines[2][index];
        if index == 0 {
            if cond1 && number < &three_lines[1][index + 1] {
                minima.push(*number);
            }
        } else if index == len_line - 1 {
            if cond1 && number < &three_lines[1][index - 1] {
                minima.push(*number);
            }
        } else {
            let cond2 = number < &three_lines[1][index - 1] && number < &three_lines[1][index + 1];
            if cond1 && cond2 {
                minima.push(*number);
            }
        }
    }
    minima
}

fn part2(filename: &str) -> i32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    0
}

struct Node {
    x: i32,
    y: i32,
    height: i32,
    connections: HashSet<Node>,
}

impl Node {
    fn new(x: i32, y: i32, height: i32) -> Node {
        Node {
            x,
            y,
            height,
            connections: HashSet::new(),
        }
    }
    
    fn connect(&mut self, node: Node) {
        self.connections.push(node);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Node {}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    // Open the file in read-only mode (ignoring errors).
    let part1_result = part1(&filename);
    println!("Result of part 1: {}", part1_result);

    // let part2_result = part2(&filename);
    // println!("Result of part 2: {}", part2_result);
}
