use std::cmp::max;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

#[derive(Debug, Clone)]
enum Element {
    Number(i32),
    Pair(PairNumber),
}

#[derive(Debug, Clone)]
struct PairNumber {
    left: Box<Element>,
    right: Box<Element>,
    nest_level: u32,
}

impl PairNumber {
    fn increment_nest_level(&mut self) {
        self.nest_level += 1;
        match *self.left {
            Element::Pair(ref mut pair) => pair.increment_nest_level(),
            _ => {}
        }
        match *self.right {
            Element::Pair(ref mut pair) => pair.increment_nest_level(),
            _ => {}
        }
    }
    fn len(&self) -> u32 {
        let mut len = 0;
        match *self.left {
            Element::Pair(ref pair) => len += pair.len(),
            Element::Number(_) => len += 1,
        }
        match *self.right {
            Element::Pair(ref pair) => len += pair.len(),
            Element::Number(_) => len += 1,
        }
        len
    }

    fn reduce(&mut self) {
        let mut keep_reducing = true;
        while keep_reducing {
            let test = self.explode(0);
            match test {
                Some((index, (left_num, right_num))) => {
                    if index > 0 {
                        self.add_at(index - 1, left_num)
                    }
                    if index < self.len() - 1 {
                        self.add_at(index + 1, right_num)
                    }
                    continue;
                }
                None => {}
            }
            if self.split() {
                continue;
            }
            keep_reducing = false;
        }
    }
    fn add_at(&mut self, index: u32, number: i32) {
        let mut current_index = 0;
        match *self.left {
            Element::Pair(ref mut pair) => {
                if index < pair.len() {
                    pair.add_at(index, number);
                    return;
                }
                current_index += pair.len();
            }
            Element::Number(num) => {
                if current_index == index {
                    self.left = Box::new(Element::Number(num + number));
                    return;
                }
                current_index += 1;
            }
        }
        match *self.right {
            Element::Pair(ref mut pair) => {
                if index < (pair.len() + current_index) {
                    pair.add_at(index - current_index, number);
                    return;
                }
            }
            Element::Number(num) => {
                if current_index == index {
                    self.right = Box::new(Element::Number(num + number));
                    return;
                }
            }
        }
        panic!("add_at: index out of bounds");
    }
    fn split(&mut self) -> bool {
        let mut result = split_element(&mut self.left, self.nest_level);
        if result {
            return result;
        }
        result = split_element(&mut self.right, self.nest_level);
        return result;
    }
    fn explode(&mut self, mut index: u32) -> Option<(u32, (i32, i32))> {
        let mut aux_len = 0;
        let change = match *self.left {
            Element::Pair(ref mut pair) => {
                if pair.nest_level == 4 {
                    let (num_left, num_right) = get_pair_numbers(&self.left);
                    self.left = Box::new(Element::Number(0));
                    return Some((index, (num_left, num_right)));
                } else {
                    aux_len = pair.len();
                    pair.explode(index)
                }
            }
            Element::Number(_) => {
                index += 1;
                None
            }
        };
        match change {
            Some(res) => {
                return Some(res);
            }
            None => {}
        }
        let change = match *self.right {
            Element::Pair(ref mut pair) => {
                if pair.nest_level == 4 {
                    let (num_left, num_right) = get_pair_numbers(&self.right);
                    self.right = Box::new(Element::Number(0));
                    return Some((index + aux_len, (num_left, num_right)));
                } else {
                    pair.explode(index + aux_len)
                }
            }
            Element::Number(_) => None,
        };
        change
    }
    fn magnitude(&self) -> i32 {
        let mut magnitude = 3 * match *self.left {
            Element::Pair(ref pair) => pair.magnitude(),
            Element::Number(num) => num,
        };
        magnitude += 2 * match *self.right {
            Element::Pair(ref pair) => pair.magnitude(),
            Element::Number(num) => num,
        };
        magnitude
    }
}

impl Add for PairNumber {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut left = self.clone();
        let mut right = other.clone();
        left.increment_nest_level();
        right.increment_nest_level();
        let mut result = PairNumber {
            left: Box::new(Element::Pair(left)),
            right: Box::new(Element::Pair(right)),
            nest_level: 0,
        };
        result.reduce();
        result
    }
}

impl fmt::Display for PairNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")
            .and_then(|_| match *self.left {
                Element::Pair(ref pair) => {
                    write!(f, "{}", pair)
                }
                Element::Number(num) => {
                    write!(f, "{}", num)
                }
            })
            .and_then(|_| write!(f, ","))
            .and_then(|_| match *self.right {
                Element::Pair(ref pair) => {
                    write!(f, "{}", pair)
                }
                Element::Number(num) => {
                    write!(f, "{}", num)
                }
            })
            .and_then(|_| write!(f, "]"))
    }
}

fn get_pair_numbers(element: &Element) -> (i32, i32) {
    let left;
    let right;
    match element {
        Element::Pair(pair) => {
            match *pair.left {
                Element::Number(num) => left = num,
                _ => {
                    panic!("Left element is not a number");
                }
            };
            match *pair.right {
                Element::Number(num) => right = num,
                _ => {
                    panic!("Right element is not a number");
                }
            };
        }
        Element::Number(_) => panic!("Element is not a pair"),
    };
    (left, right)
}

fn split_element(element: &mut Element, parent_nest_level: u32) -> bool {
    match *element {
        Element::Number(num) => {
            if num >= 10 {
                *element = Element::Pair(PairNumber {
                    left: Box::new(Element::Number(num / 2)),
                    right: Box::new(Element::Number(num / 2 + num % 2)),
                    nest_level: parent_nest_level + 1,
                });
                true
            } else {
                false
            }
        }
        Element::Pair(ref mut pair) => pair.split(),
    }
}

fn parse_line(line: &str, nest_level: u32) -> PairNumber {
    // Remove first and last []
    let line = line[1..line.len() - 1].to_string();
    let mut left_right = [String::new(), String::new()];
    let mut brackets = 0;
    let mut iter = line.chars();
    for element in left_right.iter_mut() {
        loop {
            let c = iter.next().unwrap();
            if c == ',' {
                continue;
            } else if c == '[' {
                brackets += 1;
            } else if c == ']' {
                brackets -= 1;
            }
            element.push(c);
            if brackets == 0 {
                break;
            }
        }
    }
    PairNumber {
        left: Box::new(parse_element(&left_right[0], nest_level)),
        right: Box::new(parse_element(&left_right[1], nest_level)),
        nest_level: nest_level,
    }
}

fn parse_element(line: &str, nest_level: u32) -> Element {
    if line.len() == 1 {
        return Element::Number(line.chars().next().unwrap().to_digit(10).unwrap() as i32);
    } else {
        return Element::Pair(parse_line(line, nest_level + 1));
    }
}

fn sum_numbers_in_file(filename: &str) -> PairNumber {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut result = None;
    for line in reader.lines() {
        let number = parse_line(&line.unwrap(), 0);
        result = match result {
            None => Some(number),
            Some(res) => Some(res + number),
        };
    }
    result.unwrap()
}

fn part2(filename: &str) -> i32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut numbers = Vec::new();
    for line in reader.lines() {
        numbers.push(parse_line(&line.unwrap(), 0));
    }
    let n_numb = numbers.len();
    let mut final_res = 0;
    for i in 0..n_numb {
        for j in i + 1..n_numb {
            let number = numbers[i].clone();
            let other = numbers[j].clone();
            let result = number + other;
            final_res = max(final_res, result.magnitude());
            let number = numbers[j].clone();
            let other = numbers[i].clone();
            let result = number + other;
            final_res = max(final_res, result.magnitude());
        }
    }
    final_res
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");

    let number = sum_numbers_in_file(&filename);
    println!("{}", number);
    println!("Part1: {}", number.magnitude());

    println!("Part2: {}", part2(&filename));
}
