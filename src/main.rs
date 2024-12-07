mod solutions;

use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

pub fn day1part1() {
    println!("Hello, world!");
}

struct Solution(fn(Lines<BufReader<File>>) -> String);

impl Solution {
    fn run(&self, lines: Lines<BufReader<File>>) -> String {
        (self.0)(lines)
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long)]
    part: u8,

    #[arg(short, long)]
    input: Option<String>,
}

pub fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let solutions: HashMap<&str, Solution> = vec![
        ("d1p1", Solution(solutions::day01::part1)),
        ("d1p2", Solution(solutions::day01::part2)),
        ("d2p1", Solution(solutions::day02::part1)),
        ("d2p2", Solution(solutions::day02::part2)),
        ("d3p1", Solution(solutions::day03::part1)),
        ("d3p2", Solution(solutions::day03::part2)),
        ("d4p1", Solution(solutions::day04::part1)),
        ("d4p2", Solution(solutions::day04::part2)),
        ("d5p1", Solution(solutions::day05::part1)),
        ("d5p2", Solution(solutions::day05::part2)),
        ("d6p1", Solution(solutions::day06::part1)),
        ("d6p2", Solution(solutions::day06::part2)),
        ("d7p1", Solution(solutions::day07::part1)),
        ("d7p2", Solution(solutions::day07::part2)),
        ("d8p1", Solution(solutions::day08::part1)),
        ("d8p2", Solution(solutions::day08::part2)),
        ("d9p1", Solution(solutions::day09::part1)),
        ("d9p2", Solution(solutions::day09::part2)),
        ("d10p1", Solution(solutions::day10::part1)),
        ("d10p2", Solution(solutions::day10::part2)),
        ("d11p1", Solution(solutions::day11::part1)),
        ("d11p2", Solution(solutions::day11::part2)),
        ("d12p1", Solution(solutions::day12::part1)),
        ("d12p2", Solution(solutions::day12::part2)),
        ("d13p1", Solution(solutions::day13::part1)),
        ("d13p2", Solution(solutions::day13::part2)),
        ("d14p1", Solution(solutions::day14::part1)),
        ("d14p2", Solution(solutions::day14::part2)),
        ("d15p1", Solution(solutions::day15::part1)),
        ("d15p2", Solution(solutions::day15::part2)),
        ("d16p1", Solution(solutions::day16::part1)),
        ("d16p2", Solution(solutions::day16::part2)),
        ("d17p1", Solution(solutions::day17::part1)),
        ("d17p2", Solution(solutions::day17::part2)),
        ("d18p1", Solution(solutions::day18::part1)),
        ("d18p2", Solution(solutions::day18::part2)),
        ("d19p1", Solution(solutions::day19::part1)),
        ("d19p2", Solution(solutions::day19::part2)),
        ("d20p1", Solution(solutions::day20::part1)),
        ("d20p2", Solution(solutions::day20::part2)),
        ("d21p1", Solution(solutions::day21::part1)),
        ("d21p2", Solution(solutions::day21::part2)),
        ("d22p1", Solution(solutions::day22::part1)),
        ("d22p2", Solution(solutions::day22::part2)),
        ("d23p1", Solution(solutions::day23::part1)),
        ("d23p2", Solution(solutions::day23::part2)),
        ("d24p1", Solution(solutions::day24::part1)),
        ("d24p2", Solution(solutions::day24::part2)),
        ("d25p1", Solution(solutions::day25::part1)),
        ("d25p2", Solution(solutions::day25::part2)),
    ]
    .into_iter()
    .collect();

    let cli = Cli::parse();
    let key = format!("d{}p{}", cli.day, cli.part);
    let input_file = cli
        .input
        .unwrap_or_else(|| format!("inputs/day{}.txt", cli.day));

    if let Some(runner) = solutions.get(key.as_str()) {
        if let Ok(input) = read_lines(input_file) {
            let result = runner.run(input);
            println!("{}", result);
        } else {
            println!("Failed to read input file");
        }
    } else {
        println!("Solution not found");
    }
}
