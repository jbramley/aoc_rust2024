use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Lines};

#[derive(Default, Debug)]
struct TrieNode {
    is_end: bool,
    children: HashMap<char, TrieNode>,
}

#[derive(Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut cur_node = &mut self.root;
        for c in word.chars() {
            cur_node = cur_node.children.entry(c).or_default();
        }
        cur_node.is_end = true;
    }
}

fn can_make_pattern(
    pattern: &str,
    towels: &Trie,
    offset: usize,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    if offset == pattern.len() {
        return 1;
    }
    let chars = pattern.chars().collect::<Vec<_>>();
    let mut i = offset;
    let mut backtrack: Vec<usize> = Vec::new();
    let mut n = &towels.root;
    let mut valid_arrangements = 0usize;
    while i < pattern.len() && n.children.contains_key(&chars[i]) {
        n = &n.children[&chars[i]];
        if n.is_end {
            backtrack.push(i);
        }
        i += 1;
    }
    while !backtrack.is_empty() {
        let i = backtrack.pop().unwrap();
        if let Some(x) = memo.get(&(i + 1)) {
            valid_arrangements += x;
            continue;
        }
        let x = can_make_pattern(pattern, towels, i + 1, memo);

        memo.insert(i + 1, x);
        valid_arrangements += x;
    }
    memo.insert(offset, 0);
    valid_arrangements
}
pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let mut patterns = lines.flatten().collect::<Vec<String>>();

    let mut towels = Trie::new();
    patterns
        .remove(0)
        .split_ascii_whitespace()
        .map(|s| s.trim_end_matches(','))
        .for_each(|s| towels.insert(s));

    let _ = patterns.remove(0);

    let b = patterns
        .iter()
        .filter(|p| can_make_pattern(p.as_str(), &towels, 0, &mut HashMap::new()) > 0)
        .count();

    format!("{}", b)
}

pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let mut patterns = lines.flatten().collect::<Vec<String>>();

    let mut towels = Trie::new();
    patterns
        .remove(0)
        .split_ascii_whitespace()
        .map(|s| s.trim_end_matches(','))
        .for_each(|s| towels.insert(s));

    let _ = patterns.remove(0);

    let b = patterns
        .iter()
        .map(|p| can_make_pattern(p.as_str(), &towels, 0, &mut HashMap::new()))
        .sum::<usize>();

    format!("{}", b)
}
