use aoc_utils;
use std::collections::{HashMap, HashSet, VecDeque};

fn possible_points(y: isize, x: isize) -> Vec<(isize, isize)> {
    let possibles: Vec<(isize, isize)> = vec![
        (y, x - 1),
        (y, x + 1),
        (y - 1, x),
        (y + 1, x),
        (y + 1, x + 1),
        (y + 1, x - 1),
        (y - 1, x - 1),
        (y - 1, x + 1),
    ];
    possibles
}

fn run(grid: &mut HashMap<(isize, isize), usize>) -> usize {
    for v in grid.values_mut() {
        *v += 1;
    }

    let mut flashes: VecDeque<(isize, isize)> = grid
        .iter()
        .filter(|(_, v)| **v > 9)
        .map(|(p, _)| *p)
        .collect();

    let mut seen: HashSet<(isize, isize)> = HashSet::new();
    while !flashes.is_empty() {
        let p = flashes.pop_front().unwrap();
        if seen.contains(&p) {
            continue;
        }
        seen.insert(p);
        let possibles = possible_points(p.0, p.1);
        for pp in possibles {
            if let Some(v) = grid.get_mut(&pp) {
                *v += 1;
                if *v > 9 {
                    flashes.push_back(pp);
                }
            }
        }
    }
    for v in grid.values_mut() {
        if *v > 9 {
            *v = 0;
        }
    }
    seen.len()
}

fn parse_input(lines: Vec<String>) -> HashMap<(isize, isize), usize> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, v)| {
                    (
                        (y as isize, x as isize),
                        v.to_string().parse::<usize>().unwrap(),
                    )
                })
                .collect::<Vec<((isize, isize), usize)>>()
        })
        .collect()
}

fn part_one() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let mut grid = parse_input(lines);

        let flash_count: usize = (0..100).map(|v| run(&mut grid)).sum();
        println!("Flash Count {}", flash_count);
    }
}

fn part_two() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let mut grid = parse_input(lines);

        let mut step: usize = 1;
        loop {
            run(&mut grid);
            let all_flashed = grid.values().all(|v| *v == 0);
            if all_flashed {
                println!("All on {}", step);
                break;
            }
            step += 1;
        }
    }
}

fn main() {
    part_one();
    part_two();
}
