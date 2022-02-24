use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{BinaryHeap, VecDeque};


fn parse_file(filename: &str) -> Vec<Vec<u8>>{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut nodes = Vec::new();
    let mut size = None;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut aux_row = Vec::new();
        aux_row.push(9);
        match size {
            None => {
                let aux_size = line.len();
                nodes.push(vec![9; aux_size + 2]);
                size = Some(aux_size);
            }
            Some(_) => {}
        }
        for height in line.chars() {
            let height = height.to_digit(10).unwrap() as u8;
            aux_row.push(height);
        }
        aux_row.push(9);
        nodes.push(aux_row);
    }
    nodes.push(vec![9; size.unwrap() + 2]);
    nodes
}

fn part1(filename: &str) -> i32 {
    let mut result = 0;
    let nodes = parse_file(filename);
    let len_lin = nodes[0].len();
    for index in 1..len_lin - 1 {
        let minima = find_minima_three(&nodes[index-1..index + 2].to_vec());
        result += minima.iter().sum::<u8>() as i32 + minima.len() as i32;
    }
    result
}

fn find_minima_three(three_lines: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut minima: Vec<u8> = Vec::new();
    let len_line = three_lines[1].len();
    for index in 1..len_line - 1 {
        let number = three_lines[1][index];
        let cond1 = number < three_lines[0][index] && number < three_lines[2][index];
        let cond2 = number < three_lines[1][index-1] && number < three_lines[1][index+1];
        if cond1 && cond2 {
            minima.push(number);
        }
    }
    minima
}

fn part2(filename: &str) -> i32 {
    let mut nodes = parse_file(filename);
    let (n_rows, n_cols) = (nodes.len(), nodes[0].len());
    let mut max_size: BinaryHeap<i32> = BinaryHeap::new();
    for i in 0..n_rows {
        for j in 0..n_cols {
            let height = nodes[i][j];
            if height == 9 {
                continue;
            }
            let mut size = 0;
            let mut remaining_nodes: VecDeque<(usize, usize)> = VecDeque::new();
            remaining_nodes.push_front((i, j));
            while let Some((it, jt)) = remaining_nodes.pop_front() {
                for (i_nei, j_nei) in neighbors(it, jt) {
                    if nodes[i_nei][j_nei] == 9 {
                        continue;
                    }
                    nodes[i_nei][j_nei] = 9;
                    remaining_nodes.push_back((i_nei, j_nei));
                    size += 1;
                }
            }
            max_size.push(size as i32);
        }
    }
    let mut result = 1;
    for _ in 0..3 {
        result *= max_size.pop().unwrap();
    }
    result
}

fn neighbors(i: usize, j: usize) -> [(usize, usize); 4] {
    [(i-1, j), (i+1, j), (i, j-1), (i, j+1)]
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    // Open the file in read-only mode (ignoring errors).
    let part1_result = part1(&filename);
    println!("Result of part 1: {}", part1_result);

    let part2_result = part2(&filename);
    println!("Result of part 2: {}", part2_result);
}
