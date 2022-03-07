use ndarray::{array, Array1, Array2, Axis};
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Radar {
    beacons: Array2<i32>,
}

impl Radar {
    fn overlaps(&self, other: &Radar) -> Option<(Array1<i32>, Array2<i32>)> {
        for transformation in transformations() {
            let transformed_beacons = other.beacons.dot(&transformation);
            let vec = calc_vectors_count(&transformed_beacons, &self.beacons);
            match vec {
                Some(v) => {
                    return Some((v, transformation));
                }
                None => {}
            }
        }
        None
    }
    fn transform_beacons(&self, translation: &Array1<i32>, transformation: &Array2<i32>) -> Radar {
        Radar {
            beacons: self.beacons.dot(transformation) + translation,
        }
    }
}

fn calc_vectors_count(origin: &Array2<i32>, target: &Array2<i32>) -> Option<Array1<i32>> {
    let mut vectors = HashMap::<Array1<i32>, u32>::new();
    for orig_vec in origin.axis_iter(Axis(0)) {
        for tar_vec in target.axis_iter(Axis(0)) {
            let diff = &tar_vec - &orig_vec;
            let counter = vectors.entry(diff.clone()).or_insert(0);
            *counter += 1;
            if *counter >= 12 {
                return Some(diff);
            }
        }
    }
    None
}

fn transformations() -> HashSet<Array2<i32>> {
    let mut result = HashSet::new();
    // sin and cos for theta = 0, 90, 180, 270
    let sin = array![0, 1, 0, -1];
    let cos = array![1, 0, -1, 0];
    for (sin_x, cos_x) in sin.iter().zip(cos.iter()) {
        let rot_x: Array2<i32> = array![[1, 0, 0], [0, *cos_x, -*sin_x], [0, *sin_x, *cos_x],];
        for (sin_y, cos_y) in sin.iter().zip(cos.iter()) {
            let rot_y = array![[*cos_y, 0, *sin_y], [0, 1, 0], [-*sin_y, 0, *cos_y],];
            let mut rot = rot_x.dot(&rot_y);
            for (sin_z, cos_z) in sin.iter().zip(cos.iter()) {
                let rot_z = array![[*cos_z, -*sin_z, 0], [*sin_z, *cos_z, 0], [0, 0, 1],];
                rot = rot.dot(&rot_z);
                result.insert(rot.clone());
            }
        }
    }
    result
}

fn parse_file(filename: &str) -> Vec<Radar> {
    let mut result = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut aux_radar = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        if line.starts_with("---") {
            if aux_radar.len() != 0 {
                let beacons = Array2::from_shape_vec((aux_radar.len() / 3, 3), aux_radar).unwrap();
                result.push(Radar { beacons });
                aux_radar = Vec::new();
            }
            continue;
        }
        for coord in line.split(',') {
            aux_radar.push(coord.parse::<i32>().unwrap());
        }
    }
    let beacons = Array2::from_shape_vec((aux_radar.len() / 3, 3), aux_radar).unwrap();
    result.push(Radar { beacons });
    result
}

fn get_radars_relation(
    radars: &Vec<Radar>,
) -> HashMap<usize, HashMap<usize, (Array1<i32>, Array2<i32>)>> {
    let mut result = HashMap::new();
    for i_orig in 0..radars.len() {
        for i_tar in 0..radars.len() {
            if i_orig == i_tar {
                continue;
            }
            let overlap = radars[i_orig].overlaps(&radars[i_tar]);
            match overlap {
                Some((vec, transformation)) => {
                    let map = result.entry(i_orig).or_insert(HashMap::new());
                    map.insert(i_tar, (vec.clone(), transformation.clone()));
                }
                None => {}
            }
        }
    }
    result
}
fn get_connections(
    relation: &HashMap<usize, HashMap<usize, (Array1<i32>, Array2<i32>)>>,
) -> HashMap<usize, HashSet<usize>> {
    let mut result = HashMap::new();
    for (i_orig, map) in relation.iter() {
        let aux: HashSet<usize> = map.keys().cloned().collect();
        for i_tar in &aux {
            let elem = result.entry(*i_tar).or_insert(HashSet::new());
            elem.insert(*i_orig);
        }
        let elem = result.entry(*i_orig).or_insert(HashSet::new());
        elem.extend(&aux);
    }
    result
}
fn get_path_to_zero(connections: &HashMap<usize, HashSet<usize>>, i_orig: usize) -> Vec<usize> {
    let result = Vec::new();
    let visited = HashSet::new();
    let path = find_path(&connections, i_orig, visited, result);
    match path {
        Some(p) => p,
        None => panic!("No path to zero"),
    }
}
fn find_path(
    connections: &HashMap<usize, HashSet<usize>>,
    i_orig: usize,
    mut visited: HashSet<usize>,
    mut result: Vec<usize>,
) -> Option<Vec<usize>> {
    result.push(i_orig);
    if i_orig == 0 {
        return Some(result);
    }
    if visited.contains(&i_orig) {
        return None;
    }
    visited.insert(i_orig);
    let aux = connections.get(&i_orig).unwrap().clone();
    for i_tar in aux {
        if visited.contains(&i_tar) {
            continue;
        }
        let path = find_path(connections, i_tar, visited.clone(), result.clone());
        match path {
            Some(path) => return Some(path),
            None => {}
        }
    }
    None
}

fn part1(radars: &Vec<Radar>) -> usize {
    let radars_relation = get_radars_relation(&radars);
    let connections = get_connections(&radars_relation);

    let mut beacons: HashSet<_> = HashSet::new();
    for row in radars[0].beacons.rows() {
        beacons.insert(row.clone());
    }
    for index in 1..radars.len() {
        let path = get_path_to_zero(&connections, index);
        let mut aux_beacons = radars[index].beacons.clone();
        for i in 0..path.len() - 1 {
            let (vec, transf) = &radars_relation[&path[i]][&path[i + 1]];
            aux_beacons = aux_beacons.dot(transf) + vec;
        }
        let rows = aux_beacons.rows();
        for row in rows {
            beacons.insert(row.clone());
        }
    }
    beacons.len()
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    let radars = parse_file(&filename);

    // let result = get_radars_relation(&radars);
    // let con = get_connections(&result);
    // dbg!(&con);
    // let path = get_path_to_zero(&con, 3);
    // dbg!(&path);
    let result = part1(&radars);
    println!("Part 1: {}", result);
}
