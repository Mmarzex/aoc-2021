use aoc_utils;
use pathfinding::directed::dijkstra;

const COORDS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn create_grid(first_tile: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut grid: Vec<Vec<usize>> = Vec::new();
    let template: Vec<Vec<usize>> = vec![
        vec![0, 1, 2, 3, 4],
        vec![1, 2, 3, 4, 5],
        vec![2, 3, 4, 5, 6],
        vec![3, 4, 5, 6, 7],
        vec![4, 5, 6, 7, 8],
    ];

    for row in template {
        for v in row {
            let new_tile: Vec<Vec<usize>> = first_tile
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|c| {
                            let new_c = v + c;
                            if new_c > 9 {
                                new_c - 9
                            } else {
                                new_c
                            }
                        })
                        .collect()
                })
                .collect();
        }
    }
    grid
}

fn part_two() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let grid: Vec<Vec<usize>> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|v| v.to_string().parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        let width = grid.len();

        let goal = (width as i32 * 5 - 1, width as i32 * 5 - 1);

        let result = dijkstra::dijkstra(
            &(0, 0),
            |&(x, y)| {
                COORDS
                    .iter()
                    .map(|&(dx, dy)| ((x + dx) as usize, (y + dy) as usize))
                    .filter(|(x, y)| (x / 5 < width && y / 5 < width))
                    .map(|(x, y)| {
                        grid.get(y % width)
                            .and_then(|row| row.get(x % width))
                            .map(|w| {
                                (
                                    (x as i32, y as i32),
                                    ((*w as usize + (x / width) + (y / width) - 1) % 9 + 1) as u32,
                                )
                            })
                    })
                    .flatten()
                    .collect::<Vec<_>>()
            },
            |&pos| pos == goal,
        );

        println!("{:?}", result.unwrap().1);
    }
}

fn part_one() {
    if let Some(lines) = aoc_utils::read_input("./input.txt") {
        let grid: Vec<Vec<usize>> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|v| v.to_string().parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        let goal = (grid.len() as i32 - 1, grid.len() as i32 - 1);

        let result = dijkstra::dijkstra(
            &(0, 0),
            |(x, y)| {
                COORDS
                    .iter()
                    .map(|(dx, dy)| {
                        grid.get((y + dy) as usize)
                            .and_then(|row| row.get((x + dx) as usize))
                            .map(|w| ((x + dx, y + dy), *w as i32))
                    })
                    .flatten()
                    .collect::<Vec<_>>()
            },
            |&pos| pos == goal,
        );

        println!("{:?}", result.unwrap().1);
    }
}
fn main() {
    part_one();
    part_two();
}
