use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum PacketType {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

enum LengthType {
    Length(u32),
    Number(u32),
}

struct Packet {
    version: u32,
    type_id: u32,
    content: PacketType,
}

fn bin_to_int(bin: &str) -> u32 {
    u32::from_str_radix(bin, 2).unwrap()
}

impl Packet {
    fn new(sequence: &str) -> Packet {
        let version = bin_to_int(&sequence[..3]);
        let type_id = bin_to_int(&sequence[3..6]);
        if type_id == 4 {
            Packet {
                version,
                type_id,
                content: PacketType::Literal(LiteralPacket::new(&sequence[6..])),
            }
        } else {
            Packet {
                version,
                type_id,
                content: PacketType::Operator(OperatorPacket::new(sequence[6..])),
            }
        }
    }
}

struct LiteralPacket {
    numbers: Vec<u32>,
}

impl LiteralPacket {
    fn new(sequence: &str) -> LiteralPacket {
        let mut numbers = Vec::new();
        let mut number = String::new();
        let index: usize = 0;
        let keep_reading = true;
        while keep_reading {
            if &sequence[index..index+1] == "0" {
                keep_reading = false;
            }
            numbers.push(bin_to_int(&sequence[index+1..index+4]));
            index += 4;
        }
        LiteralPacket { numbers }
    }
}

struct OperatorPacket {
    length_type: LengthType,
    subpackets: Vec<Packet>,
}

impl OperatorPacket {
    fn new(sequence: &str) -> OperatorPacket {
        let pack_start_ind;
        let length_type = if &sequence[0..1] == "0" {
            pack_start_ind = 16;
            LengthType::Length(bin_to_int(&sequence[1..pack_start_ind]))
        } else {
            pack_start_ind = 12;
            LengthType::Number(bin_to_int(&sequence[1..pack_start_ind]))
        };
        let mut subpackets = Vec::new();
        OperatorPacket {
            length_type,
            subpackets,
        }
    }
}

fn parse_file(filename: &str) -> Vec<char> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap();
    let mut result = Vec::new();
    for c in line.chars() {
        let bin = to_binary(c);
        for b in bin.chars() {
            result.push(b);
        }
    }
    dbg!(&result);
    result
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn part1(filename: &str, is_part1: bool) -> i32 {
    0
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    parse_file(&filename);
    // Open the file in read-only mode (ignoring errors).
    // let part1_result = part1(&filename, true);
    // println!("Result of part 1: {}", part1_result);

}