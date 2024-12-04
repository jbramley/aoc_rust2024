use itertools::Itertools;
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let m = lines
        .flatten()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect::<Vec<_>>()
        })
        .filter(|nums| nums.iter().all(|n| (*n).abs() > 0 && (*n).abs() <= 3))
        .map(|nums| {
            nums.iter()
                .map(|n| n.signum())
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect::<Vec<_>>()
        })
        .filter(|nums| nums.iter().all(|n| *n == 0))
        .count();
    format!("{:?}", m)
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let m = lines
        .flatten()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|line| {
            line.iter()
                .enumerate()
                .map(|(i, _n)| {
                    let mut new_line = line.clone();
                    new_line.remove(i);
                    new_line
                })
                .map(|permutation| permutation.iter().tuple_windows().map(|(a, b)| b - a).collect::<Vec<_>>())
                .filter(|nums| nums.iter().all(|n| (*n).abs() > 0 && (*n).abs() <= 3))
                .map(|nums| {
                    nums.iter()
                        .map(|n| n.signum())
                        .tuple_windows()
                        .map(|(a, b)| b - a)
                        .collect::<Vec<_>>()
                })
                .filter(|nums| nums.iter().all(|n| *n == 0))
                .collect::<Vec<_>>()
        })
        .filter(|v| v.len() > 0)
        .collect::<Vec<_>>()
        .len();
    format!("{:?}", m)
}
