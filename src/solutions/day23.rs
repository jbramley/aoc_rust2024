use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Lines};

pub fn part1(lines: Lines<BufReader<File>>) -> String {
    let mut network: HashMap<String, Vec<String>> = HashMap::new();
    lines.flatten()
        .map(|s| {
            let (s1, s2) = s.split_once("-").unwrap();
            (s1.to_string(), s2.to_string())
        })
        .for_each(|(s1, s2)| {
            network.entry(s1.clone()).and_modify(|v| v.push(s2.clone())).or_insert(vec![s2.clone()]);
            network.entry(s2.clone()).and_modify(|v| v.push(s1.clone())).or_insert(vec![s1.clone()]);
        });
    let mut targets: HashSet<(&String, &String, &String)> = HashSet::new();
    for (k, v) in network.iter() {
        for k2 in v {
            for k3 in network.get(k2).unwrap() {
                for k4 in network.get(k3).unwrap() {
                    if k4 == k && (k.starts_with('t') || k2.starts_with('t') || k3.starts_with('t')) {
                        let mut v = vec![k, k2, k3];
                        v.sort();
                        targets.insert((v[0], v[1], v[2]));
                    }
                }
            }
        }
    }
    println!("{:?}", targets);
    format!("{}", targets.len())
}

fn bron_kerbosch<'a>(r: HashSet<&'a str>, p: HashSet<&'a str>, x: HashSet<&'a str>, edges: &HashMap<&str, Vec<&'a str>>) -> Vec<HashSet<&'a str>> {
    if p.is_empty() && x.is_empty() {
        return vec![r];
    }
    let mut maximal: Vec<HashSet<&str>> = Vec::new();
    let mut my_p = p.clone();
    let p_v = Vec::from_iter(p.iter().cloned());
    let mut my_x = x.clone();
    for v in p_v {
        let mut vs: HashSet<&str> = r.clone();
        vs.insert(v);
        let n_v: HashSet<&str> = HashSet::from_iter(edges.get(v).unwrap().iter().cloned());
        maximal.extend(bron_kerbosch(vs, my_p.intersection(&n_v).cloned().collect(), my_x.intersection(&n_v).cloned().collect(), edges));
        my_p.remove(v);
        my_x.insert(v);
    }
    maximal
}
pub fn part2(lines: Lines<BufReader<File>>) -> String {
    let mut network: HashMap<&str, Vec<&str>> = HashMap::new();
    let lines = lines.flatten().collect::<Vec<String>>();
    for s in lines.iter() {
        let (s1, s2) = s.split_once("-").unwrap();
        network.entry(s1).and_modify(|v| v.push(s2)).or_insert(vec![s2]);
        network.entry(s2).and_modify(|v| v.push(s1)).or_insert(vec![s1]);
    }

    let nodes = network.iter().map(|(k,_)| *k).collect::<HashSet<_>>();
    let maximals = bron_kerbosch(HashSet::new(), nodes, HashSet::new(), &network);
    let mut supermax: HashSet<&str> = HashSet::new();
    for m in maximals {
        if m.len() > supermax.len() {
            supermax = m;
        }
    }
    let mut supermax = Vec::from_iter(supermax.iter().cloned());
    supermax.sort();
    println!("{}", supermax.join(","));
    "Not implemented yet".to_string()
}
