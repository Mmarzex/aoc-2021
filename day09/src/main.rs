use aoc_utils;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

fn part_one() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let grid: Vec<Vec<i64>> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<i64>().unwrap())
                    .collect()
            })
            .collect();

        let mut low_points: Vec<i64> = Vec::new();

        for (y, row) in grid.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                let d: bool = [-1, 0, 1]
                    .iter()
                    .cartesian_product([-1, 0, 1])
                    .map(|d| (*d.0 as i64, d.1 as i64))
                    .filter(|(dx, dy)| dx.abs() != dy.abs())
                    .filter_map(|(dx, dy)| {
                        if (dx == -1 && x > 0) || (dx == 1 && x < row.len() - 1) {
                            return Some(grid[y][x + dx as usize]);
                        }

                        if (dy == -1 && y > 0) || (dy == 1 && y < grid.len() - 1) {
                            return Some(grid[y + dy as usize][x]);
                        }
                        Option::None
                    })
                    .all(|vv| vv > *v);
                if d {
                    low_points.push(*v + 1);
                }
            }
        }

        let low_points_sum: i64 = low_points.iter().sum();

        println!("Sum {}", low_points_sum);
    }
}

fn part_two() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let grid: Vec<Vec<i64>> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<i64>().unwrap())
                    .collect()
            })
            .collect();

        let mut low_points: Vec<(usize, usize)> = Vec::new();

        for (y, row) in grid.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                let d: bool = [-1, 0, 1]
                    .iter()
                    .cartesian_product([-1, 0, 1])
                    .map(|d| (*d.0 as i64, d.1 as i64))
                    .filter(|(dx, dy)| dx.abs() != dy.abs())
                    .filter_map(|(dx, dy)| {
                        if (dx == -1 && x > 0) || (dx == 1 && x < row.len() - 1) {
                            return Some(grid[y][x + dx as usize]);
                        }

                        if (dy == -1 && y > 0) || (dy == 1 && y < grid.len() - 1) {
                            return Some(grid[y + dy as usize][x]);
                        }
                        Option::None
                    })
                    .all(|vv| vv > *v);
                if d {
                    low_points.push((y, x));
                }
            }
        }

        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut basins: Vec<usize> = Vec::new();
        let h = grid.len();
        let w = grid[0].len();

        for (ly, lx) in low_points {
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            queue.push_back((ly, lx));
            let mut size: usize = 0;
            while !queue.is_empty() {
                let (yy, xx) = queue.pop_front().unwrap();
                if grid[yy][xx] == 9 || visited.contains(&(yy, xx)) {
                    continue;
                }
                if xx != 0 {
                    queue.push_back((yy, xx - 1));
                }

                if xx < w - 1 {
                    queue.push_back((yy, xx + 1));
                }

                if yy != 0 {
                    queue.push_back((yy - 1, xx));
                }

                if yy < h - 1 {
                    queue.push_back((yy + 1, xx));
                }

                size += 1;
                visited.insert((yy, xx));
            }
            basins.push(size);
        }

        let top_three_basins: usize = basins.iter().sorted().rev().take(3).product();

        println!("{:?}", top_three_basins);
    }
}

fn main() {
    part_one();
    part_two();
}
