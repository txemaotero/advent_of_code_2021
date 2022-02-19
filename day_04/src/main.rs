use ndarray::prelude::*;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;


fn parse_file(filename: &str) -> (Vec<i32>, Vec<BingoCard>) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut aux_array = Array::zeros((5, 5));
    let mut aux_index: i32 = 0;
    let mut bingo_cards: Vec<BingoCard> = Vec::new();
    let mut number_seq: Vec<i32> = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        if index == 0 {
            number_seq = line
                .unwrap()
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            continue;
        } else if index == 1 {
            continue;
        }
        let line = line.unwrap();
        if line.trim().is_empty() {
            bingo_cards.push(BingoCard::new(aux_array.clone()));
            aux_array.fill(0);
            aux_index = 0;
            continue;
        } else {
            // fill the row aux_index of aux_array with the content of line
            aux_array.slice_mut(s![aux_index, ..]).assign(
                &line
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Array1<i32>>(),
            );
            aux_index += 1;
        }
    }
    (number_seq, bingo_cards)
}


fn part1(filename: &str) -> i32 {
    let (number_seq, mut bingo_cards) = parse_file(filename);
    for number in number_seq {
        for card in bingo_cards.iter_mut() {
            card.mark_number(number);
            if card.check_bingo() {
                return number * card.get_sum_unmarked();
            }
        }
    }
    0
}


fn part2(filename: &str) -> i32 {
    let (number_seq, mut bingo_cards) = parse_file(filename);
    let total = &bingo_cards.len();
    let mut solved = 0;
    for number in number_seq {
        for card in bingo_cards.iter_mut() {
            if card.was_solved {
                continue;
            }
            card.mark_number(number);
            if card.check_bingo() {
                solved += 1;
            }
            if solved == *total {
                return number * card.get_sum_unmarked();
            }
        }
    }
    0
}

struct BingoCard {
    card: Array2<i32>,
    was_solved: bool,
}

impl BingoCard {
    fn new(card: Array2<i32>) -> BingoCard {
        BingoCard { card, was_solved: false }
    }
    
    fn mark_number(&mut self, number: i32) {
        self.card = self.card.map(|x| if *x == number { -1 } else { x.clone() });
    }
    
    fn check_bingo(&mut self) -> bool {
        for row in self.card.axis_iter(Axis(0)) {
            if row.iter().all(|x| *x == -1) {
                self.was_solved = true;
                return true;
            }
        }
        for column in self.card.axis_iter(Axis(1)) {
            if column.iter().all(|x| *x == -1) {
                self.was_solved = true;
                return true;
            }
        }
        return false;
    }
    
    fn get_sum_unmarked(&self) -> i32 {
        self.card.iter().fold(0, |acc, x| acc + if *x == -1 { 0 } else { *x })
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
