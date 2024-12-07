use std::fs::File;
use std::io::{BufReader, Lines};

pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let mut total = 0u64;
    for line in lines.flatten() {
        let x = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let values = x
            .iter()
            .skip(1)
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let goal = x[0].strip_suffix(":").unwrap().parse::<u64>().unwrap();

        let max_mask = 1u16 << (values.len() - 1);
        for i in 0..max_mask {
            let mut attempt = values[0];
            for j in 0..(values.len() - 1) {
                let bit = (i >> j) & 1;
                if bit == 0 {
                    attempt += values[j + 1];
                } else {
                    attempt *= values[j + 1];
                }
            }
            if attempt == goal {
                total += goal;
                break;
            }
        }
    }
    format!("{}", total)
}

fn trinary_increment(trinary_mask: &Vec<u8>) -> Vec<u8> {
    let mut carry = true;
    trinary_mask
        .iter()
        .rev()
        .map(|x| {
            if carry {
                if *x == 2 {
                    carry = true;
                    0
                } else {
                    carry = false;
                    x + 1
                }
            } else {
                *x
            }
        })
        .rev()
        .collect::<Vec<u8>>()
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let mut total = 0u64;
    for line in lines.flatten() {
        let x = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let values = x
            .iter()
            .skip(1)
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let goal = x[0].strip_suffix(":").unwrap().parse::<u64>().unwrap();

        let mut trinary_mask = vec![0u8; values.len() - 1];
        for _ in 0..3u32.pow(trinary_mask.len() as u32) {
            let mut attempt = values[0];
            for (j, bit) in trinary_mask.iter().enumerate() {
                match *bit {
                    0 => attempt += values[j + 1],
                    1 => attempt *= values[j + 1],
                    2 => {
                        let s = format!("{}{}", attempt, values[j + 1]);
                        attempt = s.parse::<u64>().unwrap();
                    }
                    _ => panic!(),
                }
            }
            trinary_mask = trinary_increment(&trinary_mask);
            if attempt == goal {
                total += goal;
                break;
            }
        }
    }
    format!("{}", total)
}
