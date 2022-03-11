use std::collections::VecDeque;
use ndarray::{s, Array3};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Cuboid {
    xlims: (i32, i32),
    ylims: (i32, i32),
    zlims: (i32, i32),
}

impl Cuboid {
    fn new() -> Cuboid {
        Cuboid {
            xlims: (0, 0),
            ylims: (0, 0),
            zlims: (0, 0),
        }
    }
    fn len(&self) -> u64 {
        let mut res = (self.xlims.1 - self.xlims.0) as i64;
        res *= (self.ylims.1 - self.ylims.0) as i64;
        res *= (self.zlims.1 - self.zlims.0) as i64;
        if res < 0 {
            panic!("Negative length");
        }
        res as u64
    }
}

#[derive(Debug)]
struct Command {
    value: bool,
    cuboid: Cuboid,
}

impl Command {
    fn new(line: &str) -> Command {
        let mut parts = line.split_whitespace();
        let value = parts.next().unwrap() == "on";
        let mut val_lims = Vec::new();
        for lim in parts.next().unwrap().split(",") {
            let mut aux = Vec::new();
            for val in lim[2..].split("..") {
                aux.push(val.parse::<i32>().unwrap());
            }
            val_lims.push(aux);
        }
        let xlims = (val_lims[0][0], val_lims[0][1] + 1);
        let ylims = (val_lims[1][0], val_lims[1][1] + 1);
        let zlims = (val_lims[2][0], val_lims[2][1] + 1);
        let cuboid = Cuboid {
            xlims: xlims,
            ylims: ylims,
            zlims: zlims,
        };
        Command { value, cuboid }
    }
}

fn parse_file(filename: &str) -> Vec<Command> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut commands = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let cmd = Command::new(&line);
        let cub = &cmd.cuboid;
        if cub.xlims.1 > 51
            || cub.ylims.1 > 51
            || cub.zlims.1 > 51
            || cub.xlims.1 < -50
            || cub.ylims.1 < -50
            || cub.zlims.1 < -50
        {
            // continue;
        }
        commands.push(cmd);
    }
    commands
}

fn part1(commands: &Vec<Command>) -> u32 {
    let mut matrix: Array3<bool> = Array3::from_shape_simple_fn((101, 101, 101), || false);
    for cmd in commands {
        let cub = &cmd.cuboid;
        if cub.xlims.1 > 51
            || cub.ylims.1 > 51
            || cub.zlims.1 > 51
            || cub.xlims.1 < -50
            || cub.ylims.1 < -50
            || cub.zlims.1 < -50
        {
            continue;
        }
        matrix
            .slice_mut(s![
                (cub.xlims.0 + 50)..(cub.xlims.1 + 50),
                (cub.ylims.0 + 50)..(cub.ylims.1 + 50),
                (cub.zlims.0 + 50)..(cub.zlims.1 + 50)
            ])
            .fill(cmd.value);
    }
    matrix.iter().filter(|&x| *x).count() as u32
}

fn make_all_combination_k(N: u32, left: u32, k: u32, result: &mut Vec<Vec<u32>>, tmp: &mut VecDeque<u32>) {
    if k == 0 {
        result.push(tmp.clone().into_iter().collect());
        return;
    }
    for i in left..N {
        tmp.push_back(i);
        make_all_combination_k(N, i + 1, k - 1, result, tmp);
        tmp.pop_back();
    }
}

fn make_all_combination(N: u32) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    let mut tmp = VecDeque::new();
    for k in 1..N+1 {
        make_all_combination_k(N, 0, k, &mut result, &mut tmp);
    }
    result
}


fn part2(commands: &Vec<Command>) -> i64 {
    0
}


fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    let commands = parse_file(&filename);
    println!("{}", commands.len());
    println!("Part 1: {}", part1(&commands));
    
    println!("Part 2: {}", part2(&commands));
    println!("Exp. 2: 2758514936282235");
}
