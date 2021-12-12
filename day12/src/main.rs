use aoc_utils;
use std::collections::{HashMap, HashSet, VecDeque};

fn is_lowercase(a: &str) -> bool {
    a.to_lowercase() == a
}

fn search<'a>(
    g: &'a HashMap<&str, Vec<&str>>,
    current: &'a str,
    visited: &'a mut HashSet<&str>,
    has_visited_two_small: bool,
) -> usize {
    if current == "end" {
        return 1;
    }
    let mut count: usize = 0;
    for e in g.get(current).unwrap() {
        let mut v = visited.clone();
        let mut visited_two_small = has_visited_two_small;
        if is_lowercase(e) {
            if !v.contains(e) {
                v.insert(e);
            } else if v.contains(e) && !visited_two_small && *e != "start" && *e != "end" {
                visited_two_small = true;
            } else {
                continue;
            }
        }
        count += search(g, e, &mut v, visited_two_small);
    }
    count
}

fn part_one() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let mut g: HashMap<&str, Vec<&str>> = HashMap::new();
        for line in lines.iter() {
            let (e1, e2) = line.split_once('-').unwrap();
            g.entry(e1).or_default().push(e2);
            g.entry(e2).or_default().push(e1);
        }
        let mut v: HashSet<&str> = HashSet::new();
        v.insert("start");
        println!("Count {}", search(&g, "start", &mut v, true));
    }
}

fn part_two() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let mut g: HashMap<&str, Vec<&str>> = HashMap::new();
        for line in lines.iter() {
            let (e1, e2) = line.split_once('-').unwrap();
            g.entry(e1).or_default().push(e2);
            g.entry(e2).or_default().push(e1);
        }
        let mut v: HashSet<&str> = HashSet::new();
        v.insert("start");
        println!("Count {}", search(&g, "start", &mut v, false));
    }
}

fn main() {
    part_one();
    part_two();
}
