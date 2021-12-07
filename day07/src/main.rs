use aoc_utils;
use itertools::Itertools;
use std::collections::HashMap;

fn part_one() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let positions: Vec<i64> = lines
            .iter()
            .flat_map(|line| line.split(',').map(|l| l.parse::<i64>().unwrap()))
            .sorted()
            .collect();

        let mut fuel_costs: HashMap<i64, i64> = HashMap::new();

        for position in &positions {
            if !fuel_costs.contains_key(&position) {
                for p in &positions {
                    let fuel_cost = (p - position).abs();
                    let entry = fuel_costs.entry(*position).or_insert(0);
                    *entry += fuel_cost;
                }
            }
        }

        let lowest_fuel_cost = fuel_costs.values().min_by(|a, b| a.cmp(b));
        println!("{:?}", lowest_fuel_cost);
    }
}

fn determine_fuel_cost(a: i64, b: i64) -> i64 {
    let diff = (a - b).abs();
    (diff * (diff + 1)) / 2
}

fn part_two() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let positions: Vec<i64> = lines
            .iter()
            .flat_map(|line| line.split(',').map(|l| l.parse::<i64>().unwrap()))
            .sorted()
            .collect();

        let bound = positions.iter().max().unwrap();

        let mut fuel_costs: HashMap<i64, i64> = HashMap::new();

        for position in 0..*bound {
            if !fuel_costs.contains_key(&position) {
                for p in &positions {
                    let fuel_cost = determine_fuel_cost(*p, position);
                    let entry = fuel_costs.entry(position).or_insert(0);
                    *entry += fuel_cost;
                }
            }
        }

        let lowest_fuel_cost = fuel_costs.values().min_by(|a, b| a.cmp(b));
        println!("{:?}", lowest_fuel_cost);
    }
}

fn main() {
    part_one();
    part_two();
}
