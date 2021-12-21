use aoc_utils;
use std::collections::HashMap;

type Coord = (i32, i32);

fn adjacents() -> Vec<Vec<Coord>> {
    vec![
        vec![(-1, -1), (0, -1), (1, -1)],
        vec![(-1, 0), (0, 0), (1, 0)],
        vec![(-1, 1), (0, 1), (1, 1)],
    ]
}

fn get_translation_index(default: char, coords: &mut HashMap<Coord, char>, coord: Coord) -> isize {
    let (x, y) = coord;
    let bin_str: String = adjacents()
        .iter()
        .map(|row| {
            row.iter()
                .map(|(dx, dy)| {
                    let n: Coord = (x + dx, y + dy);
                    let e = coords.get(&n).unwrap_or(&default);
                    match *e {
                        '.' => '0',
                        '#' => '1',
                        _ => unreachable!(),
                    }
                })
                .collect::<Vec<char>>()
        })
        .flatten()
        .collect();

    isize::from_str_radix(&*bin_str, 2).unwrap()
}

fn infinity(algo: Vec<char>, i: i8) -> char {
    match algo[0] {
        '.' => '.',
        '#' => match i % 2 {
            0 => '.',
            _ => '#',
        },
        _ => unreachable!(),
    }
}

fn part_one() {
    if let Some(input) = aoc_utils::read_input_unparsed("./input.txt") {
        let (algo_str, input_img_str) = input.split_once("\n\n").unwrap();

        let algo: Vec<char> = algo_str.chars().collect();
        let input_img_v: Vec<Vec<char>> =
            input_img_str.lines().map(|l| l.chars().collect()).collect();

        let mut coords: HashMap<Coord, char> = HashMap::new();

        for (y, row) in input_img_v.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                let mut e = coords.entry((x as i32, y as i32)).or_default();
                *e = *v;
            }
        }

        for i in 0..2 {
            let mut result: HashMap<Coord, char> = HashMap::new();
            println!("ENHANCING!!!");
            let default_char = infinity(algo.clone(), i);
            let min = *coords.iter().min_by_key(|(k, _)| *k).unwrap().0;
            let max = *coords.iter().max_by_key(|(k, _)| *k).unwrap().0;
            for y in (min.1 - 1)..=(max.1 + 1) {
                for x in (min.0 - 1)..=(max.0 + 1) {
                    let translation_idx = get_translation_index(default_char, &mut coords, (x, y));
                    *result.entry((x, y)).or_default() = algo[translation_idx as usize];
                }
            }
            coords = result.clone();
        }

        let lit_pixels = coords.values().filter(|v| **v == '#').count();

        println!("{:?}", lit_pixels);
    }
}

fn part_two() {
    if let Some(input) = aoc_utils::read_input_unparsed("./input.txt") {
        let (algo_str, input_img_str) = input.split_once("\n\n").unwrap();

        let algo: Vec<char> = algo_str.chars().collect();
        let input_img_v: Vec<Vec<char>> =
            input_img_str.lines().map(|l| l.chars().collect()).collect();

        let mut coords: HashMap<Coord, char> = HashMap::new();

        for (y, row) in input_img_v.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                let mut e = coords.entry((x as i32, y as i32)).or_default();
                *e = *v;
            }
        }

        for i in 0..50 {
            let mut result: HashMap<Coord, char> = HashMap::new();
            println!("ENHANCING!!!");
            let default_char = infinity(algo.clone(), i);
            let min = *coords.iter().min_by_key(|(k, _)| *k).unwrap().0;
            let max = *coords.iter().max_by_key(|(k, _)| *k).unwrap().0;
            for y in (min.1 - 1)..=(max.1 + 1) {
                for x in (min.0 - 1)..=(max.0 + 1) {
                    let translation_idx = get_translation_index(default_char, &mut coords, (x, y));
                    *result.entry((x, y)).or_default() = algo[translation_idx as usize];
                }
            }
            coords = result.clone();
        }

        let lit_pixels = coords.values().filter(|v| **v == '#').count();

        println!("{:?}", lit_pixels);
    }
}
fn main() {
    part_two();
}
