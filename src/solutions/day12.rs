use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let plots = lines
        .flatten()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let max_y = plots.len() - 1;
    let max_x = plots[0].len() - 1;

    let mut total_fencing = 0;
    for (y, line) in plots.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if visited.contains(&(x, y)) {
                continue;
            }

            let mut plotq: VecDeque<(usize, usize)> = VecDeque::new();
            let mut thisplot: HashSet<(usize, usize)> = HashSet::new();
            let mut plotarea = 0;
            let mut plotpermiter = 0;
            plotq.push_back((x, y));

            while !plotq.is_empty() {
                let (px, py) = plotq.pop_front().unwrap();
                if thisplot.contains(&(px, py)) {
                    continue;
                }
                visited.insert((px, py));
                thisplot.insert((px, py));
                let mut borders = 4;
                if py > 0 && plots[py - 1][px] == *c {
                    borders -= 1;
                    if !thisplot.contains(&(px, py - 1)) {
                        plotq.push_back((px, py - 1));
                    }
                }
                if py < max_y && plots[py + 1][px] == *c {
                    borders -= 1;
                    if !thisplot.contains(&(px, py + 1)) {
                        plotq.push_back((px, py + 1));
                    }
                }
                if px > 0 && plots[py][px - 1] == *c {
                    borders -= 1;
                    if !thisplot.contains(&(px - 1, py)) {
                        plotq.push_back((px - 1, py));
                    }
                }
                if px < max_x && plots[py][px + 1] == *c {
                    borders -= 1;
                    if !thisplot.contains(&(px + 1, py)) {
                        plotq.push_back((px + 1, py));
                    }
                }
                plotarea += 1;
                plotpermiter += borders;
            }
            total_fencing += plotarea * plotpermiter;
        }
    }
    format!("{}", total_fencing)
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let plots = lines
        .flatten()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let max_y = plots.len() - 1;
    let max_x = plots[0].len() - 1;

    let mut total_fencing = 0;
    for (y, line) in plots.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if visited.contains(&(x, y)) {
                continue;
            }

            let mut plotq: VecDeque<(usize, usize)> = VecDeque::new();
            let mut thisplot: HashSet<(usize, usize)> = HashSet::new();
            let mut plotarea = 0;
            let mut plotpermiter = 0;
            plotq.push_back((x, y));

            while !plotq.is_empty() {
                let (px, py) = plotq.pop_front().unwrap();
                if thisplot.contains(&(px, py)) {
                    continue;
                }
                visited.insert((px, py));
                thisplot.insert((px, py));
                let mut borders = 4;
                if py > 0 && plots[py - 1][px] == *c {
                    borders -= 1;
                    if px == 0 || (px > 0 && plots[py - 1][px - 1] != *c && plots[py][px - 1] != *c)
                    {
                        borders -= 1;
                    }
                    if px == max_x
                        || (px < max_x && plots[py - 1][px + 1] != *c && plots[py][px + 1] != *c)
                    {
                        borders -= 1;
                    }
                    if !thisplot.contains(&(px, py - 1)) {
                        plotq.push_back((px, py - 1));
                    }
                }
                if py < max_y && plots[py + 1][px] == *c {
                    borders -= 1;
                    if !thisplot.contains(&(px, py + 1)) {
                        plotq.push_back((px, py + 1));
                    }
                }
                if px > 0 && plots[py][px - 1] == *c {
                    borders -= 1;
                    if py == 0 || (py > 0 && plots[py - 1][px - 1] != *c && plots[py - 1][px] != *c)
                    {
                        borders -= 1;
                    }
                    if py == max_y
                        || (py < max_y && plots[py + 1][px - 1] != *c && plots[py + 1][px] != *c)
                    {
                        borders -= 1;
                    }
                    if !thisplot.contains(&(px - 1, py)) {
                        plotq.push_back((px - 1, py));
                    }
                }
                if px < max_x && plots[py][px + 1] == *c {
                    borders -= 1;
                    if !thisplot.contains(&(px + 1, py)) {
                        plotq.push_back((px + 1, py));
                    }
                }
                plotarea += 1;
                plotpermiter += borders;
            }
            total_fencing += plotarea * plotpermiter;
        }
    }
    format!("{}", total_fencing)
}
