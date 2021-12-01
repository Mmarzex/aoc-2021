use aoc_utils;
use itertools::Itertools;

fn part_one() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let nums: Vec<i32> = lines.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        let increases = nums
            .iter()
            .zip(nums.iter().skip(1))
            .filter(|(a, b)| *a < *b)
            .count();
        println!("{}", increases);
    }
}

fn part_two() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let sums: Vec<i32> = lines
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .collect();

        let increases = sums
            .iter()
            .zip(sums.iter().skip(1))
            .filter(|(a, b)| *a < *b)
            .count();

        println!("{}", increases);
    }
}

fn main() {
    part_one();
    part_two();
}
