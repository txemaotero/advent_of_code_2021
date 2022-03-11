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
    sign: i32
}

impl Cuboid {
    fn new() -> Cuboid {
        Cuboid {
            xlims: (0, 0),
            ylims: (0, 0),
            zlims: (0, 0),
            sign: 1
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
    fn intersection(&self, other: &Self) -> Cuboid {
        let lims = [
            (
                self.xlims.0.max(other.xlims.0),
                self.xlims.1.min(other.xlims.1),
            ),
            (
                self.ylims.0.max(other.ylims.0),
                self.ylims.1.min(other.ylims.1),
            ),
            (
                self.zlims.0.max(other.zlims.0),
                self.zlims.1.min(other.zlims.1),
            ),
        ];
        for lim in lims {
            if lim.0 > lim.1 {
                return Cuboid {
                    xlims: (0, 0),
                    ylims: (0, 0),
                    zlims: (0, 0),
                    sign: 1,
                };
            }
        }
        Cuboid {
            xlims: lims[0],
            ylims: lims[1],
            zlims: lims[2],
            sign: self.sign * other.sign,
        }
    }
    fn negate(&mut self) {
        self.sign *= -1;
    }
}

#[derive(Debug)]
struct CuboidOperation {
    cuboids_list: Vec<Cuboid>,
}

impl CuboidOperation {
    fn new() -> Self {
        CuboidOperation {
            cuboids_list: Vec::new(),
        }
    }
    fn len(&self) -> usize {
        self.cuboids_list.len()
    }
    fn negate(&mut self) {
        for elem in self.cuboids_list.iter_mut() {
            elem.negate();
        }
    }
    fn intersection(&self, other: &Cuboid) -> Self {
        let mut cuboids_list = Vec::new();
        for i in 0..self.len() {
            let c1 = &self.cuboids_list[i];
            cuboids_list.push(c1.intersection(other));
        }
        CuboidOperation {
            cuboids_list,
        }
    }
    fn operate(&self) -> i64 {
        let mut result = 0;
        for elem in self.cuboids_list.iter() {
            result += (elem.len() as i64) * elem.sign as i64;
        }
        result
    }
    fn add_on(&mut self, other: Cuboid) {
        let mut inter = self.intersection(&other);
        inter.negate();
        self.cuboids_list.push(other);
        self.cuboids_list.append(&mut inter.cuboids_list)
    }
    fn add_off(&mut self, other: Cuboid) {
        let mut inter = self.intersection(&other);
        inter.negate();
        self.cuboids_list.append(&mut inter.cuboids_list)
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
            sign: 1
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

fn get_ind_set(commands: &Vec<Command>, index: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    if index  == 0 {
        if commands[0].value {
            result.push(vec![0]);
        }
        return result;
    }
    let prev = get_ind_set(commands, index - 1);
    println!("{}", index);
    result.append(&mut prev.clone());
    if commands[index as usize].value {
        result.push(vec![index as i32]);
    }
    let mut new_clone = prev;
    for elem in new_clone.iter_mut() {
        elem.push(-1);
        elem.push(index as i32);
    }
    result.append(&mut new_clone);
    result
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

fn get_all_intersections_len(commands: &Vec<Command>) -> HashMap<Vec<u32>, i64> {
    let mut result = HashMap::new();
    for combination in make_all_combination(commands.len() as u32) {
        let mut cuboid = Cuboid::new();
        let mut not_init = true;
        for i in &combination {
            if not_init {
                cuboid = commands[*i as usize].cuboid.clone();
                not_init = false;
            } else {
                cuboid = cuboid.intersection(&commands[*i as usize].cuboid);
            }
        }
        result.insert(combination, cuboid.len() as i64);
    }
    result
}

fn part2(commands: &Vec<Command>) -> i64 {
    println!("Creating all operations");
    let list_operations = get_ind_set(commands, commands.len() - 1);
    println!("Creating the maps");
    let dict = get_all_intersections_len(commands);
    println!("Adding the len");
    let mut result = 0;
    for operation in list_operations {
        let mut new_op = Vec::new();
        let mut sign = 1 as i64;
        for ind in operation {
            if ind == -1 {
                sign *= -1;
            } else {
                new_op.push(ind as u32);
            }
        }
        new_op.sort();
        result += sign * dict.get(&new_op).unwrap();
    }
    result
}


fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    let commands = parse_file(&filename);
    println!("{}", commands.len());
    println!("Part 1: {}", part1(&commands));
    
    println!("Part 2: {}", part2(&commands));
    println!("Exp. 2: 2758514936282235");
}
