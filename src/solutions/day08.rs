use gcd::Gcd;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Lines};

fn add_diff(x: usize, dir: i32, diff: usize) -> Option<usize> {
    let x = x as i32 + (dir * diff as i32);
    if x < 0 {
        None
    } else {
        Some(x as usize)
    }
}
pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let mut antenna: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let lines = lines.flatten().collect::<Vec<String>>();
    let max_row = lines.len();
    let max_col = lines[0].len();
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                antenna.entry(c).or_insert(Vec::new()).push((row, col));
            }
        }
    }
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for (_, nodes) in antenna.iter() {
        if nodes.len() <= 1 {
            continue;
        }
        nodes.iter().combinations(2).for_each(|v| {
            let node1 = v[0];
            let node2 = v[1];
            let ydir: i32 = match node1.0.cmp(&node2.0) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            let xdir: i32 = match node1.1.cmp(&node2.1) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            let ydiff = node1.0.abs_diff(node2.0);
            let xdiff = node1.1.abs_diff(node2.1);
            if let Some(new_y) = add_diff(node1.0, ydir, ydiff) {
                if let Some(new_x) = add_diff(node1.1, xdir, xdiff) {
                    if new_y < max_row && new_x < max_col {
                        antinodes.insert((new_y, new_x));
                    }
                }
            }
            if let Some(new_y) = add_diff(node2.0, -ydir, ydiff) {
                if let Some(new_x) = add_diff(node2.1, -xdir, xdiff) {
                    if new_y < max_row && new_x < max_col {
                        antinodes.insert((new_y, new_x));
                    }
                }
            }
        });
    }
    format!("{}", antinodes.len())
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let mut antenna: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let lines = lines.flatten().collect::<Vec<String>>();
    let max_row = lines.len() as isize;
    let max_col = lines[0].len() as isize;
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                antenna.entry(c).or_insert(Vec::new()).push((row, col));
            }
        }
    }
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for (_, nodes) in antenna.iter() {
        if nodes.len() <= 1 {
            continue;
        }
        nodes.iter().combinations(2).for_each(|v| {
            let node1 = v[0];
            let node2 = v[1];
            let ydiff = node1.0.abs_diff(node2.0);
            let xdiff = node1.1.abs_diff(node2.1);
            let sd = ydiff.gcd(xdiff);
            let ydir: isize = match node1.0.cmp(&node2.0) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            let xdir: isize = match node1.1.cmp(&node2.1) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            let ystep = ydir * ydiff as isize / sd as isize;
            let xstep = xdir * xdiff as isize / sd as isize;
            let mut y = node1.0 as isize;
            let mut x = node1.1 as isize;
            antinodes.insert((y, x));
            loop {
                y += ystep;
                x += xstep;
                if y >= 0 && y < max_row && x >= 0 && x < max_col {
                    antinodes.insert((y, x));
                } else {
                    break;
                }
            }
            let mut y = node1.0 as isize;
            let mut x = node1.1 as isize;
            loop {
                y -= ystep;
                x -= xstep;
                if y >= 0 && y < max_row && x >= 0 && x < max_col {
                    antinodes.insert((y, x));
                } else {
                    break;
                }
            }
        });
    }
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '.' && antinodes.contains(&(row as isize, col as isize)) {
                print!("#");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
    format!("{}", antinodes.len())
}
