use aoc_utils;

fn part_one() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let mut fish: Vec<i64> = lines[0]
            .split(',')
            .map(|l| l.parse::<i64>().unwrap())
            .collect();

        for _ in 0..80 {
            let current_len = fish.len();
            for i in 0..current_len {
                fish[i] -= 1;
                if fish[i] == -1 {
                    fish[i] = 6;
                    fish.push(8);
                }
            }
        }

        println!("{}", fish.len());
    }
}

fn part_two() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let input: Vec<i64> = lines[0]
            .split(',')
            .map(|l| l.parse::<i64>().unwrap())
            .collect();

        let mut fish = [0i64; 9];

        for f in input {
            fish[f as usize] += 1;
        }

        for _ in 0..256 {
            fish.rotate_left(1);
            fish[6] += fish[8]
        }
        let fish_count: i64 = fish.iter().sum();

        println!("{:?}", fish_count);
    }
}

fn main() {
    part_one();
    part_two();
}
