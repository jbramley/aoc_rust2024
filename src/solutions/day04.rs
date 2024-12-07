use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let mut xs: HashSet<(usize, usize)> = HashSet::new();
    let mut ms: HashSet<(usize, usize)> = HashSet::new();
    let mut as_: HashSet<(usize, usize)> = HashSet::new();
    let mut ss: HashSet<(usize, usize)> = HashSet::new();

    for (row, line) in lines.flatten().enumerate() {
        line.chars()
            .into_iter()
            .enumerate()
            .for_each(|(col, c)| match c {
                'X' => {
                    xs.insert((row, col));
                }
                'M' => {
                    ms.insert((row, col));
                }
                'A' => {
                    as_.insert((row, col));
                }
                'S' => {
                    ss.insert((row, col));
                }
                _ => {}
            })
    }
    let mut xmas_count = 0;
    for x in xs.iter() {
        for rd in -1..=1 {
            for cd in -1..=1 {
                if rd == 0 && cd == 0 {
                    continue;
                }
                let mr = x.0 as i32 + rd;
                let ar = mr + rd;
                let sr = ar + rd;

                let mc = x.1 as i32 + cd;
                let ac = mc + cd;
                let sc = ac + cd;

                if sr < 0 || sc < 0 {
                    continue;
                }

                let mr = mr as usize;
                let mc = mc as usize;
                let ar = ar as usize;
                let ac = ac as usize;
                let sr = sr as usize;
                let sc = sc as usize;

                if ms.contains(&(mr, mc)) && as_.contains(&(ar, ac)) && ss.contains(&(sr, sc)) {
                    xmas_count += 1;
                }
            }
        }
    }

    format!("{}", xmas_count)
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let chars = lines
        .flatten()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let good_corners = ["MMSS", "MSMS", "SMSM", "SSMM"];
    let mut xmas_count = 0;
    for r in 0..chars.len() {
        for c in 0..chars[r].len() {
            let r2 = r + 2;
            let c2 = c + 2;
            let ar = r + 1;
            let ac = c + 1;
            if r2 >= chars.len() || c2 >= chars[r].len() {
                continue;
            }
            if chars[ar][ac] != 'A' {
                continue;
            }
            let corners = [chars[r][c], chars[r][c2], chars[r2][c], chars[r2][c2]]
                .iter()
                .collect::<String>();
            if good_corners.contains(&corners.as_str()) {
                xmas_count += 1;
            }
        }
    }
    format!("{}", xmas_count)
}
