#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;

use aoc_utils;
use nalgebra::Vector2;

fn part_one() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let points: Vec<(Vector2<i64>, Vector2<i64>)> = lines
            .iter()
            .map(
                |line| match scan_fmt!(line, "{d},{d} -> {d},{d}", i64, i64, i64, i64) {
                    Ok((x1, y1, x2, y2)) => (Vector2::new(x1, y1), Vector2::new(x2, y2)),
                    _ => panic!("oh no"),
                },
            )
            .collect();
        let overlaps = points
            .iter()
            .fold(
                HashMap::<Vector2<i64>, i64>::new(),
                |mut acc, (start, end)| {
                    let d = end - start;
                    let dir = if d.y == 0 {
                        Vector2::new(d.x.signum(), 0)
                    } else if d.x == 0 {
                        Vector2::new(0, d.y.signum())
                    } else {
                        return acc;
                    };

                    let mut s = *start;
                    while s != *end + dir {
                        acc.entry(s).and_modify(|v| *v += 1).or_insert(1);
                        s += dir;
                    }
                    acc
                },
            )
            .iter()
            .filter(|(_, v)| **v > 1)
            .count();

        println!("Overlaps {}", overlaps);
    }
}

fn part_two() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let points: Vec<(Vector2<i64>, Vector2<i64>)> = lines
            .iter()
            .map(
                |line| match scan_fmt!(line, "{d},{d} -> {d},{d}", i64, i64, i64, i64) {
                    Ok((x1, y1, x2, y2)) => (Vector2::new(x1, y1), Vector2::new(x2, y2)),
                    _ => panic!("oh no"),
                },
            )
            .collect();

        let overlaps = points
            .iter()
            .fold(
                HashMap::<Vector2<i64>, i64>::new(),
                |mut acc, (start, end)| {
                    let d = end - start;
                    let dir = if d.y == 0 {
                        Vector2::new(d.x.signum(), 0)
                    } else if d.x == 0 {
                        Vector2::new(0, d.y.signum())
                    } else {
                        Vector2::new(d.x.signum(), d.y.signum())
                    };

                    let mut s = *start;
                    while s != *end + dir {
                        acc.entry(s).and_modify(|v| *v += 1).or_insert(1);
                        s += dir;
                    }
                    acc
                },
            )
            .iter()
            .filter(|(_, v)| **v > 1)
            .count();

        println!("Overlaps {}", overlaps);
    }
}

fn main() {
    part_one();
    part_two();
}
