use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};


fn parse_file(filename: &str) -> Vec<_> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut commands = Vec::new();
    for line in reader.lines() {
        continue
    }
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    println!("Hello, world!");
}
