use aoc_utils;
use itertools::Itertools;
use std::collections::VecDeque;

fn inv(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn score_completion(stack: &VecDeque<char>) -> usize {
    stack
        .iter()
        .map(|v| match v {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => unreachable!(),
        })
        .fold(0, |acc, x| (acc * 5) + x)
}

fn part_two() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let mut incomplete_lines: Vec<String> = Vec::new();
        for line in lines {
            let mut stack: VecDeque<char> = VecDeque::new();
            let mut is_corrupt = false;
            for c in line.chars() {
                if c == '(' || c == '[' || c == '{' || c == '<' {
                    stack.push_front(c);
                } else if inv(stack.pop_front().unwrap()) != c {
                    is_corrupt = true;
                    break;
                }
            }
            if !stack.is_empty() && !is_corrupt {
                incomplete_lines.push(line);
            }
        }

        let mut scores: Vec<usize> = Vec::new();

        for line in incomplete_lines {
            let mut stack: VecDeque<char> = VecDeque::new();
            for c in line.chars() {
                if c == '(' || c == '[' || c == '{' || c == '<' {
                    stack.push_front(c);
                } else if inv(stack.pop_front().unwrap()) != c {
                    break;
                }
            }
            scores.push(score_completion(&stack));
        }

        scores = scores.iter().sorted().map(|x| *x).collect();

        let mid_point: usize = scores.len() / 2;

        println!("{:?} mid_point {}", scores[mid_point], mid_point);
    }
}

fn part_one() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let mut invalid_lines: Vec<String> = Vec::new();
        let mut score: usize = 0;
        for line in lines {
            let mut stack: VecDeque<char> = VecDeque::new();
            for c in line.chars() {
                if c == '(' || c == '[' || c == '{' || c == '<' {
                    stack.push_front(c);
                } else if inv(stack.pop_front().unwrap()) != c {
                    invalid_lines.push(line);
                    score += match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => panic!("oh no"),
                    };
                    break;
                }
            }
        }
        println!("{}", score);
    }
}

fn main() {
    part_one();
    part_two();
}
