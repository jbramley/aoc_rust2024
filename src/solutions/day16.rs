use itertools::Itertools;
use num::abs;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Lines};
use std::slice::Iter;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}
impl Display for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Dir::E => write!(f, "E"),
            Dir::N => write!(f, "N"),
            Dir::S => write!(f, "S"),
            Dir::W => write!(f, "W"),
        }
    }
}

impl Dir {
    fn iterator() -> Iter<'static, Dir> {
        static DIRS: [Dir; 4] = [Dir::N, Dir::E, Dir::S, Dir::W];
        DIRS.iter()
    }

    fn dv(&self) -> (i32, i32) {
        match self {
            Dir::N => (0, -1),
            Dir::E => (1, 0),
            Dir::S => (0, 1),
            Dir::W => (-1, 0),
        }
    }

    fn child_dirs(&self) -> [Dir; 3] {
        match self {
            Dir::N => [Dir::W, Dir::N, Dir::E],
            Dir::E => [Dir::N, Dir::E, Dir::S],
            Dir::S => [Dir::E, Dir::S, Dir::W],
            Dir::W => [Dir::S, Dir::W, Dir::N],
        }
    }
}

#[derive(Copy, Clone)]
struct Node {
    x: usize,
    y: usize,
    f: usize,
    g: usize,
    h: usize,
    dir: Dir,
}

impl Node {
    fn new(x: usize, y: usize, g: usize, ex: usize, ey: usize, dir: Dir) -> Node {
        let h = (ex - x) + (y - ey);
        Node {
            x,
            y,
            f: g + h,
            g,
            h,
            dir,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f.cmp(&other.f)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl Eq for Node {}

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
pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let maze = lines
        .flatten()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>();
    let start = find_in_grid(&maze, 'S').unwrap();
    let end = find_in_grid(&maze, 'E').unwrap();

    let mut visited: HashMap<(usize, usize), Node> = HashMap::new();
    let mut nodes: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    nodes.push(Reverse(Node::new(
        start.0,
        start.1,
        0,
        end.0,
        end.1,
        Dir::E,
    )));

    let mut best_score = 1usize;
    while !nodes.is_empty() {
        let c_n = nodes.pop().unwrap().0;
        visited.insert((c_n.x, c_n.y), c_n);
        if c_n.x == end.0 && c_n.y == end.1 {
            best_score = c_n.g;
            break;
        }
        for d in c_n.dir.child_dirs() {
            let dv = d.dv();
            let ch = (
                (c_n.x as i32 + dv.0) as usize,
                (c_n.y as i32 + dv.1) as usize,
            );
            if visited.contains_key(&ch) {
                continue;
            }
            if maze[ch.1][ch.0] == '#' {
                continue;
            }
            let mc = if d == c_n.dir { 1 } else { 1001 };
            let chn = Node::new(ch.0, ch.1, c_n.g + mc, end.0, end.1, d);
            nodes.push(Reverse(chn))
        }
    }
    format!("{}", best_score)
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let maze = lines
        .flatten()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>();
    let start = find_in_grid(&maze, 'S').unwrap();
    let end = find_in_grid(&maze, 'E').unwrap();

    let mut q: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    q.push(Reverse(Node::new(
        start.0,
        start.1,
        0,
        end.0,
        end.1,
        Dir::E,
    )));

    let mut prev: HashMap<(usize, usize, Dir), Vec<(usize, usize, Dir)>> = HashMap::new();
    let mut dist: HashMap<(usize, usize, Dir), Option<usize>> = HashMap::new();
    for r in 0..maze.len() {
        for c in 0..maze[0].len() {
            for d in Dir::iterator() {
                prev.insert((c, r, *d), vec![]);
                dist.insert((c, r, *d), None);
            }
        }
    }

    while !q.is_empty() {
        let n = q.pop().unwrap().0;

        for d in n.dir.child_dirs() {
            let dv = d.dv();
            let ch = if d == n.dir {
                (
                    (n.x as i32 + dv.0) as usize,
                    (n.y as i32 + dv.1) as usize,
                    d,
                )
            } else {
                (n.x, n.y, d)
            };
            if maze[ch.1][ch.0] == '#' {
                continue;
            }
            let g = n.g + if d == n.dir { 1 } else { 1000 };
            match dist.get(&ch).unwrap() {
                None => {
                    dist.entry(ch).and_modify(|x| {
                        *x = Some(g);
                    });
                    prev.entry(ch).and_modify(|v| v.push((n.x, n.y, n.dir)));
                    q.push(Reverse(Node::new(ch.0, ch.1, g, ch.0, ch.1, d)));
                }
                Some(dg) => {
                    if g < *dg {
                        dist.entry(ch).and_modify(|x| {
                            *x = Some(g);
                        });
                        prev.entry(ch)
                            .and_modify(|v| v.clear())
                            .and_modify(|v| v.push((n.x, n.y, n.dir)));
                        q.push(Reverse(Node::new(ch.0, ch.1, g, ch.0, ch.1, d)));
                    } else if g == *dg {
                        prev.entry(ch).and_modify(|v| v.push((n.x, n.y, n.dir)));
                    }
                }
            }
        }
    }

    let mut shortest_paths_nodes: HashSet<(usize, usize)> = HashSet::new();
    let mut visited: HashSet<(usize, usize, Dir)> = HashSet::new();

    let shortest_path = Dir::iterator()
        .filter_map(|d| *dist.get(&(end.0, end.1, *d)).unwrap())
        .min()
        .unwrap();
    let mut q: VecDeque<(usize, usize, Dir)> = VecDeque::new();
    Dir::iterator()
        .filter(|d| {
            if let Some(x) = dist.get(&(end.0, end.1, **d)).unwrap() {
                *x == shortest_path
            } else {
                false
            }
        })
        .for_each(|d| {
            q.push_back((end.0, end.1, *d));
        });

    while !q.is_empty() {
        let (x, y, d) = q.pop_front().unwrap();
        if visited.contains(&(x, y, d)) {
            continue;
        }
        shortest_paths_nodes.insert((x, y));
        visited.insert((x, y, d));
        if let Some(pn) = prev.get(&(x, y, d)) {
            pn.iter().for_each(|n| q.push_back(*n));
        }
    }
    println!("shortest path: {}", shortest_path);
    println!("nodes on a shortest path: {}", shortest_paths_nodes.len());

    "Not implemented yet".to_string()
}
