use itertools::Itertools;
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let line = lines
        .flatten()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let mut checksum = 0u64;
    let mut i = 0usize;
    let mut block = 0u64;
    let mut j = line.len() - 1;
    let mut j_rem = line[j];
    loop {
        if i >= j {
            break;
        }
        for _ in 0..line[i] {
            checksum += block * (i as u64 / 2);
            block += 1;
        }
        i += 1;
        for _ in 0..line[i] {
            if j_rem == 0 {
                j -= 2;
                j_rem = line[j];
            }
            if j <= i {
                break;
            }
            checksum += block * (j as u64 / 2);
            j_rem -= 1;
            block += 1;
        }
        i += 1;
    }
    for _ in 0..j_rem {
        checksum += block * (j as u64 / 2);
        block += 1;
    }
    format!("{}", checksum)
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let line = lines
        .flatten()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let mut spaces = line.iter().skip(1).step_by(2).copied().collect::<Vec<_>>();
    let mut blocks = 0;
    let mut start_blocks = line
        .iter()
        .map(|x| {
            let y = blocks;
            blocks += x;
            y
        })
        .collect::<Vec<_>>();
    let max_id = line.len() - 1;
    let mut checksum = 0u64;

    line.iter().rev().enumerate().step_by(2).for_each(|(i, x)| {
        let id = (max_id - i) / 2;
        let mut block: u64;
        if let Some((pos, _)) = spaces.iter().find_position(|y| **y >= *x) {
            if (pos * 2 + 1) < id * 2 {
                block = start_blocks[pos * 2 + 1] as u64;
                spaces[pos] -= *x;
                start_blocks[pos * 2 + 1] += *x;
            } else {
                block = start_blocks[id * 2] as u64;
            }
        } else {
            block = start_blocks[id * 2] as u64;
        }
        for _ in 0..*x {
            checksum += block * id as u64;
            block += 1;
        }
    });
    format!("{}", checksum)
}
