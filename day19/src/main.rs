#[macro_use]
extern crate scan_fmt;

use aoc_utils;
use itertools::Itertools;
use std::collections::HashSet;

fn rotate([x, y, z]: [i32; 3], rot: u8) -> [i32; 3] {
    match rot {
        0 => [x, y, z],
        1 => [x, z, -y],
        2 => [x, -y, -z],
        3 => [x, -z, y],
        4 => [y, x, -z],
        5 => [y, z, x],
        6 => [y, -x, z],
        7 => [y, -z, -x],
        8 => [z, x, y],
        9 => [z, y, -x],
        10 => [z, -x, -y],
        11 => [z, -y, x],
        12 => [-x, y, -z],
        13 => [-x, z, y],
        14 => [-x, -y, z],
        15 => [-x, -z, -y],
        16 => [-y, x, z],
        17 => [-y, z, -x],
        18 => [-y, -x, -z],
        19 => [-y, -z, x],
        20 => [-z, x, -y],
        21 => [-z, y, x],
        22 => [-z, -x, y],
        23 => [-z, -y, -x],
        _ => unreachable!(),
    }
}

fn merge_scan(total_scan: &mut HashSet<[i32; 3]>, scan: &[[i32; 3]]) -> Option<[i32; 3]> {
    for rot in 0..24 {
        let r = scan.iter().map(|&v| rotate(v, rot)).collect::<Vec<_>>();
        let distances = total_scan
            .iter()
            .cartesian_product(&r)
            .map(|([x1, y1, z1], [x2, y2, z2])| [x1 - x2, y1 - y2, z1 - z2]);
        for [dx, dy, dz] in distances {
            let translated = r.iter().map(|[x, y, z]| [x + dx, y + dy, z + dz]);
            if translated
                .clone()
                .filter(|v| total_scan.contains(v))
                .count()
                >= 12
            {
                total_scan.extend(translated);
                return Some([dx, dy, dz]);
            }
        }
    }
    None
}

fn part_one() {
    if let Some(input) = aoc_utils::read_input_unparsed("./input.txt") {
        let mut scanners: Vec<Vec<[i32; 3]>> = input
            .split("\n\n")
            .map(|v| {
                v.lines()
                    .skip(1)
                    .map(|line| {
                        let (x, y, z) = scan_fmt!(line, "{d},{d},{d}", i32, i32, i32).unwrap();
                        [x, y, z]
                    })
                    .collect()
            })
            .collect();

        let mut total_scan: HashSet<[i32; 3]> = scanners.remove(0).into_iter().collect();
        let mut dists = Vec::new();
        while !scanners.is_empty() {
            for i in (0..scanners.len()).rev() {
                if let Some(d) = merge_scan(&mut total_scan, &scanners[i]) {
                    dists.push(d);
                    scanners.swap_remove(i);
                }
            }
        }

        let largest_m_dist = dists
            .iter()
            .tuple_combinations()
            .map(|([x1, y1, z1], [x2, y2, z2])| (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs())
            .max()
            .unwrap();

        println!("{:?}", total_scan.len());
        println!("{}", largest_m_dist);
    }
}
fn main() {
    part_one();
}
