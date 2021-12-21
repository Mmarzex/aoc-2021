use itertools::iproduct;
use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::combinator::map;
use nom::sequence::{delimited, separated_pair};
use nom::{Finish, IResult};
use std::ops::{Add, ControlFlow};
use std::str::FromStr;

#[derive(Debug)]
enum SnailFish {
    Regular(u32),
    Pair(Box<SnailFish>, Box<SnailFish>),
}

impl SnailFish {
    fn new_pair(a: SnailFish, b: SnailFish) -> SnailFish {
        SnailFish::Pair(Box::new(a), Box::new(b))
    }

    fn value(&self) -> Option<u32> {
        match self {
            SnailFish::Regular(v) => Some(*v),
            _ => None,
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            SnailFish::Regular(v) => *v,
            SnailFish::Pair(a, b) => (a.magnitude() * 3) + (b.magnitude() * 2),
        }
    }

    fn eval(mut self) -> Self {
        loop {
            while let ControlFlow::Break(explosion) = self.check_explode(0, 0) {
                self.explode(0, &explosion);
            }
            if !self.split() {
                return self;
            }
        }
    }

    fn check_explode(
        &mut self,
        index: usize,
        depth: usize,
    ) -> ControlFlow<(usize, u32, u32), usize> {
        match self {
            SnailFish::Pair(a, b) if depth == 4 => {
                let (a, b) = (a.value().unwrap(), b.value().unwrap());
                *self = SnailFish::Regular(0);
                ControlFlow::Break((index, a, b))
            }
            SnailFish::Pair(a, b) => b.check_explode(a.check_explode(index, depth + 1)?, depth + 1),
            SnailFish::Regular(_) => ControlFlow::Continue(index + 1),
        }
    }

    fn explode(
        &mut self,
        index: usize,
        e @ (i, a, b): &(usize, u32, u32),
    ) -> ControlFlow<(), usize> {
        match self {
            SnailFish::Regular(n) if index + 1 == *i => {
                *self = SnailFish::Regular(*n + a);
                ControlFlow::Continue(index + 1)
            }
            SnailFish::Regular(n) if index == *i + 1 => {
                *self = SnailFish::Regular(*n + b);
                ControlFlow::Break(())
            }
            SnailFish::Regular(_) => ControlFlow::Continue(index + 1),
            SnailFish::Pair(a, b) => b.explode(a.explode(index, e)?, e),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            SnailFish::Regular(v) if *v >= 10 => {
                *self = SnailFish::Regular(*v / 2) + SnailFish::Regular((*v + 1) / 2);
                true
            }
            SnailFish::Regular(_) => false,
            SnailFish::Pair(a, b) => a.split() || b.split(),
        }
    }
}

impl Clone for SnailFish {
    fn clone(&self) -> Self {
        match self {
            SnailFish::Regular(n) => SnailFish::Regular(*n),
            SnailFish::Pair(a, b) => a.as_ref().clone() + b.as_ref().clone(),
        }
    }
}

impl FromStr for SnailFish {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
            .finish()
            .map(|(_, v)| v)
            .map_err(|e| nom::error::Error::new(e.input.to_owned(), e.code))
    }
}

impl Add<SnailFish> for SnailFish {
    type Output = SnailFish;

    fn add(self, r: Self) -> Self::Output {
        SnailFish::Pair(Box::new(self), Box::new(r))
    }
}

fn parse(line: &str) -> IResult<&str, SnailFish> {
    alt((
        map(nom::character::complete::u32, SnailFish::Regular),
        delimited(
            tag("["),
            map(separated_pair(parse, tag(","), parse), |(a, b)| a + b),
            tag("]"),
        ),
    ))(line)
}

fn part_one() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        // let input: Vec<SnailFish> = lines.iter().map(parse).map(|(_, v)| v).collect();
        let input: Vec<SnailFish> = lines
            .iter()
            .map(|l| SnailFish::from_str(l).unwrap())
            .collect();

        let result = input
            .into_iter()
            .reduce(|a, b| (a + b).eval())
            .unwrap()
            .magnitude();
        println!("{:?}", result);
    }
}

fn part_two() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let input: Vec<SnailFish> = lines
            .iter()
            .map(|l| SnailFish::from_str(l).unwrap())
            .collect();

        // let max_magnitude = input
        //     .into_iter()
        //     .tuple_combinations()
        //     .map(|(a, b)| {
        //         let one = (a.clone() + b.clone()).eval().magnitude();
        //         let two = (a + b).eval().magnitude();
        //         one.max(two)
        //     })
        //     .max()
        //     .unwrap();

        let max_magnitude = iproduct!(0..input.len(), 0..input.len())
            .filter_map(|(a, b)| {
                (a != b).then(|| (input[a].clone() + input[b].clone()).eval().magnitude())
            })
            .max()
            .unwrap();

        println!("Max {}", max_magnitude);
    }
}

fn main() {
    part_one();
    part_two();
}
