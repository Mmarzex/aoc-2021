use aoc_utils;

fn part_one() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let inputs: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();
        let mut gamma_bits: Vec<char> = Vec::new();
        let mut epsilon_bits: Vec<char> = Vec::new();

        for i in 0..inputs[0].len() {
            let mut zeroes: i64 = 0;
            let mut ones: i64 = 0;
            for r in &inputs {
                if r[i] == '0' {
                    zeroes += 1;
                } else {
                    ones += 1;
                }
            }
            if ones > zeroes {
                gamma_bits.push('1');
                epsilon_bits.push('0');
            } else {
                gamma_bits.push('0');
                epsilon_bits.push('1');
            }
        }
        let gamma: String = gamma_bits.iter().collect();
        let epsilon: String = epsilon_bits.iter().collect();
        let gamma_int = isize::from_str_radix(gamma.as_str(), 2).unwrap();
        let epsilon_int = isize::from_str_radix(epsilon.as_str(), 2).unwrap();
        println!(
            "gamma {} epsilon {} product {}",
            gamma_int,
            epsilon_int,
            gamma_int * epsilon_int
        );
    }
}

fn part_two() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let inputs: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();

        let mut oxygen_inputs = inputs.clone();
        for i in 0..12 {
            let mut zeroes: i64 = 0;
            let mut ones: i64 = 0;
            for r in &oxygen_inputs {
                if r[i] == '0' {
                    zeroes += 1;
                } else {
                    ones += 1;
                }
            }

            let c = if zeroes == ones {
                '1'
            } else if zeroes > ones {
                '0'
            } else {
                '1'
            };

            oxygen_inputs = oxygen_inputs
                .iter()
                .filter(|&r| r[i] == c)
                .map(|r| r.clone())
                .collect();

            if oxygen_inputs.len() == 1 {
                break;
            }
        }

        let mut co2_inputs = inputs.clone();
        for i in 0..12 {
            let mut zeroes: i64 = 0;
            let mut ones: i64 = 0;
            for r in &co2_inputs {
                if r[i] == '0' {
                    zeroes += 1;
                } else {
                    ones += 1;
                }
            }

            let c = if zeroes == ones {
                '0'
            } else if zeroes > ones {
                '1'
            } else {
                '0'
            };

            co2_inputs = co2_inputs
                .iter()
                .filter(|&r| r[i] == c)
                .map(|r| r.clone())
                .collect();

            if co2_inputs.len() == 1 {
                break;
            }
        }

        let oxygen_binary: String = oxygen_inputs.first().unwrap().iter().collect();
        let co2_binary: String = co2_inputs.first().unwrap().iter().collect();

        let oxygen = isize::from_str_radix(oxygen_binary.as_str(), 2).unwrap();
        let co2 = isize::from_str_radix(co2_binary.as_str(), 2).unwrap();

        println!("oxygen {} co2 {} product {}", oxygen, co2, oxygen * co2);
    }
}

fn main() {
    part_one();
    part_two();
}
