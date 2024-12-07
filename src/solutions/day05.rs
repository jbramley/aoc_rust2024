use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Lines};

fn is_valid(printing: &Vec<u32>, orderings: &HashMap<u32, HashSet<u32>>) -> bool {
    for i in 0..printing.len() - 1 {
        for j in i + 1..printing.len() {
            if orderings[&printing[j]].contains(&printing[i]) {
                return false;
            }
        }
    }
    true
}

fn get_orderings(lines: &mut Lines<BufReader<File>>) -> HashMap<u32, HashSet<u32>> {
    let mut orderings: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in lines.by_ref().flatten() {
        if line.is_empty() {
            break;
        }
        let (x, y) = line
            .split("|")
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();
        orderings.entry(x).or_insert(HashSet::new()).insert(y);
    }
    orderings
}

pub fn part1(mut lines: Lines<BufReader<File>>) -> String {
    let orderings = get_orderings(&mut lines);

    let mut middle_sums: u32 = 0;
    for line in lines.flatten() {
        let x = line
            .split(",")
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        if is_valid(&x, &orderings) {
            middle_sums += x[(x.len() - 1) / 2]
        }
    }
    format!("{}", middle_sums)
}

pub fn part2(mut lines: Lines<BufReader<File>>) -> String {
    let orderings = get_orderings(&mut lines);

    let mut middle_sums: u32 = 0;
    for line in lines.flatten() {
        let mut x = line
            .split(",")
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        if is_valid(&x, &orderings) {
            continue;
        }
        x.sort_by(|a, b| {
            if orderings[&a].contains(&b) {
                std::cmp::Ordering::Less
            } else if orderings[&b].contains(&a) {
                std::cmp::Ordering::Greater
            } else {
                a.cmp(b)
            }
        });
        middle_sums += x[(x.len() - 1) / 2];
    }
    format!("{}", middle_sums)
}
