#[macro_use]
extern crate scan_fmt;
use aoc_utils;
use nalgebra::Matrix5;

fn is_winner(board: &Matrix5<(i64, bool)>) -> bool {
    let row_won = board.row_iter().any(|row| row.iter().all(|(_, b)| *b));
    let column_won = board
        .column_iter()
        .any(|column| column.iter().all(|(_, b)| *b));
    row_won || column_won
}

fn get_score(num: i64, board: &Matrix5<(i64, bool)>) -> i64 {
    let sum: i64 = board
        .iter()
        .filter(|(_, b)| *b == false)
        .map(|(x, _)| *x)
        .sum();
    num * sum
}

fn part_one() {
    if let Some(mut lines) = aoc_utils::read_input("./input.txt") {
        let moves: Vec<i64> = lines
            .remove(0)
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let mut boards: Vec<Matrix5<(i64, bool)>> = lines
            .chunks(5)
            .map(|l| {
                Matrix5::from_iterator(l.iter().flat_map(|x| {
                    if let Ok((a, b, c, d, e)) =
                        scan_fmt!(x, "{d} {d} {d} {d} {d}", i64, i64, i64, i64, i64)
                    {
                        vec![(a, false), (b, false), (c, false), (d, false), (e, false)]
                    } else {
                        Vec::new()
                    }
                }))
            })
            .collect();

        for m in moves {
            for (i, board) in boards.iter_mut().enumerate() {
                board.iter_mut().for_each(|x| {
                    if x.0 == m {
                        x.1 = true;
                    }
                });
                if is_winner(&board) {
                    println!("Score {}", get_score(m, &board));
                    return;
                }
            }
        }
    }
}

fn part_two() {
    if let Some(mut lines) = aoc_utils::read_input("./input.txt") {
        let moves: Vec<i64> = lines
            .remove(0)
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let mut boards: Vec<Matrix5<(i64, bool)>> = lines
            .chunks(5)
            .map(|l| {
                Matrix5::from_iterator(l.iter().flat_map(|x| {
                    if let Ok((a, b, c, d, e)) =
                        scan_fmt!(x, "{d} {d} {d} {d} {d}", i64, i64, i64, i64, i64)
                    {
                        vec![(a, false), (b, false), (c, false), (d, false), (e, false)]
                    } else {
                        Vec::new()
                    }
                }))
            })
            .collect();

        let mut wins = vec![false; boards.len()];

        for m in moves {
            for (i, board) in boards.iter_mut().enumerate() {
                board.iter_mut().for_each(|x| {
                    if x.0 == m {
                        x.1 = true;
                    }
                });
                if is_winner(&board) {
                    wins[i] = true;
                    if wins.iter().all(|b| *b) {
                        println!("{:?}", board);
                        println!("Score {}", get_score(m, &board));
                        return;
                    }
                }
            }
        }
    }
}
fn main() {
    part_one();
    part_two();
}
