use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let topomap = lines
        .flatten()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let max_y = topomap.len() - 1;
    let max_x = topomap[0].len() - 1;
    let mut total = 0;
    for (row, line) in topomap.iter().enumerate() {
        line.iter()
            .enumerate()
            .filter(|(_, c)| **c == 0)
            .for_each(|(col, c)| {
                let mut trails: VecDeque<(usize, usize, &u32)> = VecDeque::new();
                let mut trailends: HashSet<(usize, usize)> = HashSet::new();
                trails.push_back((row, col, c));
                while !trails.is_empty() {
                    if let Some((y, x, c)) = trails.pop_front() {
                        if *c == 9 {
                            trailends.insert((x, y));
                            continue;
                        }
                        if y > 0 && topomap[y - 1][x] == *c + 1 {
                            trails.push_back((y - 1, x, &topomap[y - 1][x]));
                        }
                        if y < max_y && topomap[y + 1][x] == *c + 1 {
                            trails.push_back((y + 1, x, &topomap[y + 1][x]));
                        }
                        if x > 0 && topomap[y][x - 1] == *c + 1 {
                            trails.push_back((y, x - 1, &topomap[y][x - 1]));
                        }
                        if x < max_x && topomap[y][x + 1] == *c + 1 {
                            trails.push_back((y, x + 1, &topomap[y][x + 1]));
                        }
                    }
                }
                total += trailends.len();
            })
    }

    format!("{}", total)
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let topomap = lines
        .flatten()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let max_y = topomap.len() - 1;
    let max_x = topomap[0].len() - 1;
    let mut total = 0;
    for (row, line) in topomap.iter().enumerate() {
        line.iter()
            .enumerate()
            .filter(|(_, c)| **c == 0)
            .for_each(|(col, c)| {
                let mut trails: VecDeque<(usize, usize, &u32)> = VecDeque::new();
                trails.push_back((row, col, c));
                while !trails.is_empty() {
                    if let Some((y, x, c)) = trails.pop_front() {
                        if *c == 9 {
                            total += 1;
                            continue;
                        }
                        if y > 0 && topomap[y - 1][x] == *c + 1 {
                            trails.push_back((y - 1, x, &topomap[y - 1][x]));
                        }
                        if y < max_y && topomap[y + 1][x] == *c + 1 {
                            trails.push_back((y + 1, x, &topomap[y + 1][x]));
                        }
                        if x > 0 && topomap[y][x - 1] == *c + 1 {
                            trails.push_back((y, x - 1, &topomap[y][x - 1]));
                        }
                        if x < max_x && topomap[y][x + 1] == *c + 1 {
                            trails.push_back((y, x + 1, &topomap[y][x + 1]));
                        }
                    }
                }
            })
    }

    format!("{}", total)
}
