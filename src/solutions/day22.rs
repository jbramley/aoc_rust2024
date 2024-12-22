use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let mut total = 0u64;

    for line in lines.flatten() {
        let mut x = line.parse::<u64>().unwrap();
        for _ in 0..2000 {
            x = (x * 64) ^ x;
            x = x % 16777216;

            x = (x / 32) ^ x;
            x = x % 16777216;

            x = (x * 2048) ^ x;
            x = x % 16777216;
        }
        println!("{}: {}", line, x);
        total += x;
    }
    format!("{}", total)
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let mut seq: HashMap<(i8, i8, i8, i8), u64> = HashMap::new();

    for line in lines.flatten() {
        let mut x = line.parse::<u64>().unwrap();
        let mut last5 = [-10i8; 5];
        let mut seen: HashSet<(i8, i8, i8, i8)> = HashSet::new();
        for _ in 0..2000 {
            x = (x * 64) ^ x;
            x = x % 16777216;

            x = (x / 32) ^ x;
            x = x % 16777216;

            x = (x * 2048) ^ x;
            x = x % 16777216;

            let d = (x % 10) as i8;
            last5[0] = last5[1];
            last5[1] = last5[2];
            last5[2] = last5[3];
            last5[3] = last5[4];
            last5[4] = d;
            if last5[0] > -10 {
                let k = (
                    last5[1] - last5[0],
                    last5[2] - last5[1],
                    last5[3] - last5[2],
                    last5[4] - last5[3],
                );
                if seen.contains(&k) {
                    continue;
                }
                seq.entry(k)
                    .and_modify(|t| *t += d as u64)
                    .or_insert(d as u64);
                seen.insert(k);
                // println!("({}, {}, {}, {}): {}", last5[1] - last5[0], last5[2] - last5[1], last5[3] - last5[2], last5[4] - last5[3], d)
            }
        }
        // println!("-----------------------------");
    }
    let max = seq.values().max().unwrap();
    seq.iter()
        .filter_map(|(k, v)| if v == max { Some(k) } else { None })
        .for_each(|k| println!("{:?}", k));
    format!("{}", max)
}
