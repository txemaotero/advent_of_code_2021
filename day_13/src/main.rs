use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;


fn parse_file(filename: &str) -> (Vec<Vec<i32>>, Vec<(String, i32)>){
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut points = Vec::new();
    let mut commands = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains("fold") {
            let cmd = line.split_whitespace().last().unwrap().split("=").collect::<Vec<&str>>();
            commands.push((cmd[0].to_string(), cmd[1].parse::<i32>().unwrap()));
        } else if line.trim().is_empty() {
            continue;
        } else {
            points.push(line.split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>());
        }

    }
    (points, commands)
}

fn apply_fold(points: Vec<Vec<i32>>, command: &(String, i32)) -> Vec<Vec<i32>> {
    let mut new_points = HashSet::new();
    let (direction, amount) = command;
    for point in points {
        let mut coord = point[0];
        if direction == "y" {
            coord = point[1];
        }
        coord -= amount;
        if coord > 0 {
            coord = -coord;
        } else if coord == 0 {
            continue;
        }
        coord += amount;
        if direction == "x" {
            new_points.insert((coord, point[1]));
        } else {
            new_points.insert((point[0], coord));
        }
    }
    new_points.iter().map(|(x, y)| Vec::from([*x, *y])).collect()
}

fn part1(filename: &str) -> usize {
    let (points, commands) = parse_file(filename);
    let new_points = apply_fold(points, &commands[0]);
    new_points.len()
}

fn part2(filename: &str) -> usize {
    let (mut points, commands) = parse_file(filename);
    for command in commands {
        points = apply_fold(points, &command);
    }
    represent(&points);
    points.len()
}

fn represent(points: &Vec<Vec<i32>>) {
    let mut min_x = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut min_y = std::i32::MAX;
    let mut max_y = std::i32::MIN;
    for point in points {
        min_x = std::cmp::min(min_x, point[0]);
        max_x = std::cmp::max(max_x, point[0]);
        min_y = std::cmp::min(min_y, point[1]);
        max_y = std::cmp::max(max_y, point[1]);
    }
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut is_point = false;
            for point in points {
                if point[0] == x && point[1] == y {
                    is_point = true;
                    break;
                }
            }
            if is_point {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
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
