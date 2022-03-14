// Position Numeration
//
// ######################
// #1617.18.19. 20. 2122#
// #####3##7##11##15#####
//     #2##6##10##14#
//     #1##5## 9##13#
//     #0##4## 8##12#
//     ##############
//     
// This script only solve the part 2. Part 1 was solved by hand (result 15538).

use std::cmp::min;
use std::collections::{HashMap, VecDeque};

static HALL_SITES: [usize; 7] = [16, 17, 18, 19, 20, 21, 22];

fn stacks() -> HashMap<char, [usize; 4]> {
    HashMap::from([
        ('A', [0, 1, 2, 3]),
        ('B', [4, 5, 6, 7]),
        ('C', [8, 9, 10, 11]),
        ('D', [12, 13, 14, 15]),
    ])
}

fn costs() -> HashMap<char, u32> {
    HashMap::from([('A', 1), ('B', 10), ('C', 100), ('D', 1000)])
}

fn distances() -> HashMap<usize, HashMap<usize, u32>> {
    let mut res = HashMap::from([
        (
            0,
            HashMap::from([
                (16, 6),
                (17, 5),
                (18, 5),
                (19, 7),
                (20, 9),
                (21, 11),
                (22, 12),
            ]),
        ),
        (
            4,
            HashMap::from([
                (16, 8),
                (17, 7),
                (18, 5),
                (19, 5),
                (20, 7),
                (21, 9),
                (22, 10),
            ]),
        ),
        (
            8,
            HashMap::from([
                (16, 10),
                (17, 9),
                (18, 7),
                (19, 5),
                (20, 5),
                (21, 7),
                (22, 8),
            ]),
        ),
        (
            12,
            HashMap::from([
                (16, 12),
                (17, 11),
                (18, 9),
                (19, 7),
                (20, 5),
                (21, 5),
                (22, 6),
            ]),
        ),
    ]);
    for i in 0..4 {
        for j in 1..4 {
            res.insert(
                i * 4 + j,
                res[&(i * 4)]
                    .iter()
                    .map(|(k, v)| (*k, *v - j as u32))
                    .collect(),
            );
        }
    }
    res
}

fn paths() -> HashMap<(usize, usize), Vec<usize>> {
    // from doors to all hall
    let mut res = HashMap::from([
        ((3, 16), vec![17]),
        ((3, 17), vec![]),
        ((3, 18), vec![]),
        ((3, 19), vec![18]),
        ((3, 20), vec![18, 19]),
        ((3, 21), vec![18, 19, 20]),
        ((3, 22), vec![18, 19, 20, 21]),
        ((7, 16), vec![18, 17]),
        ((7, 17), vec![18]),
        ((7, 18), vec![]),
        ((7, 19), vec![]),
        ((7, 20), vec![19]),
        ((7, 21), vec![19, 20]),
        ((7, 22), vec![19, 20, 21]),
        ((11, 16), vec![19, 18, 17]),
        ((11, 17), vec![19, 18]),
        ((11, 18), vec![19]),
        ((11, 19), vec![]),
        ((11, 20), vec![]),
        ((11, 21), vec![20]),
        ((11, 22), vec![20, 21]),
        ((15, 16), vec![20, 19, 18, 17]),
        ((15, 17), vec![20, 19, 18]),
        ((15, 18), vec![20, 19]),
        ((15, 19), vec![20]),
        ((15, 20), vec![]),
        ((15, 21), vec![]),
        ((15, 22), vec![21]),
    ]);
    // from deeper in the rooms
    for room in 0..4 {
        for deep in 1..4 {
            for hall in HALL_SITES {
                let mut element = res.get(&(room * 4 + 3, hall)).unwrap().clone();
                let new_key = (room * 4 + 3 - deep, hall);
                element.push(room * 4 + 3 - deep + 1);
                res.insert(new_key, element);
            }
        }
    }
    // inverse
    let mut res_tot = HashMap::new();
    for (k, v) in res.iter() {
        res_tot.insert((k.0, k.1), v.clone());
        res_tot.insert((k.1, k.0), v.clone());
    }
    res_tot
}

struct Data {
    stacks: HashMap<char, [usize; 4]>,
    costs: HashMap<char, u32>,
    distances: HashMap<usize, HashMap<usize, u32>>,
    paths: HashMap<(usize, usize), Vec<usize>>,
}

impl Data {
    fn new() -> Data {
        Data {
            stacks: stacks(),
            costs: costs(),
            distances: distances(),
            paths: paths(),
        }
    }
}

fn get_initial_state() -> HashMap<usize, char> {
    HashMap::from([
        (0, 'C'),
        (1, 'D'),
        (2, 'D'),
        (3, 'D'),
        (4, 'A'),
        (5, 'B'),
        (6, 'C'),
        (7, 'B'),
        (8, 'D'),
        (9, 'A'),
        (10, 'B'),
        (11, 'A'),
        (12, 'B'),
        (13, 'C'),
        (14, 'A'),
        // (15, 'C'),
        (22, 'C'), // This initial guess results in the solution (adding 900)
    ])
}

fn part2() -> u32 {
    let table = get_initial_state();
    let mut result = 10000000;
    let partial_result = 0;
    let data = Data::new();
    play_game(table, &mut result, partial_result, 0, &data);
    result
}

fn play_game(
    table: HashMap<usize, char>,
    result: &mut u32,
    partial_result: u32,
    depth: u32,
    data: &Data,
) {
    if all_in_place(&table, data) {
        if partial_result < *result {
            println!("{}", partial_result);
            *result = partial_result;
        }
        return;
    }
    let possible_moves = get_possibilities(&table, data);
    let len = possible_moves.len();
    if len == 0 {
        return;
    }
    if partial_result > min(*result, 49632) {
        return;
    }
    for (index, (orig, dest)) in possible_moves.iter().enumerate() {
        if depth <= 0 {
            println!("Depth {}: {}/{}", depth, index, len);
        }
        let mut new_table = table.clone();
        let letter = table[&orig];
        new_table.insert(*dest, letter);
        new_table.remove(&orig);
        let dist;
        if orig < dest {
            dist = data.distances[&orig][&dest];
        } else {
            dist = data.distances[&dest][&orig];
        }
        let new_partial_result = partial_result + dist * data.costs[&letter];
        play_game(new_table, result, new_partial_result, depth + 1, &data);
    }
}

fn get_possibilities(table: &HashMap<usize, char>, data: &Data) -> VecDeque<(usize, usize)> {
    let mut res = VecDeque::new();
    for ((orig, dest), obstacles) in data.paths.iter() {
        if table.contains_key(&dest) || obstacles.iter().any(|&x| table.contains_key(&x)) {
            continue;
        }
        if table.contains_key(&orig) {
            let letter = table[&orig];
            if is_on_site(orig, &letter, table, data) {
                continue;
            }
            // if origin is on hall we can only move to the good site
            if HALL_SITES.contains(orig) {
                if is_on_site(dest, &letter, table, data) {
                    res.push_front((*orig, *dest));
                }
            } else {
                res.push_back((*orig, *dest));
            }
        }
    }
    res
}

fn is_on_site(origin: &usize, letter: &char, table: &HashMap<usize, char>, data: &Data) -> bool {
    let numbers = data.stacks[letter];
    if !numbers.contains(origin) {
        return false;
    }
    let dist_to_origin = origin - origin / 4 * 4;
    for diff in 1..dist_to_origin + 1 {
        let index = origin - diff;
        if !table.contains_key(&index) {
            return false;
        }
        if table.get(&index).unwrap() != letter {
            return false;
        }
    }
    true
}

fn all_in_place(table: &HashMap<usize, char>, data: &Data) -> bool {
    let mut letters_site = HashMap::new();
    for (site, letter) in table.iter() {
        if HALL_SITES.contains(site) {
            return false;
        }
        let nums = letters_site.entry(*letter).or_insert(Vec::new());
        nums.push(*site);
    }
    for (letter, numbers) in letters_site.iter_mut() {
        numbers.sort();
        if numbers != data.stacks.get(letter).unwrap() {
            return false;
        }
    }
    true
}

fn main() {
    println!("Part 2 result: {}", part2());
    // Solution 47258
}
