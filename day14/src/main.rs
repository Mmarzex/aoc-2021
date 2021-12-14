#[macro_use]
extern crate scan_fmt;

use aoc_utils;
use itertools::Itertools;
use std::collections::HashMap;

fn part_one() {
    if let Some(input) = aoc_utils::read_input_unparsed("./input.txt") {
        let (polymer_template_str, insertion_rules_str) = input.split_once("\n\n").unwrap();
        let mut polymer_template: Vec<char> = polymer_template_str.chars().collect();
        let insertion_rules: Vec<((char, char), char)> = insertion_rules_str
            .lines()
            .map(|v| {
                let (pair, v) = scan_fmt!(v, "{} -> {}", String, char).unwrap();
                let split_pair: Vec<char> = pair.chars().collect();
                ((split_pair[0], split_pair[1]), v)
            })
            .collect();

        for _ in 0..10 {
            let mut i: usize = 0;
            while i < polymer_template.len() - 1 {
                let current_pair = (polymer_template[i], polymer_template[i + 1]);
                let rule_match = insertion_rules.iter().find(|(p, _)| current_pair == *p);
                if let Some(rule) = rule_match {
                    polymer_template.insert(i + 1, rule.1);
                    i += 2;
                } else {
                    i += 1;
                }
            }
        }

        let counts = polymer_template.iter().counts();
        let most_common = counts.values().max().unwrap();
        let least_common = counts.values().min().unwrap();
        println!("{}", most_common - least_common);
    }
}

fn part_two() {
    if let Some(input) = aoc_utils::read_input_unparsed("./input.txt") {
        let (polymer_template_str, insertion_rules_str) = input.split_once("\n\n").unwrap();
        let mut polymer_template: Vec<char> = polymer_template_str.chars().collect();
        let insertion_rules: HashMap<(char, char), char> = insertion_rules_str
            .lines()
            .map(|v| {
                let (pair, v) = scan_fmt!(v, "{} -> {}", String, char).unwrap();
                let split_pair: Vec<char> = pair.chars().collect();
                ((split_pair[0], split_pair[1]), v)
            })
            .collect();

        let mut counts: HashMap<(char, char), usize> = HashMap::new();

        for i in 0..polymer_template.len() - 1 {
            let mut e = counts
                .entry((polymer_template[i], polymer_template[i + 1]))
                .or_default();
            *e += 1;
        }

        for _ in 0..40 {
            let mut new_counts: HashMap<(char, char), usize> = HashMap::new();
            for k in counts.keys() {
                let e1_k = (k.0, insertion_rules[k]);
                let e2_k = (insertion_rules[k], k.1);
                let mut e1 = new_counts.entry(e1_k).or_default();
                *e1 += counts[k];
                let mut e2 = new_counts.entry(e2_k).or_default();
                *e2 += counts[k];
            }
            counts = new_counts;
        }

        let mut result_counts: HashMap<char, usize> = HashMap::new();

        for k in counts.keys() {
            let mut e1 = result_counts.entry(k.0).or_default();
            *e1 += counts[k];
        }
        let last_char = polymer_template.last().unwrap();
        let mut last_e = result_counts.entry(*last_char).or_default();
        *last_e += 1;

        let (min, max) = result_counts.values().minmax().into_option().unwrap();

        println!("{}", max - min);
    }
}
fn main() {
    part_one();
    part_two();
}
