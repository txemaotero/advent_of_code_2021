use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn bin_to_int(bin: &str) -> u64 {
    u64::from_str_radix(bin, 2).unwrap()
}

fn parse_file(filename: &str) -> String {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap();
    parse_line(line)
}

fn parse_line(line: String) -> String {
    let mut result = Vec::new();
    for c in line.chars() {
        let bin = to_binary(c);
        for b in bin.chars() {
            result.push(b);
        }
    }
    result.into_iter().collect()
}

enum PacketType {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

enum LengthType {
    Length(u64),
    Number(u64),
}

struct Packet {
    version: u8,
    type_id: u8,
    content: PacketType,
}

impl Packet {
    fn new(sequence: &str) -> (Packet, usize) {
        let version = bin_to_int(&sequence[..3]) as u8;
        let type_id = bin_to_int(&sequence[3..6]) as u8;
        let last_index;
        let content;
        if type_id == 4 {
            let result = LiteralPacket::new(&sequence[6..]);
            let packet = result.0;
            last_index = result.1;
            content = PacketType::Literal(packet);
        } else {
            let result = OperatorPacket::new(&sequence[6..]);
            let packet = result.0;
            last_index = result.1;
            content = PacketType::Operator(packet);
        }
        (Packet {
            version,
            type_id,
            content,
        }, last_index + 6)
    }
    
    fn total_version_numbers(&self) -> u64 {
        return self.version as u64 + match self.content {
            PacketType::Literal(_) => 0,
            PacketType::Operator(ref o) => {
                let mut total = 0;
                for p in o.subpackets.iter() {
                    total += p.total_version_numbers();
                }
                total
            },
        }
    }   
    
    fn operation(&self) -> u64 {
        match self.content {
            PacketType::Literal(ref l) => l.number,
            PacketType::Operator(ref o) => o.do_operation(self.type_id) 
        }
    }
}

struct LiteralPacket {
    number: u64,
}

impl LiteralPacket {
    fn new(sequence: &str) -> (LiteralPacket, usize) {
        let mut index: usize = 0;
        let mut keep_reading = true;
        let mut number_string = String::new();
        while keep_reading {
            if &sequence[index..index+1] == "0" {
                keep_reading = false;
            }
            number_string.push_str(&sequence[index+1..index+5]);
            index += 5;
        }
        let number = bin_to_int(&number_string);
        (LiteralPacket { number }, index)
    }
}

struct OperatorPacket {
    subpackets: Vec<Packet>,
}

impl OperatorPacket {
    fn new(sequence: &str) -> (OperatorPacket, usize) {
        let pack_start_ind;
        let length_type = if &sequence[0..1] == "0" {
            pack_start_ind = 16;
            LengthType::Length(bin_to_int(&sequence[1..pack_start_ind]))
        } else {
            pack_start_ind = 12;
            LengthType::Number(bin_to_int(&sequence[1..pack_start_ind]))
        };
        let mut subpackets = Vec::new();
        let mut start_index = pack_start_ind;
        match length_type {
            LengthType::Length(len) => {
                while start_index < (len as usize + pack_start_ind) {
                    let (packet, index) = Packet::new(&sequence[start_index..]);
                    subpackets.push(packet);
                    start_index += index;
                }
            },
            LengthType::Number(num) => {
                for _ in 0..num {
                    let (packet, index) = Packet::new(&sequence[start_index..]);
                    subpackets.push(packet);
                    start_index += index;
                }
            },
        }
        (OperatorPacket {
            subpackets,
        }, start_index)
    }
    
    fn do_operation(&self, oper_type: u8) -> u64 {
        if oper_type == 0 {
            return self.subpackets.iter().map(|p| p.operation()).sum();
        } else if oper_type == 1 {
            return self.subpackets.iter().map(|p| p.operation()).product();
        } else if oper_type == 2 {
            return self.subpackets.iter().map(|p| p.operation()).min().unwrap();
        } else if oper_type == 3 {
            return self.subpackets.iter().map(|p| p.operation()).max().unwrap();
        } else if oper_type == 5 {
            return (self.subpackets[0].operation() > self.subpackets[1].operation()) as u64;
        } else if oper_type == 6 {
            return (self.subpackets[0].operation() < self.subpackets[1].operation()) as u64;
        } else if oper_type == 7 {
            return (self.subpackets[0].operation() == self.subpackets[1].operation()) as u64;
        } else {
            panic!("Invalid operation type {}", oper_type);
        }
    }
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");

    let sequence = parse_file(&filename);
    let (packet, _) = Packet::new(&sequence);

    println!("Result of part 1: {}", packet.total_version_numbers());
    println!("Result of part 2: {}", packet.operation());

}