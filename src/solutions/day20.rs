use itertools::Itertools;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufReader, Lines};

fn find_in_grid(grid: &Vec<Vec<char>>, c: char) -> Option<(usize, usize)> {
    grid.iter()
        .enumerate()
        .filter_map(|(r, ml)| {
            if let Some((col, _)) = ml.iter().find_position(|mc| **mc == c) {
                Some((col, r))
            } else {
                None
            }
        })
        .next()
}

#[derive(Hash)]
struct Node {
    x: usize,
    y: usize,
    p: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.p.eq(&other.p)
    }
}
impl Eq for Node {}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.p.partial_cmp(&other.p)
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.p.cmp(&other.p)
    }
}

fn adj(n: &Node, max_width: usize, max_height: usize) -> Vec<(usize, usize)> {
    let mut v: Vec<(usize, usize)> = Vec::new();
    if n.x > 0 {
        v.push((n.x - 1, n.y));
    }
    if n.x < max_width - 1 {
        v.push((n.x + 1, n.y));
    }
    if n.y > 0 {
        v.push((n.x, n.y - 1));
    }
    if n.y < max_height - 1 {
        v.push((n.x, n.y + 1));
    }
    v
}

fn adj2(n: (usize, usize), max_width: usize, max_height: usize) -> Vec<(usize, usize)> {
    let mut v: Vec<(usize, usize)> = Vec::new();
    if n.0 > 1 {
        v.push((n.0 - 2, n.1));
    }
    if n.0 < max_width - 2 {
        v.push((n.0 + 2, n.1));
    }
    if n.1 > 1 {
        v.push((n.0, n.1 - 2));
    }
    if n.1 < max_height - 2 {
        v.push((n.0, n.1 + 2));
    }
    v
}

pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let maze = lines
        .flatten()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>();
    let (sx, sy) = find_in_grid(&maze, 'S').unwrap();
    let (ex, ey) = find_in_grid(&maze, 'E').unwrap();
    let width = maze[0].len();
    let height = maze.len();

    let mut prev: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    for r in 0..height {
        for c in 0..width {
            prev.insert((c, r), None);
            dist.insert((c, r), usize::MAX);
        }
    }
    dist.entry((sx, sy)).and_modify(|d| *d = 0);
    let mut q: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    q.push(Reverse(Node { x: sx, y: sy, p: 0 }));

    while !q.is_empty() {
        let n = q.pop().unwrap().0;
        for (x, y) in adj(&n, width, height) {
            if maze[y][x] == '#' {
                continue;
            }
            if n.p + 1 < dist[&(x, y)] {
                dist.entry((x, y)).and_modify(|d| *d = n.p + 1);
                prev.entry((x, y)).and_modify(|p| *p = Some((n.x, n.y)));
                q.push(Reverse(Node { x, y, p: n.p + 1 }));
            }
        }
    }
    let mut shortest_path: HashMap<(usize, usize), usize> = HashMap::new();
    let (mut nx, mut ny) = (ex, ey);
    loop {
        shortest_path.insert((nx, ny), dist[&(nx, ny)]);
        if let Some((x, y)) = prev[&(nx, ny)] {
            (nx, ny) = (x, y);
        } else {
            break;
        }
    }
    println!("{}", shortest_path.len());
    let mut shortcuts = 0;
    for ((x, y), d) in shortest_path.iter() {
        if *d < 100 {
            continue;
        }
        for (nx, ny) in adj2((*x, *y), width, height) {
            if let Some(nd) = shortest_path.get(&(nx, ny)) {
                if *nd < *d - 100 {
                    shortcuts += 1;
                    // println!("Found a shortcut from ({}, {}) to ({}, {})", *x, *y, nx, ny);
                }
            }
        }
    }
    format!("{}", shortcuts)
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let maze = lines
        .flatten()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>();
    let (sx, sy) = find_in_grid(&maze, 'S').unwrap();
    let (ex, ey) = find_in_grid(&maze, 'E').unwrap();
    let width = maze[0].len();
    let height = maze.len();

    let mut prev: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    for r in 0..height {
        for c in 0..width {
            prev.insert((c, r), None);
            dist.insert((c, r), usize::MAX);
        }
    }
    dist.entry((sx, sy)).and_modify(|d| *d = 0);
    let mut q: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    q.push(Reverse(Node { x: sx, y: sy, p: 0 }));

    while !q.is_empty() {
        let n = q.pop().unwrap().0;
        for (x, y) in adj(&n, width, height) {
            if maze[y][x] == '#' {
                continue;
            }
            if n.p + 1 < dist[&(x, y)] {
                dist.entry((x, y)).and_modify(|d| *d = n.p + 1);
                prev.entry((x, y)).and_modify(|p| *p = Some((n.x, n.y)));
                q.push(Reverse(Node { x, y, p: n.p + 1 }));
            }
        }
    }
    let mut shortest_path: HashMap<(usize, usize), usize> = HashMap::new();
    let (mut nx, mut ny) = (ex, ey);
    loop {
        shortest_path.insert((nx, ny), dist[&(nx, ny)]);
        if let Some((x, y)) = prev[&(nx, ny)] {
            (nx, ny) = (x, y);
        } else {
            break;
        }
    }
    // println!("{:?}", shortest_path);
    let mut shortcuts = 0;
    for ((x, y), d) in shortest_path.iter() {
        if *d < 102 {
            continue;
        }
        for ((nx, ny), nd) in shortest_path.iter() {
            let m = manhattan((x, y), (nx, ny));
            if *d < 100 + m {
                continue;
            }
            if m <= 20 && *nd <= *d - 100 - m {
                shortcuts += 1;
            }
        }
    }
    format!("{}", shortcuts)
}

fn manhattan(p0: (&usize, &usize), p1: (&usize, &usize)) -> usize {
    max(p0.1, p1.1) - min(p0.1, p1.1) + max(p0.0, p1.0) - min(p0.0, p1.0)
}
