use std::collections::{HashMap, HashSet};

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let connections = parse_input(INPUT);
    println!(
        "Part 1: {}",
        find_trios(&connections)
            .iter()
            .filter(|(a, b, c)| a.contains('t') || b.contains('t') || c.contains('t'))
            .count()
    );

    let mut max_clique = find_max_clique(
        &connections,
        HashSet::new(),
        connections.keys().map(|c| *c).collect(),
        HashSet::new(),
    )
    .into_iter()
    .collect::<Vec<_>>();

    max_clique.sort();
    println!("Part 2: {}", max_clique.join(","));
}

type ConnectionMap<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn parse_input<'a>(input: &'a str) -> ConnectionMap<'a> {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        connections.entry(a).or_default().insert(b);
        connections.entry(b).or_default().insert(a);
    }

    connections
}

fn find_trios<'a>(connections: &ConnectionMap<'a>) -> HashSet<(&'a str, &'a str, &'a str)> {
    let mut trios = HashSet::new();
    for (c1, conns) in connections {
        for c2 in conns {
            if c2 <= c1 {
                continue;
            }

            for c3 in connections.get(c2).unwrap() {
                if c3 <= c2 {
                    continue;
                }

                if conns.contains(c3)
                    && c1 != c3
                    && (c1.starts_with('t') || c2.starts_with('t') || c3.starts_with('t'))
                {
                    let mut trio = [c1, c2, c3];
                    trio.sort();
                    let [a, b, c] = trio;
                    trios.insert((*a, *b, *c));
                }
            }
        }
    }

    trios
}

fn find_max_clique<'a>(
    connections: &ConnectionMap<'a>,
    clique: HashSet<&'a str>,
    mut consider: HashSet<&'a str>,
    mut exclude: HashSet<&'a str>,
) -> HashSet<&'a str> {
    if consider.len() == 0 && exclude.len() == 0 {
        return clique;
    }

    let mut max = HashSet::new();

    for v in consider.clone() {
        let mut clique = clique.clone();
        clique.insert(v);

        let cc = find_max_clique(
            connections,
            clique,
            consider
                .intersection(connections.get(v).unwrap())
                .map(|c| *c)
                .collect(),
            exclude
                .intersection(connections.get(v).unwrap())
                .map(|c| *c)
                .collect(),
        );
        if cc.len() > max.len() {
            max = cc
        }

        consider.remove(v);
        exclude.insert(v);
    }

    max
}
