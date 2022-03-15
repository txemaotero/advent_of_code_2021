use ndarray::{Array2, Axis};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn parse_file(filename: &str) -> Array2<u8> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut vec = Vec::new();
    let mut nlines = 0;
    for line in reader.lines() {
        nlines += 1;
        let line = line.unwrap();
        for c in line.chars() {
            vec.push({
                if c == 'v' {
                    2
                } else if c == '>' {
                    1
                } else {
                    0
                }
            });
        }
    }
    let ncols = vec.len() / nlines;
    Array2::from_shape_vec((nlines, ncols), vec).unwrap()
}

fn move_east(configuration: &mut Array2<u8>) -> u32 {
    let ncols = configuration.ncols();
    let mut moves = 0;
    for mut row in configuration.rows_mut() {
        let mut current_col = 0;
        let first_element = row[0];
        while current_col < ncols {
            let element = row[current_col];
            if element == 0 || element == 2 {
                current_col += 1;
                continue
            }
            let destiny = if current_col == ncols - 1 {
                first_element
            } else {
                row[current_col + 1]
            };
            if destiny == 0 {
                row[(current_col + 1) % ncols] = element;
                row[current_col] = 0;
                current_col += 1;
                moves += 1;
            }
            current_col += 1;
        }
    }
    moves
}

fn move_south(configuration: &mut Array2<u8>) -> u32 {
    let nrows = configuration.nrows();
    let mut moves = 0;
    for mut col in configuration.axis_iter_mut(Axis(1)) {
        let mut current_row = 0;
        let first_element = col[0];
        while current_row < nrows {
            let element = col[current_row];
            if element == 0 || element == 1 {
                current_row += 1;
                continue
            }
            let destiny = if current_row == nrows - 1 {
                first_element
            } else {
                col[current_row + 1]
            };
            if destiny == 0 {
                col[(current_row + 1) % nrows] = element;
                col[current_row] = 0;
                current_row += 1;
                moves += 1;
            }
            current_row += 1;
        }
    }
    moves
}

fn print_configuration(configuration: &Array2<u8>) {
    for row in configuration.rows() {
        for element in row.iter() {
            if *element == 0 {
                print!(".");
            } else if *element == 1 {
                print!(">");
            } else {
                print!("v");
            }
        }
        println!("");
    }
}

fn part1(conf: &mut Array2<u8>) -> u32 {
    let mut iteration = 0;
    loop {
        iteration += 1;
        let moves_east = move_east(conf);
        let moves_south = move_south(conf);
        if moves_east == 0 && moves_south == 0 {
            return iteration
        }
    }
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    let mut conf = parse_file(&filename);
    println!("Part 1: {}", part1(&mut conf));
}
