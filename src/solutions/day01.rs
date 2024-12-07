use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::iter::zip;

pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let mut locations1 = vec![];
    let mut locations2 = vec![];
    for line in lines.flatten() {
        let (loc1, loc2) = line.split_once("   ").unwrap();
        locations1.push(loc1.parse::<i32>().unwrap());
        locations2.push(loc2.parse::<i32>().unwrap());
    }
    locations1.sort();
    locations2.sort();
    let diffs = zip(locations1, locations2)
        .map(|(x, y)| (x - y).abs())
        .reduce(|x, y| x + y)
        .unwrap();
    format!("{}\n", diffs)
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let mut locations1 = vec![];
    let mut locations2: HashMap<i32, i32> = HashMap::new();
    for line in lines.flatten() {
        let (loc1, loc2) = line.split_once("   ").unwrap();
        locations1.push(loc1.parse::<i32>().unwrap());
        locations2
            .entry(loc2.parse::<i32>().unwrap())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
    let similarity = locations1
        .into_iter()
        .fold(0, |acc, x| acc + (x * locations2.get(&x).unwrap_or(&0)));
    format!("{}\n", similarity)
}
