use ndarray::{s, Array1, Array3};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Clone)]
struct Cuboid {
    xlims: (i32, i32),
    ylims: (i32, i32),
    zlims: (i32, i32),
}

impl Cuboid {
    fn all_limits(&self) -> Vec<i32> {
        vec![
            self.xlims.0,
            self.xlims.1,
            self.ylims.0,
            self.ylims.1,
            self.zlims.0,
            self.zlims.1,
        ]
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
    fn overlaps(&self, other: &Self) -> bool {
        let xoverlap = self.xlims.0 <= other.xlims.1 && other.xlims.0 <= self.xlims.1;
        let yoverlap = self.ylims.0 <= other.ylims.1 && other.ylims.0 <= self.ylims.1;
        let zoverlap = self.zlims.0 <= other.zlims.1 && other.zlims.0 <= self.zlims.1;
        xoverlap && yoverlap && zoverlap
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

fn part2(commands: &Vec<Command>) -> i64 {
    let mut result = 0;
    let cmd_len = commands.len();
    let overlaps = get_overlaps(commands);

    for (cmd_ind, cmd) in commands.iter().enumerate() {
        print!("\r{} of {}", cmd_ind + 1, cmd_len);
        std::io::stdout().flush().ok();
        if cmd.value {
            result += cmd.cuboid.len() as i64;
            let inter_cont = intersection_contribution(vec![cmd_ind], commands, &overlaps);
            result -= inter_cont;
        }
    }
    println!("");
    result
}

fn intersection_contribution(
    indexes: Vec<usize>,
    commands: &Vec<Command>,
    overlaps: &HashMap<usize, HashSet<usize>>,
) -> i64 {
    let last = indexes[indexes.len() - 1];
    let connections = overlaps.get(&last).unwrap();
    if connections.is_empty() {
        return 0;
    }
    let mut result = 0;
    for conn in connections {
        let mut all_ind = indexes.clone();
        all_ind.push(*conn);
        result += get_overlap_len(&all_ind, commands) as i64;
        result -= intersection_contribution(all_ind, commands, overlaps);
    }
    result
}

fn get_overlap_len(indexes: &Vec<usize>, commands: &Vec<Command>) -> u64 {
    let mut limits = Array1::<i32>::from_vec(commands[indexes[0]].cuboid.all_limits().to_vec());
    for ind in indexes.iter().skip(1) {
        let new_lims = &commands[*ind].cuboid.all_limits();
        for dim in [0, 2, 4] {
            limits[dim] = limits[dim].max(new_lims[dim]);
            limits[dim + 1] = limits[dim + 1].min(new_lims[dim + 1]);
        }
    }
    let mut result = 1;
    for dim in [0, 2, 4] {
        if limits[dim] > limits[dim + 1] {
            return 0;
        }
        result *= (limits[dim + 1] - limits[dim]) as u64;
    }
    result
}

fn get_overlaps(commands: &Vec<Command>) -> HashMap<usize, HashSet<usize>> {
    let mut result = HashMap::new();
    for (ind, cmd) in commands.iter().enumerate() {
        result.insert(ind, HashSet::new());
        for j in ind + 1..commands.len() {
            if cmd.cuboid.overlaps(&commands[j].cuboid) {
                result.get_mut(&ind).unwrap().insert(j);
            }
        }
    }
    result
}
fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    let commands = parse_file(&filename);

    println!("Part 1: {}", part1(&commands));
    println!("Part 2: {}", part2(&commands));
}
