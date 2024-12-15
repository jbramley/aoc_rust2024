use itertools::Itertools;
use std::cmp::min;
use std::fs::File;
use std::io::{BufReader, Lines};

fn parse_button(button_str: &str) -> (u64, u64) {
    let v = button_str
        .split_ascii_whitespace()
        .skip(2)
        .map(|s| {
            let (_, s) = s.split_at(2);
            s.trim_end_matches(",").parse::<u64>().unwrap()
        })
        .collect::<Vec<_>>();
    (v[0], v[1])
}

fn parse_prize(prize_str: &str) -> (u64, u64) {
    let v = prize_str
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| {
            let (_, s) = s.split_at(2);
            s.trim_end_matches(",").parse::<u64>().unwrap()
        })
        .collect::<Vec<_>>();
    (v[0], v[1])
}

fn parse_prize_p2(prize_str: &str) -> (u64, u64) {
    let v = prize_str
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| {
            let (_, s) = s.split_at(2);
            s.trim_end_matches(",").parse::<u64>().unwrap()
        })
        .collect::<Vec<_>>();
    (v[0] + 10000000000000, v[1] + 10_000_000_000_000)
}
pub fn part1(lines: Lines<BufReader<File>>) -> String {
    const TOKENS_A: u64 = 3;
    const TOKENS_B: u64 = 1;

    let mut min_tokens = 0u64;
    for mut arcade in &lines.flatten().chunks(4) {
        let a = parse_button(arcade.next().unwrap().as_str());
        let b = parse_button(arcade.next().unwrap().as_str());
        let prize = parse_prize(arcade.next().unwrap().as_str());
        let max_a = min(100, min(prize.0 / a.0, prize.1 / a.1));
        let mut tokens = 401;
        for i in 0..=max_a {
            let x = prize.0 - i * a.0;
            let y = prize.1 - i * a.1;
            if x % b.0 == 0 && y % b.1 == 0 && x / b.0 == y / b.1 && x / b.0 <= 100 {
                let t = i * TOKENS_A + (x / b.0) * TOKENS_B;
                if t < tokens {
                    tokens = t;
                }
            }
        }
        if tokens < 400 {
            println!(
                "Prize ({},{}) gotten in {} tokens",
                prize.0, prize.1, tokens
            );
            min_tokens += tokens;
        } else {
            println!("Prize ({},{}) can't be won", prize.0, prize.1);
        }
    }
    format!("{}", min_tokens)
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    const TOKENS_A: u64 = 3;
    const TOKENS_B: u64 = 1;

    let mut min_tokens = 0u64;
    for mut arcade in &lines.flatten().chunks(4) {
        let a = parse_button(arcade.next().unwrap().as_str());
        let (ax, ay) = (a.0 as f64, a.1 as f64);
        let b = parse_button(arcade.next().unwrap().as_str());
        let (bx, by) = (b.0 as f64, b.1 as f64);
        let p = parse_prize_p2(arcade.next().unwrap().as_str());
        let (px, py) = (p.0 as f64, p.1 as f64);

        let ca = (px * by - py * bx) / (ax * by - ay * bx);
        let cb = (px - ax * ca) / bx;
        if ca % 1.0 != 0.0 || cb % 1.0 != 0.0 {
            continue;
        }
        min_tokens += ca as u64 * TOKENS_A + cb as u64 * TOKENS_B;
    }
    format!("{}", min_tokens)
}
