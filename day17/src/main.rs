#[macro_use]
extern crate scan_fmt;

use aoc_utils;
use std::collections::HashSet;

fn try_initial_velocity(x1: i32, x2: i32, y1: i32, y2: i32, dx: i32, dy: i32) -> (bool, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut dx = dx;
    let mut dy = dy;

    let mut highest_y = i32::MIN;

    loop {
        x += dx;
        y += dy;
        if dx > 0 {
            dx -= 1;
        } else if dx < 0 {
            dx += 1;
        }
        dy -= 1;

        highest_y = highest_y.max(y);

        if x >= x1 && x <= x2 && y >= y1 && y <= y2 {
            return (true, highest_y);
        }

        if x > x2 || y < y1 {
            return (false, highest_y);
        }
    }
}

fn solve() {
    if let Some(input) = aoc_utils::read_input_unparsed("./input.txt") {
        let (x1, x2, y1, y2) = scan_fmt!(
            &input,
            "target area: x={d}..{d}, y={d}..{d}",
            i32,
            i32,
            i32,
            i32
        )
        .unwrap();

        let mut valids: Vec<i32> = Vec::new();

        let mut valid_initials: HashSet<(i32, i32)> = HashSet::new();

        for dx in 1..1000 {
            for dy in -1000..1000 {
                let (is_valid, highest_y) = try_initial_velocity(x1, x2, y1, y2, dx, dy);
                if is_valid {
                    valids.push(highest_y);
                    valid_initials.insert((dx, dy));
                }
            }
        }

        let highest_y = valids.iter().max().unwrap();

        println!("highest {:?}", highest_y);
        println!("Initial Velocities {}", valid_initials.len());
    }
}

fn main() {
    solve();
}
