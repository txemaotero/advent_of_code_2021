use ndarray::{Array, Array2, Axis};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

fn non_zero_index(connection_mat: &Array2<u8>, start: usize) -> Vec<usize> {
    connection_mat
        .index_axis(Axis(0), start)
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == 1)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>()
}

fn find_paths_part2(
    mut connection_mat: Array2<u8>,
    index_to_type: &HashMap<usize, Cave>,
    start: usize,
    result: &mut i32,
    mut visit_counter: HashMap<usize, u32>,
) {
    if let Cave::End = index_to_type[&start] {
        *result += 1;
        return;
    }
    if connection_mat.index_axis(Axis(0), start).sum() == 0 {
        return;
    }
    let new_starts = non_zero_index(&connection_mat, start);
    match index_to_type[&start] {
        Cave::Start => {
            connection_mat.index_axis_mut(Axis(0), start).fill(0);
            connection_mat.index_axis_mut(Axis(1), start).fill(0);
        }
        Cave::Small => {
            *visit_counter.get_mut(&start).unwrap() += 1;
            let count = visit_counter.iter().filter(|(_, &v)| v > 1).count();
            if count == 1 {
                connection_mat.index_axis_mut(Axis(0), start).fill(0);
                connection_mat.index_axis_mut(Axis(1), start).fill(0);
            } else if count > 1 {
                return;
            }
        }
        _ => {}
    };
    for new_start in new_starts {
        find_paths_part2(
            connection_mat.clone(),
            index_to_type,
            new_start,
            result,
            visit_counter.clone(),
        );
    }
}

fn find_paths(
    mut connection_mat: Array2<u8>,
    index_to_type: &HashMap<usize, Cave>,
    start: usize,
    result: &mut i32,
) {
    if let Cave::End = index_to_type[&start] {
        *result += 1;
        return;
    }
    if connection_mat.index_axis(Axis(0), start).sum() == 0 {
        return;
    }
    let new_starts = non_zero_index(&connection_mat, start);
    match index_to_type[&start] {
        Cave::Start | Cave::Small => {
            connection_mat.index_axis_mut(Axis(0), start).fill(0);
            connection_mat.index_axis_mut(Axis(1), start).fill(0);
        }
        _ => {}
    };
    for new_start in new_starts {
        find_paths(
            connection_mat.clone(),
            index_to_type,
            new_start,
            result,
        );
    }
}

fn part1(filename: &str) -> i32 {
    let (connection_mat, index_to_type) = parse_file(filename);
    let mut result = 0;
    let start_ind = index_to_type
        .iter()
        .find_map(|(ind, v)| match v {
            Cave::Start => Some(*ind),
            _ => None,
        })
        .unwrap();
    find_paths(
        connection_mat,
        &index_to_type,
        start_ind,
        &mut result,
    );
    result
}

fn part2(filename: &str) -> i32 {
    let (connection_mat, index_to_type) = parse_file(filename);
    let mut result = 0;
    let start_ind = index_to_type
        .iter()
        .find_map(|(ind, v)| match v {
            Cave::Start => Some(*ind),
            _ => None,
        })
        .unwrap();
    let visit_counter: HashMap<usize, u32> =
        index_to_type.iter().map(|(ind, _)| (*ind, 0)).collect();
    find_paths_part2(
        connection_mat,
        &index_to_type,
        start_ind,
        &mut result,
        visit_counter,
    );
    result
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    // Open the file in read-only mode (ignoring errors).
    let part1_result = part1(&filename);
    println!("Result of part 1: {}", part1_result);

    let part2_result = part2(&filename);
    println!("Result of part 2: {}", part2_result);
}
