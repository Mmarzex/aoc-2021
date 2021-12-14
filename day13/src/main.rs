#[macro_use]
extern crate scan_fmt;

use aoc_utils;

fn pp_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        let s: String = row.iter().collect();
        println!("{}", s);
    }
}

fn merge_y(grid: Vec<Vec<char>>, y: usize) -> Vec<Vec<char>> {
    let mut new_grid: Vec<Vec<char>> = grid.clone();
    new_grid.remove(y);
    let mut chunks: Vec<Vec<Vec<char>>> = new_grid
        .chunks(new_grid.len() / 2)
        .map(|x| x.to_vec())
        .collect();
    let mut top: Vec<Vec<char>> = chunks.first().unwrap().clone();
    let bottom: Vec<Vec<char>> = chunks[1].iter().rev().map(|v| v.to_vec()).collect();

    // MERGE
    for (y, row) in bottom.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            let b_v = bottom[y][x];
            if b_v == '#' {
                top[y][x] = '#';
            }
        }
    }
    top
}

fn merge_x(grid: Vec<Vec<char>>, x: usize) -> Vec<Vec<char>> {
    let new_grid: Vec<Vec<char>> = grid.clone();
    let mut left: Vec<Vec<char>> = Vec::new();

    for mut row in new_grid {
        row.remove(x);
        let mut chunks: Vec<Vec<char>> = row.chunks(row.len() / 2).map(|x| x.to_vec()).collect();
        let mut r: Vec<char> = chunks.first().unwrap().clone();
        let l: Vec<char> = chunks[1].iter().rev().map(|c| *c).collect();
        for (i, v) in l.iter().map(|c| *c).enumerate() {
            if v == '#' {
                r[i] = '#';
            }
        }
        left.push(r);
    }

    left
}

fn part_two() {
    if let Some(input) = aoc_utils::read_input_unparsed("./input.txt") {
        let (coordinates_str, folds_str) = input.split_once("\n\n").unwrap();
        let coordinates: Vec<(usize, usize)> = coordinates_str
            .split("\n")
            .map(|l| scan_fmt!(l, "{d},{d}", usize, usize).unwrap())
            .collect();
        let folds: Vec<(char, usize)> = folds_str
            .split("\n")
            .map(|l| scan_fmt!(l, "fold along {}={d}", char, usize).unwrap())
            .collect();

        let max_width = coordinates
            .iter()
            .max_by(|(x1, _), (x2, _)| x1.cmp(x2))
            .unwrap()
            .0
            + 1;
        let max_height = coordinates
            .iter()
            .max_by(|(_, y1), (_, y2)| y1.cmp(y2))
            .unwrap()
            .1
            + 1;

        let mut grid: Vec<Vec<char>> = (0..max_height).map(|_| vec!['.'; max_width]).collect();
        for (x, y) in coordinates {
            grid[y][x] = '#';
        }

        for (a, v) in folds {
            grid = match a {
                'x' => merge_x(grid, v),
                'y' => merge_y(grid, v),
                _ => unreachable!(),
            }
        }

        pp_grid(&grid);
    }
}

fn part_one() {
    if let Some(input) = aoc_utils::read_input_unparsed("./input.txt") {
        let (coordinates_str, folds_str) = input.split_once("\n\n").unwrap();
        let coordinates: Vec<(usize, usize)> = coordinates_str
            .split("\n")
            .map(|l| scan_fmt!(l, "{d},{d}", usize, usize).unwrap())
            .collect();
        let folds: Vec<(char, usize)> = folds_str
            .split("\n")
            .map(|l| scan_fmt!(l, "fold along {}={d}", char, usize).unwrap())
            .collect();
        let first_fold = folds.first().unwrap();

        let max_width = coordinates
            .iter()
            .max_by(|(x1, _), (x2, _)| x1.cmp(x2))
            .unwrap()
            .0
            + 1;
        let max_height = coordinates
            .iter()
            .max_by(|(_, y1), (_, y2)| y1.cmp(y2))
            .unwrap()
            .1
            + 1;

        println!("{:?}", first_fold);
        println!("max width {:?} max height {:?}", max_width, max_height);

        let mut grid: Vec<Vec<char>> = (0..max_height).map(|_| vec!['.'; max_width]).collect();
        for (x, y) in coordinates {
            grid[y][x] = '#';
        }
        if first_fold.0 == 'y' {
            let top = merge_y(grid, first_fold.1);
            let dots_count: usize = top
                .iter()
                .map(|row| row.iter().filter(|v| **v == '#').count())
                .sum();

            println!("Dots {}", dots_count);
        } else {
            let column_to_remove: usize = first_fold.1;
            let mut new_grid: Vec<Vec<char>> = merge_x(grid, column_to_remove);
            let dots_count: usize = new_grid
                .iter()
                .map(|row| row.iter().filter(|v| **v == '#').count())
                .sum();

            println!("Dots {}", dots_count);
        }
    }
}

fn main() {
    part_one();
    part_two();
}
