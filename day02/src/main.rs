use aoc_utils;

#[derive(Debug)]
struct Instruction {
    step: String,
    value: u32,
}

fn part_one() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let instructions: Vec<Instruction> = lines
            .iter()
            .map(|x| {
                let mut y = x.split(" ");
                Instruction {
                    step: y.next().unwrap().to_string(),
                    value: y.next().unwrap().to_string().parse::<u32>().unwrap(),
                }
            })
            .collect();
        let mut depth: u32 = 0;
        let mut horizontal: u32 = 0;

        for instruction in instructions {
            match instruction.step.as_str() {
                "forward" => horizontal += instruction.value,
                "down" => depth += instruction.value,
                "up" => depth -= instruction.value,
                _ => println!("something wrong"),
            };
        }
        println!(
            "Part one depth: {} horizontal {} product {}",
            depth,
            horizontal,
            depth * horizontal
        );
    }
}

fn part_two() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let instructions: Vec<Instruction> = lines
            .iter()
            .map(|x| {
                let mut y = x.split(" ");
                Instruction {
                    step: y.next().unwrap().to_string(),
                    value: y.next().unwrap().to_string().parse::<u32>().unwrap(),
                }
            })
            .collect();

        let mut aim: u32 = 0;
        let mut depth: u32 = 0;
        let mut horizontal: u32 = 0;

        for instruction in instructions {
            match instruction.step.as_str() {
                "forward" => {
                    horizontal += instruction.value;
                    depth += aim * instruction.value;
                }
                // "forward" => horizontal += instruction.value,
                "down" => aim += instruction.value,
                "up" => aim -= instruction.value,
                _ => println!("something wrong"),
            };
        }
        println!(
            "Part two depth: {} horizontal {} product {}",
            depth,
            horizontal,
            depth * horizontal
        );
    }
}

fn main() {
    part_one();
    part_two();
    println!("Hello, world!");
}
