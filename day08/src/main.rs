#[macro_use]
extern crate scan_fmt;

use aoc_utils;
use itertools::Itertools;
use linked_hash_set::LinkedHashSet;
use std::collections::{HashMap, HashSet};
const VALIDS: [i64; 4] = [2, 4, 3, 7];

fn part_one() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let output_lines: Vec<i64> = lines
            .iter()
            .flat_map(|line| {
                let line_split = line.split(" | ");
                let output = line_split.last().unwrap();
                output.split(' ').map(|x| x.to_string().len() as i64)
            })
            .filter(|x| VALIDS.contains(x))
            .collect();

        println!("{:?}", output_lines.len());
    }
}

fn parse_signal(signals: &Vec<String>) -> HashMap<String, i64> {
    let mut translation: HashMap<String, i64> = HashMap::new();
    let signal_sets: Vec<HashSet<char>> = signals.iter().map(|x| x.chars().collect()).collect();
    let eight = signal_sets.iter().find(|signal| signal.len() == 7).unwrap();
    let seven = signal_sets.iter().find(|signal| signal.len() == 3).unwrap();
    let four = signal_sets.iter().find(|signal| signal.len() == 4).unwrap();
    let one = signal_sets.iter().find(|signal| signal.len() == 2).unwrap();
    let nine = signal_sets
        .iter()
        .find(|signal| signal.len() == 6 && signal.intersection(four).count() == 4)
        .unwrap();
    let zero = signal_sets
        .iter()
        .find(|signal| {
            signal.len() == 6 && signal.intersection(seven).count() == 3 && *signal != nine
        })
        .unwrap();
    let three = signal_sets
        .iter()
        .find(|signal| signal.len() == 5 && signal.intersection(seven).count() == 3)
        .unwrap();
    let six = signal_sets
        .iter()
        .find(|signal| signal.len() == 6 && *signal != zero && *signal != nine)
        .unwrap();
    let five = signal_sets
        .iter()
        .find(|signal| {
            signal.len() == 5 && signal.intersection(six).count() == 5 && *signal != three
        })
        .unwrap();
    let two = signal_sets
        .iter()
        .find(|signal| signal.len() == 5 && *signal != three && *signal != five)
        .unwrap();

    translation.insert(eight.iter().sorted().collect(), 8);
    translation.insert(seven.iter().sorted().collect(), 7);
    translation.insert(four.iter().sorted().collect(), 4);
    translation.insert(one.iter().sorted().collect(), 1);
    translation.insert(nine.iter().sorted().collect(), 9);
    translation.insert(zero.iter().sorted().collect(), 0);
    translation.insert(three.iter().sorted().collect(), 3);
    translation.insert(six.iter().sorted().collect(), 6);
    translation.insert(five.iter().sorted().collect(), 5);
    translation.insert(two.iter().sorted().collect(), 2);
    translation
}

fn part_two() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let input: Vec<(Vec<String>, Vec<String>)> = lines
            .iter()
            .map(|line| {
                let mut line_split = line.split(" | ");
                (
                    line_split
                        .next()
                        .unwrap()
                        .split(' ')
                        .map(|x| x.to_string())
                        .collect(),
                    line_split
                        .next()
                        .unwrap()
                        .split(' ')
                        .map(|x| x.to_string())
                        .collect(),
                )
            })
            .collect();

        let mut ans: i64 = 0;

        for (signals, output) in input {
            let translation = parse_signal(&signals);
            let output_str: String = output
                .iter()
                .map(|x| {
                    let xx: String = x.chars().sorted().collect();
                    translation.get(&xx).unwrap().to_string()
                })
                .collect();
            let output_num: i64 = output_str.parse().unwrap();
            ans += output_num;
        }
        println!("{}", ans);
    }
}
fn main() {
    part_one();
    part_two();
}
