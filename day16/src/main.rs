use aoc_utils;
use bitvec::vec::BitVec;

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: usize,
    inner_packet: InnerPacket,
}

impl Packet {
    fn new(version: usize, inner_packet: InnerPacket) -> Packet {
        Packet {
            version,
            inner_packet,
        }
    }
}

fn sum_packet_versions(packet: &Packet) -> usize {
    let sub = match &packet.inner_packet {
        InnerPacket::Literal(_) => 0,
        InnerPacket::Operator(op) => op
            .sub_packets
            .iter()
            .map(sum_packet_versions)
            .sum::<usize>(),
    };
    sub + packet.version
}

fn evaluate(packet: &Packet) -> usize {
    match &packet.inner_packet {
        InnerPacket::Literal(l) => l.value,
        InnerPacket::Operator(op) => {
            let mut sub_values = op.sub_packets.iter().map(evaluate);
            match op.type_id {
                0 => sub_values.sum(),
                1 => sub_values.product(),
                2 => sub_values.min().unwrap(),
                3 => sub_values.max().unwrap(),
                5 => (sub_values.next().unwrap() > sub_values.next().unwrap()) as usize,
                6 => (sub_values.next().unwrap() < sub_values.next().unwrap()) as usize,
                7 => (sub_values.next().unwrap() == sub_values.next().unwrap()) as usize,
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LiteralPacket {
    value: usize,
}

impl LiteralPacket {
    fn new(bv: &BitVec, i: &mut usize) -> LiteralPacket {
        let mut value_v = BitVec::new();
        let mut read = true;
        while read {
            read &= bv[*i];
            *i += 1;
            for _ in 0..4 {
                value_v.push(bv[*i]);
                *i += 1;
            }
        }
        let mut ii = 0;
        let vl = value_v.len();
        let value = number(&value_v, &mut ii, vl);
        LiteralPacket { value }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum InnerPacket {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

#[derive(Debug, PartialEq, Eq)]
struct OperatorPacket {
    type_id: usize,
    length_type_id: LengthTypeId,
    sub_packets: Vec<Packet>,
}

impl OperatorPacket {
    fn new(bv: &BitVec, type_id: usize, i: &mut usize) -> OperatorPacket {
        let length_type = bv[*i];
        *i += 1;
        let length_type_id = match length_type {
            false => LengthTypeId::LengthOfSub(number(bv, i, 15)),
            true => LengthTypeId::NumberOfSub(number(bv, i, 11)),
        };
        let mut sub_packets = Vec::new();
        match length_type_id {
            LengthTypeId::LengthOfSub(total_length) => {
                let starting_bit_location = *i;
                while total_length != *i - starting_bit_location {
                    sub_packets.push(parse_packet(bv, i));
                }
            }
            LengthTypeId::NumberOfSub(number) => {
                for _ in 0..number {
                    sub_packets.push(parse_packet(bv, i))
                }
            }
        }

        OperatorPacket {
            type_id,
            length_type_id,
            sub_packets,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum LengthTypeId {
    LengthOfSub(usize),
    NumberOfSub(usize),
}

fn number(bv: &BitVec, i: &mut usize, n: usize) -> usize {
    let value = (0..n)
        .map(|j| bv[*i + j])
        .rev()
        .enumerate()
        .map(|(i, bit)| if bit { 1 << i } else { 0 })
        .sum();
    *i += n;
    value
}

fn parse_packet(bv: &BitVec, i: &mut usize) -> Packet {
    let version = number(bv, i, 3);
    let type_id = number(bv, i, 3);
    match type_id {
        4 => Packet::new(version, InnerPacket::Literal(LiteralPacket::new(bv, i))),
        _ => Packet::new(
            version,
            InnerPacket::Operator(OperatorPacket::new(bv, type_id, i)),
        ),
    }
}

fn part_one() {
    if let Some(line) = aoc_utils::read_input_unparsed("./input.txt") {
        let mut bv: BitVec = BitVec::new();
        line.chars()
            .flat_map(|c| {
                (0..4)
                    .rev()
                    .map(move |i| (c.to_digit(16).unwrap() as u8 >> i) & 1 == 1)
            })
            .for_each(|b| bv.push(b));
        let mut pos: usize = 0;
        let packet = parse_packet(&bv, &mut pos);
        let version_sum = sum_packet_versions(&packet);
        println!("Sum: {}", version_sum);
    }
}

fn part_two() {
    if let Some(line) = aoc_utils::read_input_unparsed("./input.txt") {
        let mut bv: BitVec = BitVec::new();
        line.chars()
            .flat_map(|c| {
                (0..4)
                    .rev()
                    .map(move |i| (c.to_digit(16).unwrap() as u8 >> i) & 1 == 1)
            })
            .for_each(|b| bv.push(b));
        let mut pos: usize = 0;
        let packet = parse_packet(&bv, &mut pos);

        println!("{:?}", evaluate(&packet));
    }
}
fn main() {
    part_one();
    part_two();
    println!("Hello, world!");
}
