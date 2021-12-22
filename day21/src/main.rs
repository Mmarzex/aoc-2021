#[macro_use]
extern crate scan_fmt;

use aoc_utils;
use itertools::{iproduct, Itertools};
use std::collections::HashMap;

fn part_one() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let starting_positions: Vec<(usize, usize)> = lines
            .iter()
            .map(|line| scan_fmt!(line, "Player {d} starting position: {d}", usize, usize).unwrap())
            .collect();

        let mut dice_value: usize = 1;
        let mut rolls: usize = 0;
        let mut p1_pos: usize = starting_positions[0].1;
        let mut p2_pos: usize = starting_positions[1].1;
        let mut p1_score: usize = 0;
        let mut p2_score: usize = 0;
        let mut current_player: usize = 1;

        while p1_score < 1000 && p2_score < 1000 {
            let values: usize = (0..3)
                .map(|_| {
                    let d = dice_value.clone();
                    dice_value += 1;
                    if dice_value > 100 {
                        dice_value = 1;
                    }
                    rolls += 1;
                    d
                })
                .sum();
            if current_player == 1 {
                for _ in 1..=values {
                    if p1_pos == 10 {
                        p1_pos = 1;
                    } else {
                        p1_pos += 1
                    }
                }
                p1_score += p1_pos;
                current_player = 2;
            } else {
                for _ in 1..=values {
                    if p2_pos == 10 {
                        p2_pos = 1;
                    } else {
                        p2_pos += 1;
                    }
                }
                p2_score += p2_pos;
                current_player = 1;
            }
        }

        let result = vec![p1_score, p2_score].iter().min().unwrap() * rolls;

        println!("p1 {} p2 {} rolls {}", p1_score, p2_score, rolls);
        println!("Result {}", result);
    }
}

type MemoCache = HashMap<(usize, usize, usize, usize, bool), (usize, usize)>;

fn quantum(
    memo: &mut MemoCache,
    p1_pos: usize,
    p2_pos: usize,
    p1_score: usize,
    p2_score: usize,
    p1_turn: bool,
) -> (usize, usize) {
    if p1_score >= 21 {
        return (1, 0);
    }
    if p2_score >= 21 {
        return (0, 1);
    }

    if let Some(score) = memo.get(&(p1_pos, p2_pos, p1_score, p2_score, p1_turn)) {
        return *score;
    }

    if let Some(&score) = memo.get(&(p2_pos, p1_pos, p2_score, p1_score, !p1_turn)) {
        return (score.1, score.0);
    }

    let mut score = (0, 0);
    for (d1, d2, d3) in iproduct!([1, 2, 3], [1, 2, 3], [1, 2, 3]) {
        let d = d1 + d2 + d3;
        let (s1, s2) = if p1_turn {
            let p1_pos = p1_pos + d - if p1_pos + d > 10 { 10 } else { 0 };
            quantum(memo, p1_pos, p2_pos, p1_score + p1_pos, p2_score, false)
        } else {
            let p2_pos = p2_pos + d - if p2_pos + d > 10 { 10 } else { 0 };
            quantum(memo, p1_pos, p2_pos, p1_score, p2_score + p2_pos, true)
        };
        score.0 += s1;
        score.1 += s2;
    }

    memo.insert((p1_pos, p2_pos, p1_score, p2_score, p1_turn), score);
    score
}

fn part_two() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let starting_positions: Vec<(usize, usize)> = lines
            .iter()
            .map(|line| scan_fmt!(line, "Player {d} starting position: {d}", usize, usize).unwrap())
            .collect();

        let mut p1_pos: usize = starting_positions[0].1;
        let mut p2_pos: usize = starting_positions[1].1;
        let scores = quantum(&mut HashMap::new(), p1_pos, p2_pos, 0, 0, true);
        let most_wins = scores.0.max(scores.1);
        println!("scores {:?}", most_wins);
    }
}
fn main() {
    part_one();
    part_two();
}
