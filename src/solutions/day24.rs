use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Lines};
use itertools::Itertools;
use topological_sort::TopologicalSort;


#[derive(Debug)]
enum Operation {
    AND,
    OR,
    XOR
}

impl Operation {
    fn exec(&self, a: bool, b: bool) -> bool {
        match self {
            Operation::AND => a & b,
            Operation::OR => a | b,
            Operation::XOR => a ^ b,
        }
    }
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Operation::AND,
            "OR" => Operation::OR,
            "XOR" => Operation::XOR,
            _ => panic!()
        }
    }
}

#[derive(Debug)]
struct Gate {
    inputs: [String; 2],
    op: Operation,
    output: Vec<String>,
}


pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let mut wires: HashMap<String, bool> = HashMap::new();
    let mut gates: HashMap<String, Gate> = HashMap::new();
    let mut rmap_gates: HashMap<String, String> = HashMap::new();

    for line in lines.flatten() {
        if line.contains(":") {
            let w = line[..3].to_string();
            let b = line.ends_with("1");
            // println!("Init: {} -> {}", w.clone(), b);
            wires.insert(w, b);
        } else if line.contains("->") {
            let v = line.split_ascii_whitespace().collect::<Vec<_>>();
            let n = format!("{} {} {}", v[0], v[1], v[2]);
            gates.entry(n.clone())
                .and_modify(|g| g.output.push(v[4].to_string()))
                .or_insert(Gate {
                    inputs: [v[0].to_string(), v[2].to_string()],
                    op: Operation::from(v[1]),
                    output: vec![v[4].to_string()]
                });
            wires.insert(v[4].to_string(), false);
            rmap_gates.insert(v[4].to_string(), n.clone());
        }
    }

    let mut ts = TopologicalSort::<&str>::new();
    for (n, g) in gates.iter() {
        if let Some(dep) = rmap_gates.get(&g.inputs[0]) {
            ts.add_dependency(dep.as_str(), n.as_str());
        }
        if let Some(dep) = rmap_gates.get(&g.inputs[1]) {
            ts.add_dependency(dep.as_str(), n.as_str());
        }
    }

    while !ts.is_empty() {
        let n = ts.pop().unwrap();
        let g = gates.get(n).unwrap();
        let a = wires.get(&g.inputs[0]).unwrap().clone();
        let b = wires.get(&g.inputs[1]).unwrap().clone();
        for out in g.output.iter() {
            wires.entry(out.clone()).and_modify(|d| *d = g.op.exec(a,b));
        }
        // println!("{} -> {:?}: {} {:?} {} -> {}", n, g.output, a, g.op, b, g.op.exec(a,b));
    }

    let mut ans = 0u64;
    for (i, z) in wires.keys()
        .filter(|k| k.starts_with("z"))
        .sorted()
        .enumerate() {
        if wires[z] {
            ans += 2u64<<(i-1);
        }
        // print!("{}", if wires[z] { "1" } else { "0" });
        
    }
    format!("{}", ans)
}

pub fn part2(_lines: Lines<BufReader<File>>) -> String {
    "Not implemented yet".to_string()
}
