use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let mut stones = lines
        .flatten()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    for _ in 0..25 {
        let mut new_stones: Vec<String> = Vec::new();
        stones.iter().for_each(|s| {
            if s == "0" {
                new_stones.push("1".to_string());
            } else if s.len() % 2 == 0 {
                let s = s.split_at(s.len() / 2);
                [s.0, s.1]
                    .iter()
                    .map(|sp| format!("{}", sp.parse::<u64>().unwrap()))
                    .for_each(|sx| new_stones.push(sx));
            } else {
                new_stones.push(format!("{}", s.parse::<u64>().unwrap() * 2024u64));
            }
        });
        stones = new_stones.clone();
        // println!("{}: {:?}", stones.len(), stones);
    }
    format!("{}", stones.len())
}

pub fn count_stones_after_blinks(
    s: &str,
    blinks: u8,
    memo: &mut HashMap<(String, u8), u64>,
) -> u64 {
    if memo.contains_key(&(s.to_string(), blinks)) {
        memo[&(s.to_string(), blinks)]
    } else if blinks == 0 {
        1
    } else if s == "0" {
        let ct = count_stones_after_blinks("1", blinks - 1, memo);
        memo.insert((s.to_string(), blinks), ct);
        ct
    } else if s.len() % 2 == 0 {
        let st = s.split_at(s.len() / 2);
        let ct = [st.0, st.1]
            .iter()
            .map(|sp| {
                count_stones_after_blinks(
                    format!("{}", sp.parse::<u64>().unwrap()).as_str(),
                    blinks - 1,
                    memo,
                )
            })
            .sum();
        memo.insert((s.to_string(), blinks), ct);
        ct
    } else {
        let ct = count_stones_after_blinks(
            format!("{}", s.parse::<u64>().unwrap() * 2024u64).as_str(),
            blinks - 1,
            memo,
        );
        memo.insert((s.to_string(), blinks), ct);
        ct
    }
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let mut stones = lines
        .flatten()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut memo: HashMap<(String, u8), u64> = HashMap::new();
    let mut total_stones = 0u64;
    for s in stones {
        total_stones += count_stones_after_blinks(s.as_str(), 75, &mut memo);
    }
    format!("{}", total_stones)
}
