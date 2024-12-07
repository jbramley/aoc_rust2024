use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Lines};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'V' => Direction::Down,
            _ => panic!("Invalid direction"),
        }
    }

    fn move_pos(&self, pos: (isize, isize)) -> (isize, isize) {
        match self {
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
        }
    }

    fn rotate_right(&self) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }
}

fn get_route(map: &Vec<Vec<char>>, pos: &(isize, isize), dir: &Direction) -> Vec<(usize, usize)> {
    let x_bounds = (0isize, map[0].len() as isize - 1);
    let y_bounds = (0isize, map.len() as isize - 1);
    let mut pos = *pos;
    let mut dir = *dir;
    let mut route: Vec<(usize, usize)> = Vec::new();
    loop {
        route.push((pos.0 as usize, pos.1 as usize));
        let next_pos = dir.move_pos(pos);
        if next_pos.0 < x_bounds.0
            || next_pos.0 > x_bounds.1
            || next_pos.1 < y_bounds.0
            || next_pos.1 > y_bounds.1
        {
            break;
        }
        if map[next_pos.1 as usize][next_pos.0 as usize] == '#' {
            dir = dir.rotate_right();
        } else {
            pos = next_pos;
        }
    }
    route
}

fn detect_loop(map: &Vec<Vec<char>>, pos: &(isize, isize), dir: &Direction) -> bool {
    let x_bounds = (0isize, map[0].len() as isize - 1);
    let y_bounds = (0isize, map.len() as isize - 1);
    let mut pos = *pos;
    let mut dir = *dir;
    let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();
    loop {
        visited.insert((pos.0 as usize, pos.1 as usize, dir));
        let next_pos = dir.move_pos(pos);
        if next_pos.0 < x_bounds.0
            || next_pos.0 > x_bounds.1
            || next_pos.1 < y_bounds.0
            || next_pos.1 > y_bounds.1
        {
            return false;
        }
        if map[next_pos.1 as usize][next_pos.0 as usize] == '#' {
            dir = dir.rotate_right();
        } else {
            if visited.contains(&(next_pos.0 as usize, next_pos.1 as usize, dir)) {
                return true;
            }
            pos = next_pos;
        }
    }
}
pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let mut map = lines
        .flatten()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut pos = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, c)| {
                if *c != '.' && *c != '#' {
                    Some((x as isize, y as isize))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let x_bounds = (0isize, map[0].len() as isize - 1);
    let y_bounds = (0isize, map.len() as isize - 1);
    let mut dir = Direction::from_char(map[pos.1 as usize][pos.0 as usize]);
    loop {
        map[pos.1 as usize][pos.0 as usize] = 'X';
        let next_pos = dir.move_pos(pos);
        if next_pos.0 < x_bounds.0
            || next_pos.0 > x_bounds.1
            || next_pos.1 < y_bounds.0
            || next_pos.1 > y_bounds.1
        {
            break;
        }
        if map[next_pos.1 as usize][next_pos.0 as usize] == '#' {
            dir = dir.rotate_right();
        } else {
            pos = next_pos;
        }
    }
    let visited = map
        .iter()
        .map(|v| v.iter().filter(|c| **c == 'X').count())
        .sum::<usize>();
    format!("{:?}", visited)
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let mut map = lines
        .flatten()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let pos = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, c)| {
                if *c != '.' && *c != '#' {
                    Some((x as isize, y as isize))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let dir = Direction::from_char(map[pos.1 as usize][pos.0 as usize]);
    let route = get_route(&map, &pos, &dir);

    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
    for obstacle_pos in route {
        map[obstacle_pos.1][obstacle_pos.0] = '#';
        if detect_loop(&map, &pos, &dir) {
            obstacles.insert(obstacle_pos);
        }
        map[obstacle_pos.1][obstacle_pos.0] = '.';
    }
    format!("{}", obstacles.len())
}
