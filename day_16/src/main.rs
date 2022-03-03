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

struct LiteralPacket {
    number: u64,
}

impl LiteralPacket {
    fn new(sequence: &str) -> (LiteralPacket, usize) {
        let mut index: usize = 0;
        let mut keep_reading = true;
        // println!("Literal Packet");
        // println!("{}", sequence);
        let mut number_string = String::new();
        while keep_reading {
            if &sequence[index..index+1] == "0" {
                keep_reading = false;
            }
            number_string.push_str(&sequence[index+1..index+5]);
            index += 5;
        }
        // index += 4 - (index % 4);

        let number = bin_to_int(&number_string);
        // println!("Number: {}", number);
        (LiteralPacket { number }, index)
    }
}

struct OperatorPacket {
    length_type: LengthType,
    subpackets: Vec<Packet>,
}

impl OperatorPacket {
    fn new(sequence: &str) -> (OperatorPacket, usize) {
        let pack_start_ind;
        // println!("Operator Packet");
        let length_type = if &sequence[0..1] == "0" {
            pack_start_ind = 16;
            // println!("Length: {} -> {}", bin_to_int(&sequence[1..pack_start_ind]), &sequence[1..pack_start_ind]);
            LengthType::Length(bin_to_int(&sequence[1..pack_start_ind]))
        } else {
            pack_start_ind = 12;
            // println!("Number: {} -> {}", bin_to_int(&sequence[1..pack_start_ind]), &sequence[1..pack_start_ind]);
            LengthType::Number(bin_to_int(&sequence[1..pack_start_ind]))
        };
        let mut subpackets = Vec::new();
        let mut start_index = pack_start_ind;
        match length_type {
            LengthType::Length(len) => {
                // println!("Len {}: pack ind {}", len, pack_start_ind);
                while start_index < (len as usize + pack_start_ind) {
                    let (packet, index) = Packet::new(&sequence[start_index..]);
                    subpackets.push(packet);
                    start_index += index;
                    // println!("Len {}: pack ind {}", len, pack_start_ind);
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
            length_type,
            subpackets,
        }, pack_start_ind)
    }
}

enum PacketType {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

enum LengthType {
    Length(u64),
    Number(u64),
}

fn bin_to_int(bin: &str) -> u64 {
    u64::from_str_radix(bin, 2).unwrap()
}

struct Packet {
    version: u64,
    type_id: u64,
    content: PacketType,
    sequence: String,
}

impl Packet {
    fn new(sequence: &str) -> (Packet, usize) {
        let version = bin_to_int(&sequence[..3]);
        let type_id = bin_to_int(&sequence[3..6]);
        println!("sequence: {}", sequence);
        println!("version: {} -> {}, type_id: {} -> {}", version, &sequence[..3], type_id, &sequence[3..6]);
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
            sequence: sequence[..last_index+6].to_string(),
        }, last_index + 6)
    }
    
    fn total_version_numbers(&self) -> u64 {
        return self.version + match self.content {
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
    
    fn total_number_packets(&self) -> u64 {
        println!("Seq: {}", self.sequence);
        return 1 + match self.content {
            PacketType::Literal(_) => {
                println!("Literal version {}", self.version);
                0
            },
            PacketType::Operator(ref o) => {
                println!("Operator version {}", self.version);
                let mut total = 0;
                for p in o.subpackets.iter() {
                    total += p.total_number_packets();
                }
                total
            },
        }
    }
}


fn parse_file(filename: &str) -> String {
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
    result.into_iter().collect()
}

fn part1(filename: &str) -> u64 {
    let sequence = parse_file(filename);
    let (packet, _) = Packet::new(&sequence);
    println!("Total packet numbers: {}", packet.total_number_packets());
    packet.total_version_numbers()
}

fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");

    let part1_result = part1(&filename);
    println!("Result of part 1: {}", part1_result);

}