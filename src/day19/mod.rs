use std::collections::HashMap;

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let Input { towels, patterns } = parse_input(INPUT);
    let prefix_tree = construct_prefix_tree(&towels);
    let possible = patterns
        .iter()
        .filter(|pat| has_total_match(pat, &prefix_tree))
        .count();
    println!("Part 1: {}", possible);

    let mut known_counts = HashMap::new();

    let total_matches: usize = patterns
        .iter()
        .map(|pat| count_total_matches(pat, &prefix_tree, &mut known_counts))
        .sum();
    println!("Part 2: {}", total_matches);
}

struct Input<'a> {
    towels: Vec<&'a str>,
    patterns: Vec<&'a str>,
}

fn parse_input<'a>(input: &'a str) -> Input<'a> {
    let mut lines = input.lines();

    let towels = lines.next().unwrap().split(", ").collect();

    lines.next().unwrap();

    let patterns = lines.collect();

    Input { towels, patterns }
}

#[derive(Debug)]
struct PrefixTree {
    end: bool,
    children: HashMap<char, PrefixTree>,
}

impl PrefixTree {
    fn all_match_lengths(&self, pattern: &str) -> Vec<usize> {
        let mut match_lengths = vec![];
        let mut node = self;
        for (len, c) in pattern.chars().enumerate() {
            if let Some(next) = node.children.get(&c) {
                node = next;
                if node.end {
                    match_lengths.push(len + 1)
                }
            } else {
                break;
            }
        }

        match_lengths
    }
}

impl Default for PrefixTree {
    fn default() -> Self {
        Self {
            end: false,
            children: HashMap::new(),
        }
    }
}

fn construct_prefix_tree(towels: &[&str]) -> PrefixTree {
    let mut tree = PrefixTree {
        end: false,
        children: HashMap::new(),
    };

    for towel in towels {
        let mut node = &mut tree;
        for ch in towel.chars() {
            node = node.children.entry(ch).or_default();
        }
        node.end = true;
    }

    tree
}

fn has_total_match(pattern: &str, prefix_tree: &PrefixTree) -> bool {
    let mut stack = vec![0];
    while let Some(len) = stack.pop() {
        if len == pattern.len() {
            return true;
        }

        let match_lengths = prefix_tree.all_match_lengths(&pattern[len..]);
        stack.extend(match_lengths.iter().map(|l| l + len));
    }

    false
}

fn count_total_matches<'a>(
    pattern: &'a str,
    prefix_tree: &PrefixTree,
    known_counts: &mut HashMap<&'a str, usize>,
) -> usize {
    if known_counts.contains_key(pattern) {
        return *known_counts.get(pattern).unwrap();
    }

    let mut count = 0;
    for match_len in prefix_tree.all_match_lengths(&pattern) {
        if match_len == pattern.len() {
            count += 1
        } else if match_len > 0 {
            count += count_total_matches(&pattern[match_len..], prefix_tree, known_counts)
        }
    }

    known_counts.insert(pattern, count);

    count
}
