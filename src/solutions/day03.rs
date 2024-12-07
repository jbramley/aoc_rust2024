use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let v = lines.flatten().collect::<Vec<_>>().join("\n");
    let mulre = Regex::new("mul\\((\\d+),(\\d+)\\)").unwrap();
    let mut total: i32 = 0;
    for (_, [x, y]) in mulre.captures_iter(&v).map(|c| c.extract()) {
        total += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();
    }
    format!("Total: {}", total)
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let v = lines.flatten().collect::<Vec<_>>().join("\n");
    let mulre = Regex::new("(mul\\((\\d+),(\\d+)\\)|(do|don't)\\(\\))").unwrap();
    let mut total: i32 = 0;
    let mut do_mul = true;
    mulre
        .captures_iter(&v)
        .for_each(|c| match c.get(0).unwrap().as_str() {
            "do()" => do_mul = true,
            "don't()" => do_mul = false,
            _ => {
                if do_mul {
                    let x = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    let y = c.get(3).unwrap().as_str().parse::<i32>().unwrap();
                    total += x * y;
                }
            }
        });
    format!("Total: {}", total)
}
