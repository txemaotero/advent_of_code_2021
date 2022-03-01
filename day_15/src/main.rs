use ndarray::{s, Array2};
use pathfinding::prelude::astar;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_file(filename: &str, part1: bool) -> Array2<i32> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut result = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut row = Vec::new();
        for el in line.chars() {
            row.push(el.to_digit(10).unwrap() as i32);
        }
        result.push(row);
    }
    let mut flat = Vec::new();
    for row in result.iter() {
        for el in row.iter() {
            flat.push(*el);
        }
    }
    let (nrows, ncols) = (result.len(), result[0].len());
    let result = Array2::from_shape_vec((nrows, ncols), flat).unwrap();
    if part1 {
        return result;
    }
    let mut final_res = Array2::zeros((result.shape()[0] * 5, result.shape()[1] * 5));
    for x_ind in 0..5 {
        for y_ind in 0..5 {
            let aux = result.clone().mapv(|a| ((a + x_ind + y_ind - 1) % 9) + 1);
            final_res
                .slice_mut(s![
                    (x_ind as usize * nrows)..((x_ind as usize + 1) * nrows),
                    (y_ind as usize * ncols)..((y_ind as usize + 1) * ncols)
                ])
                .assign(&aux);
        }
    }
    final_res
}

fn neighbor(point: (i32, i32), size: (i32, i32)) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    let (x, y) = point;
    if x > 0 {
        result.push((x - 1, y));
    }
    if x < size.0 - 1 {
        result.push((x + 1, y));
    }
    if y > 0 {
        result.push((x, y - 1));
    }
    if y < size.1 - 1 {
        result.push((x, y + 1));
    }
    result
}

fn part1(filename: &str, is_part1: bool) -> i32 {
    let nodes_mat = parse_file(filename, is_part1);
    let size = (nodes_mat.ncols() as i32, nodes_mat.nrows() as i32);

    let start: (i32, i32) = (0, 0);
    let goal: (i32, i32) = (size.0 - 1, size.1 - 1);
    let result = astar(
        &start,
        |&pos| {
            neighbor(pos, size)
                .into_iter()
                .map(|p| (p, nodes_mat[[p.0 as usize, p.1 as usize]]))
        },
        |&(x, y)| (x - goal.0) ^ 2 + (y - goal.0) ^ 2,
        |&p| p == goal,
    );
    match result {
        Some((_, cost)) => return cost,
        None => panic!("No path found!"),
    }
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    // Open the file in read-only mode (ignoring errors).
    let part1_result = part1(&filename, true);
    println!("Result of part 1: {}", part1_result);

    let part2_result = part1(&filename, false);
    println!("Result of part 2: {}", part2_result);
}
