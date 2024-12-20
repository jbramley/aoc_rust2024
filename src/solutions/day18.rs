use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufReader, Lines};

// const WIDTH: usize = 7;
// const HEIGHT: usize = 7;
// const TAKE: usize = 12;
const WIDTH: usize = 71;
const HEIGHT: usize = 71;
const TAKE: usize = 1024;
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

fn adj(n: &Node) -> Vec<(usize, usize)> {
    let mut v: Vec<(usize, usize)> = Vec::new();
    if n.x > 0 {
        v.push((n.x - 1, n.y));
    }
    if n.x < WIDTH - 1 {
        v.push((n.x + 1, n.y));
    }
    if n.y > 0 {
        v.push((n.x, n.y - 1));
    }
    if n.y < HEIGHT - 1 {
        v.push((n.x, n.y + 1));
    }
    v
}
pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let mut grid = [['.'; WIDTH]; HEIGHT];

    lines
        .flatten()
        .take(TAKE)
        .map(|s| {
            let (s1, s2) = s.split_once(',').unwrap();
            (s1.parse::<usize>().unwrap(), s2.parse::<usize>().unwrap())
        })
        .for_each(|(x, y)| {
            grid[y][x] = '#';
        });

    let (sx, sy) = (0, 0);
    let (ex, ey) = (WIDTH - 1, HEIGHT - 1);

    let mut prev: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    for r in 0..HEIGHT {
        for c in 0..WIDTH {
            prev.insert((c, r), None);
            dist.insert((c, r), usize::MAX);
        }
    }
    let mut q: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    q.push(Reverse(Node { x: sx, y: sy, p: 0 }));

    while !q.is_empty() {
        let n = q.pop().unwrap().0;
        for (x, y) in adj(&n) {
            if grid[y][x] == '#' {
                continue;
            }
            if n.p + 1 < dist[&(x, y)] {
                dist.entry((x, y)).and_modify(|d| *d = n.p + 1);
                prev.entry((x, y)).and_modify(|p| *p = Some((n.x, n.y)));
                q.push(Reverse(Node { x, y, p: n.p + 1 }));
            }
        }
    }

    format!("{}", dist[&(ex, ey)])
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let mut grid = [['.'; WIDTH]; HEIGHT];

    let coords = lines
        .flatten()
        .map(|s| {
            let (s1, s2) = s.split_once(',').unwrap();
            (s1.parse::<usize>().unwrap(), s2.parse::<usize>().unwrap())
        })
        .collect::<Vec<(usize, usize)>>();

    coords.iter().take(TAKE).for_each(|(x, y)| {
        grid[*y][*x] = '#';
    });

    let mut s = "".to_string();
    for (x, y) in coords.iter().skip(TAKE) {
        grid[*y][*x] = '#';
        if dijkstra(&mut grid) == usize::MAX {
            s = format!("{},{}", x, y);
            break;
        }
    }
    s
}

fn dijkstra(grid: &mut [[char; 71]; 71]) -> usize {
    let (sx, sy) = (0, 0);
    let (ex, ey) = (WIDTH - 1, HEIGHT - 1);

    let mut prev: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    let mut dist: HashMap<(usize, usize), usize> = HashMap::new();
    for r in 0..HEIGHT {
        for c in 0..WIDTH {
            prev.insert((c, r), None);
            dist.insert((c, r), usize::MAX);
        }
    }
    let mut q: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    q.push(Reverse(Node { x: sx, y: sy, p: 0 }));

    while !q.is_empty() {
        let n = q.pop().unwrap().0;
        for (x, y) in adj(&n) {
            if grid[y][x] == '#' {
                continue;
            }
            if n.p + 1 < dist[&(x, y)] {
                dist.entry((x, y)).and_modify(|d| *d = n.p + 1);
                prev.entry((x, y)).and_modify(|p| *p = Some((n.x, n.y)));
                q.push(Reverse(Node { x, y, p: n.p + 1 }));
            }
        }
    }

    dist[&(ex, ey)]
}
