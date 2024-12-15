use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufReader, Lines};

fn find_first_gap(
    map: &Vec<Vec<char>>,
    robot_pos: (i32, i32),
    dx: i32,
    dy: i32,
) -> Option<(i32, i32)> {
    let mut x = robot_pos.0 + dx;
    let mut y = robot_pos.1 + dy;

    while x >= 0 && x < map[0].len() as i32 && y >= 0 && y < map.len() as i32 {
        if map[y as usize][x as usize] == '.' {
            return Some((x, y));
        } else if map[y as usize][x as usize] == '#' {
            return None;
        } else {
            x += dx;
            y += dy;
        }
    }
    None
}

fn move_robot(
    map: &mut Vec<Vec<char>>,
    first_gap: (i32, i32),
    robot_pos: (i32, i32),
    dx: i32,
    dy: i32,
) {
    let (mut x, mut y) = first_gap;
    while x != robot_pos.0 || y != robot_pos.1 {
        map[y as usize][x as usize] = map[(y - dy) as usize][(x - dx) as usize];
        y -= dy;
        x -= dx;
    }
    map[robot_pos.1 as usize][robot_pos.0 as usize] = '.';
}

pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let mut on_map = true;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut steps: String = String::new();
    for line in lines.flatten() {
        if on_map {
            if line.is_empty() {
                on_map = false;
            } else {
                map.push(line.chars().collect::<Vec<_>>());
            }
        } else {
            steps.push_str(line.as_str());
        }
    }
    let robot_pos = map
        .iter()
        .enumerate()
        .filter_map(|(r, line)| {
            if let Some((c, _)) = line.iter().find_position(|ch| **ch == '@') {
                Some((r, c))
            } else {
                None
            }
        })
        .next();
    let mut robot_pos = match robot_pos {
        Some(p) => (p.1 as i32, p.0 as i32),
        None => {
            panic!("Could not find robot");
        }
    };
    for step in steps.chars() {
        let (dx, dy) = match step {
            '<' => (-1, 0),
            '>' => (1, 0),
            'v' => (0, 1),
            '^' => (0, -1),
            _ => (0, 0),
        };
        if let Some(first_gap) = find_first_gap(&map, robot_pos, dx, dy) {
            move_robot(&mut map, first_gap, robot_pos, dx, dy);
            robot_pos.0 += dx;
            robot_pos.1 += dy;
        }
    }
    let gps = map
        .iter()
        .enumerate()
        .map(|(r, line)| {
            line.iter()
                .enumerate()
                .map(|(c, ch)| match *ch {
                    'O' => 100 * r + c,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    format!("{}", gps)
}

fn build_move_tree(
    map: &Vec<Vec<char>>,
    robot_pos: (i32, i32),
    dy: i32,
) -> Option<HashMap<(usize, usize), Vec<(usize, usize)>>> {
    let mut tree: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    q.push_back((robot_pos.0 as usize, robot_pos.1 as usize));
    while !q.is_empty() {
        if let Some((x, y)) = q.pop_front() {
            if tree.contains_key(&(x, y)) {
                continue;
            }
            let pc = (x, (y as i32 + dy) as usize);
            let pl = (x - 1, (y as i32 + dy) as usize);
            let pr = (x + 1, (y as i32 + dy) as usize);
            match map[y][x] {
                '#' => return None,
                '.' => continue,
                '@' | '[' | ']' => {
                    let mut v = vec![];
                    if map[pc.1][pc.0] == ']' && !q.contains(&pl) {
                        v.push(pl);
                        q.push_back(pl);
                    }
                    if !q.contains(&pc) {
                        v.push(pc);
                        q.push_back(pc);
                    }
                    if map[pc.1][pc.0] == '[' && !q.contains(&pr) {
                        v.push(pr);
                        q.push_back(pr)
                    }

                    tree.insert((x, y), v);
                }
                _ => return None,
            }
        }
    }
    Some(tree)
}

fn move_robot_tree(
    map: &mut Vec<Vec<char>>,
    tree: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    pos: (usize, usize),
    dy: i32,
) {
    if tree.contains_key(&pos) {
        tree[&pos]
            .iter()
            .for_each(|xy| move_robot_tree(map, tree, *xy, dy));
    }
    if tree.contains_key(&(pos.0, (pos.1 as i32 - dy) as usize)) {
        map[pos.1][pos.0] = map[(pos.1 as i32 - dy) as usize][pos.0];
    } else {
        map[pos.1][pos.0] = '.';
    }
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let mut on_map = true;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut steps: String = String::new();
    for line in lines.flatten() {
        if on_map {
            if line.is_empty() {
                on_map = false;
            } else {
                let mut wc: Vec<char> = Vec::new();
                line.chars().for_each(|c| {
                    match c {
                        '#' | '.' => {
                            wc.push(c);
                            wc.push(c);
                        }
                        '@' => {
                            wc.push(c);
                            wc.push('.');
                        }
                        'O' => {
                            wc.push('[');
                            wc.push(']');
                        }
                        _ => {}
                    };
                });
                map.push(wc);
            }
        } else {
            steps.push_str(line.as_str());
        }
    }
    let robot_pos = map
        .iter()
        .enumerate()
        .filter_map(|(r, line)| {
            if let Some((c, _)) = line.iter().find_position(|ch| **ch == '@') {
                Some((r, c))
            } else {
                None
            }
        })
        .next();
    let mut robot_pos = match robot_pos {
        Some(p) => (p.1 as i32, p.0 as i32),
        None => {
            panic!("Could not find robot");
        }
    };

    for step in steps.chars() {
        let (dx, dy) = match step {
            '<' => (-1, 0),
            '>' => (1, 0),
            'v' => (0, 1),
            '^' => (0, -1),
            _ => (0, 0),
        };
        if dx != 0 {
            if let Some(first_gap) = find_first_gap(&map, robot_pos, dx, dy) {
                move_robot(&mut map, first_gap, robot_pos, dx, dy);
                robot_pos.0 += dx;
            }
        } else {
            if let Some(move_tree) = build_move_tree(&map, robot_pos, dy) {
                move_robot_tree(
                    &mut map,
                    &move_tree,
                    (robot_pos.0 as usize, robot_pos.1 as usize),
                    dy,
                );
                robot_pos.1 += dy;
            }
        }
    }
    let gps = map
        .iter()
        .enumerate()
        .map(|(r, line)| {
            line.iter()
                .enumerate()
                .map(|(c, ch)| match *ch {
                    '[' => 100 * r + c,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    format!("{}", gps)
}
