use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{stdin, BufReader, Lines, Read};

pub fn part1(lines: Lines<BufReader<File>>) -> String {
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;
    const MID_X: i32 = WIDTH / 2;
    const MID_Y: i32 = HEIGHT / 2;

    let robot_re = Regex::new("p=(\\d+),(\\d+) v=(-?\\d+),(-?\\d+)").unwrap();
    let mut quadrants = (0, 0, 0, 0);
    for line in lines.flatten() {
        let x = robot_re
            .captures_iter(line.as_str())
            .map(|c| c.extract())
            .map(|(_, [px, py, vx, vy])| {
                (
                    px.parse::<i32>().unwrap(),
                    py.parse::<i32>().unwrap(),
                    vx.parse::<i32>().unwrap(),
                    vy.parse::<i32>().unwrap(),
                )
            })
            .next()
            .unwrap();
        let mut fx = (x.0 + 100 * x.2) % WIDTH;
        let mut fy = (x.1 + 100 * x.3) % HEIGHT;
        if fx < 0 {
            fx += WIDTH
        }
        if fy < 0 {
            fy += HEIGHT
        }

        if fx < MID_X && fy < MID_Y {
            quadrants.0 += 1;
        } else if fx < MID_X && fy > MID_Y {
            quadrants.1 += 1;
        } else if fx > MID_X && fy < MID_Y {
            quadrants.2 += 1;
        } else if fx > MID_X && fy > MID_Y {
            quadrants.3 += 1;
        }
    }
    format!("{}", quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3)
}

struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn display_robots(robots: &Vec<Robot>, width: i32, height: i32) {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];
    for r in robots {
        grid[r.y as usize][r.x as usize] = '#';
    }
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn maybe_a_tree(robots: &Vec<Robot>, width: i32, height: i32) -> bool {
    // Attempt 1: Assume the tree is in the middle
    // let mid_x = width / 2;
    //
    // robots.iter().filter(|r| r.x == mid_x).count() as i32 > height / 2

    // Attempt 2: Assume the tree is just somewhere
    let xs: HashMap<i32, i32> = robots.iter().fold(HashMap::new(), |mut acc, r| {
        *acc.entry(r.x).or_insert(0) += 1;
        acc
    });
    *xs.values().max().unwrap() > 20
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;

    let robot_re = Regex::new("p=(\\d+),(\\d+) v=(-?\\d+),(-?\\d+)").unwrap();
    let mut robots: Vec<Robot> = Vec::new();
    lines.flatten().for_each(|line| {
        let x = robot_re
            .captures_iter(line.as_str())
            .map(|c| c.extract())
            .map(|(_, [px, py, vx, vy])| Robot {
                x: px.parse::<i32>().unwrap(),
                y: py.parse::<i32>().unwrap(),
                vx: vx.parse::<i32>().unwrap(),
                vy: vy.parse::<i32>().unwrap(),
            })
            .next()
            .unwrap();
        robots.push(x);
    });
    let mut i = 0;
    loop {
        if maybe_a_tree(&robots, WIDTH, HEIGHT) {
            display_robots(&robots, WIDTH, HEIGHT);
            println!("Second {}", i);
            let _ = stdin().read(&mut [0u8]).unwrap();
        }
        for r in robots.iter_mut() {
            r.x = (r.x + r.vx) % WIDTH;
            r.y = (r.y + r.vy) % HEIGHT;
            if r.x < 0 {
                r.x += WIDTH
            }
            if r.y < 0 {
                r.y += HEIGHT
            }
        }
        i += 1;
        if i > 1000000 {
            break;
        }
    }
    format!("{}", i)
}
