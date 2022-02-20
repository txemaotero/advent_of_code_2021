use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn part1(filename: &str, diagonal: bool) -> u32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut frequency = HashMap::new();

    for line in reader.lines() {
        let (start, end) = parse_input_line(&line.unwrap());
        let line = Line::new(start, end);
        for point in line.points(diagonal) {
            let count = frequency.entry(point).or_insert(0);
            *count += 1;
        }
    }
    frequency.values().map(|&x| (x >= 2) as u32).sum::<u32>() 
}

fn parse_input_line(line: &str) -> ((i32, i32), (i32, i32)) {
    let iter = line.split(" -> ");
    let mut points = iter.map(|point| {
        let mut coords = point.split(",");
        let x = coords.next().unwrap().parse().unwrap();
        let y = coords.next().unwrap().parse().unwrap();
        (x, y)
    });
    (points.next().unwrap(), points.next().unwrap())
}

struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

impl Line {
    fn new(start: (i32, i32), end: (i32, i32)) -> Line {
        Line {
            start: start,
            end: end,
        }
    }

    fn points(&self, diagonal: bool) -> Vec<(i32, i32)> {
        let (x0, y0) = self.start;
        let (x1, y1) = self.end;
        let mut points: Vec<(i32, i32)> = Vec::new();
        if x0 == x1 {
            let vals = if y0 < y1 {
                y0..y1 + 1
            } else {
                y1..y0 + 1
            };
            for yaux in vals {
                points.push((x0, yaux));
            }
        } else if y0 == y1 {
            let vals = if x0 < x1 {
                x0..x1 + 1
            } else {
                x1..x0 + 1
            };
            for xaux in vals {
                points.push((xaux, y0));
            }
        } else if diagonal {
            if self.is_diagonal() {
                if x0 < x1 {
                    if y0 < y1 {
                        for (xaux, yaux) in (x0..x1+1).zip(y0..y1+1) {
                            points.push((xaux, yaux));
                        }
                    } else {
                        for (xaux, yaux) in (x0..x1+1).zip((y1..y0+1).rev()) {
                            points.push((xaux, yaux));
                        }
                    }
                } else {
                    if y0 < y1 {
                        for (xaux, yaux) in ((x1..x0+1).rev()).zip(y0..y1+1) {
                            points.push((xaux, yaux));
                        }
                    } else {
                        for (xaux, yaux) in ((x1..x0+1).rev()).zip((y1..y0+1).rev()) {
                            points.push((xaux, yaux));
                        }
                    }
                }
            }
        }
        points
    }
    
    fn is_diagonal(&self) -> bool {
        let (diffx, diffy) = ((self.start.0 - self.end.0).abs(), (self.start.1 - self.end.1).abs()); 
        return diffx == diffy;
    }
    
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    // Open the file in read-only mode (ignoring errors).
    let part1_result = part1(&filename, false);
    println!("Result of part 1: {}", part1_result);

    let part2_result = part1(&filename, true);
    println!("Result of part 2: {}", part2_result);
}
