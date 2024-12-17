use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let mut lines = lines.flatten();
    let mut ra = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(2)
        .next()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let mut rb = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(2)
        .next()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let mut rc = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(2)
        .next()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let _ = lines.next();
    let pgm = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .chars()
        .filter(|c| *c != ',')
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>();

    let mut pc = 0usize;
    let mut out: Vec<String> = Vec::new();
    while pc < pgm.len() {
        match pgm[pc] {
            0 => {
                let exp = match pgm[pc + 1] {
                    0..=3 => pgm[pc + 1],
                    4 => ra,
                    5 => rb,
                    6 => rc,
                    _ => panic!(),
                };
                ra = ra / 2u64.pow(exp as u32)
            }
            1 => rb = rb ^ pgm[pc + 1],
            2 => {
                let exp = match pgm[pc + 1] {
                    0..=3 => pgm[pc + 1],
                    4 => ra,
                    5 => rb,
                    6 => rc,
                    _ => panic!(),
                };
                rb = exp % 8
            }
            3 => {
                if ra != 0 {
                    pc = pgm[pc + 1] as usize;
                    continue;
                }
            }
            4 => rb = rb ^ rc,
            5 => {
                let exp = match pgm[pc + 1] {
                    0..=3 => pgm[pc + 1],
                    4 => ra,
                    5 => rb,
                    6 => rc,
                    _ => panic!(),
                };
                out.push(format!("{}", exp % 8))
            }
            6 => {
                let exp = match pgm[pc + 1] {
                    0..=3 => pgm[pc + 1],
                    4 => ra,
                    5 => rb,
                    6 => rc,
                    _ => panic!(),
                };
                rb = ra / 2u64.pow(exp as u32)
            }
            7 => {
                let exp = match pgm[pc + 1] {
                    0..=3 => pgm[pc + 1],
                    4 => ra,
                    5 => rb,
                    6 => rc,
                    _ => panic!(),
                };
                rc = ra / 2u64.pow(exp as u32)
            }
            _ => panic!(),
        }
        pc += 2;
    }
    format!("{}", out.join(","))
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let mut lines = lines.flatten();
    let _ = lines.next();
    let mut rb = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(2)
        .next()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let mut rc = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(2)
        .next()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let _ = lines.next();
    let pgm = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .chars()
        .filter(|c| *c != ',')
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>();

    let srb = rb;
    let src = rc;

    let mut q: VecDeque<u64> = VecDeque::new();
    let mut valid: Vec<u64> = Vec::new();
    q.push_back(0);
    while !q.is_empty() {
        println!("{:?}", q);
        // let _ = stdin().read(&mut[0u8]);
        let x = q.pop_front().unwrap();
        for i in 0..8 {
            let mut ra = i + x * 8;
            rb = srb;
            rc = src;
            let mut pc = 0usize;
            let mut out: Vec<u64> = Vec::new();
            while pc < pgm.len() {
                match pgm[pc] {
                    0 => {
                        let exp = match pgm[pc + 1] {
                            0..=3 => pgm[pc + 1],
                            4 => ra,
                            5 => rb,
                            6 => rc,
                            _ => panic!(),
                        };
                        ra = ra / 2u64.pow(exp as u32);
                        // println!("0,{}: ra: {:b}, rb: {:b}, rc: {:b}", pgm[pc+1], ra, rb, rc);
                    }
                    1 => {
                        rb = rb ^ pgm[pc + 1];
                        // println!("1,{}: ra: {:b}, rb: {:b}, rc: {:b}", pgm[pc+1], ra, rb, rc);
                    }
                    2 => {
                        let exp = match pgm[pc + 1] {
                            0..=3 => pgm[pc + 1],
                            4 => ra,
                            5 => rb,
                            6 => rc,
                            _ => panic!(),
                        };
                        rb = exp % 8;
                        // println!("2,{}: ra: {:b}, rb: {:b}, rc: {:b}", pgm[pc+1], ra, rb, rc);
                    }
                    3 => {
                        if ra != 0 {
                            pc = pgm[pc + 1] as usize;
                            continue;
                        }
                    }
                    4 => {
                        rb = rb ^ rc;
                        // println!("4,{}: ra: {:b}, rb: {:b}, rc: {:b}", pgm[pc+1], ra, rb, rc);
                    }
                    5 => {
                        let exp = match pgm[pc + 1] {
                            0..=3 => pgm[pc + 1],
                            4 => ra,
                            5 => rb,
                            6 => rc,
                            _ => panic!(),
                        };
                        out.push(exp % 8);
                        // println!("5,{}: ra: {:b}, rb: {:b}, rc: {:b}", pgm[pc+1], ra, rb, rc);
                    }
                    6 => {
                        let exp = match pgm[pc + 1] {
                            0..=3 => pgm[pc + 1],
                            4 => ra,
                            5 => rb,
                            6 => rc,
                            _ => panic!(),
                        };
                        rb = ra / 2u64.pow(exp as u32);
                        // println!("6,{}: ra: {:b}, rb: {:b}, rc: {:b}", pgm[pc+1], ra, rb, rc);
                    }
                    7 => {
                        let exp = match pgm[pc + 1] {
                            0..=3 => pgm[pc + 1],
                            4 => ra,
                            5 => rb,
                            6 => rc,
                            _ => panic!(),
                        };
                        rc = ra / 2u64.pow(exp as u32);
                        // println!("7,{}: ra: {:b}, rb: {:b}, rc: {:b}", pgm[pc+1], ra, rb, rc);
                    }
                    _ => panic!(),
                }
                pc += 2;
            }
            if out.len() <= pgm.len() && out == pgm[pgm.len() - out.len()..pgm.len()] {
                q.push_back(i + x * 8);
                if out == pgm {
                    valid.push(i + x * 8);
                }
            }
        }
    }
    format!("{}", valid.iter().min().unwrap())
}
