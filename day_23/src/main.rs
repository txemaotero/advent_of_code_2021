// Position Numeration
//
// ######################
// #1617.18.19. 20. 2122#
// #####3##7##11##15#####
//     #2##6##10##14#
//     #1##5## 9##13#
//     #0##4## 8##12#
//     ##############

use std::collections::HashMap;

static ORIGIN_SITES: [usize; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

fn stacks() -> HashMap<char, [usize; 4]> {
    HashMap::from([
        ('A', [0, 1, 2, 3]),
        ('B', [4, 5, 6, 7]),
        ('C', [8, 9, 10, 11]),
        ('D', [12, 13, 14, 15]),
    ])
}

fn distances() -> HashMap<usize, HashMap<usize, u32>> {
    let mut res = HashMap::from([
        (0, HashMap::from([(16, 6), (17, 5), (18, 5), (19, 7), (20, 9), (21, 11), (22, 12)])),
        (4, HashMap::from([(16, 8), (17, 7), (18, 5), (19, 5), (20, 7), (21, 9), (22, 10)])),
        (8, HashMap::from([(16, 10), (17, 9), (18, 7), (19, 5), (20, 5), (21, 7), (22, 8)])),
        (12, HashMap::from([(16, 12), (17, 11), (18, 9), (19, 7), (20, 5), (21, 5), (22, 6)])),
    ]);
    for i in 0..4 {
        for j in 1..4 {
            res.insert(i * 4 + j, res[&(i * 4)].iter().map(|(k, v)| (*k, *v - j as u32)).collect());
        }
    }
    res
}

struct Piece {
    leter: char,
    position: usize,
    moved: bool,
    in_its_place: bool,
}

impl Piece {
    fn new(leter: char, position: usize) -> Piece {
        Piece {
            leter: leter,
            position: position,
            moved: false,
            in_its_place: false,
        }
    }
}

fn get_initial_state() -> (HashMap<usize, bool>, Vec<Piece>) {
    let table: HashMap<usize, bool> = (0..23).into_iter().map(|i| (i, ORIGIN_SITES.contains(&i))).collect(); 
    let mut pieces = Vec::new();
    pieces.push(Piece::new('C', 0));
    pieces.push(Piece::new('D', 1));
    pieces.push(Piece::new('D', 2));
    pieces.push(Piece::new('D', 3));

    pieces.push(Piece::new('A', 4));
    pieces.push(Piece::new('B', 5));
    pieces.push(Piece::new('C', 6));
    pieces.push(Piece::new('B', 7));

    pieces.push(Piece::new('D', 8));
    pieces.push(Piece::new('A', 9));
    pieces.push(Piece::new('B', 10));
    pieces.push(Piece::new('A', 11));

    pieces.push(Piece::new('B', 12));
    pieces.push(Piece::new('C', 13));
    pieces.push(Piece::new('A', 14));
    pieces.push(Piece::new('C', 15));
    (table, pieces)
}

fn part2() -> u32 {
    let (table, pieces) = get_initial_state();
    let mut result = 10000000;
    let mut partial_result = 0;
    play_game(table, pieces, &mut result, &mut partial_result);
    result
}

fn play_game(table: HashMap<usize, bool>, pieces: Vec<Piece>, result: &mut u32, partial_result: &mut u32) {
}

fn main() {
    dbg!(distances());
}
