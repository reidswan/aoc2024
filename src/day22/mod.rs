use std::collections::HashMap;

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let input = parse_input(&INPUT);
    println!(
        "Part 1: {}",
        input.iter().map(|n| evolve_2000(*n)).sum::<usize>()
    );
    let mut all_sequences = vec![];
    for n in input {
        all_sequences.push(sequences(&price_changes(n)));
    }

    println!("Part 2: {}", find_best_sequence(&all_sequences));
}

fn find_best_sequence(all_price_changes: &[Vec<((isize, isize, isize, isize), isize)>]) -> isize {
    let mut hm: HashMap<_, isize> = HashMap::new();

    for price_changes in all_price_changes {
        let mut hm2: HashMap<_, isize> = HashMap::new();

        for (seq, price) in price_changes {
            if !hm2.contains_key(seq) {
                hm2.insert(seq, *price);
            }
        }

        for (k, v) in hm2 {
            *hm.entry(*k).or_default() += v
        }
    }

    *hm.iter().max_by_key(|(_, v)| *v).unwrap().1
}

fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn evolve_number(number: usize) -> usize {
    let step1 = prune(mix(number, number * 64));
    let step2 = prune(mix(step1, step1 / 32));
    prune(mix(step2, step2 * 2048))
}

fn evolve_2000(number: usize) -> usize {
    (0..2000).fold(number, |a, _| evolve_number(a))
}

#[inline(always)]
fn mix(a: usize, b: usize) -> usize {
    a ^ b
}

#[inline(always)]
fn prune(a: usize) -> usize {
    a % 16777216
}

fn price_changes(number: usize) -> Vec<(isize, isize)> {
    let mut price_changes = Vec::with_capacity(1999);
    let mut prev_price = (number % 10) as isize;
    let mut number = number;
    for _ in 0..2000 {
        number = evolve_number(number);
        let price = (number % 10) as isize;
        price_changes.push((price - prev_price, price));
        prev_price = price
    }

    price_changes
}

fn sequences(seq: &[(isize, isize)]) -> Vec<((isize, isize, isize, isize), isize)> {
    seq.iter()
        .zip(seq.iter().skip(1))
        .zip(seq.iter().skip(2))
        .zip(seq.iter().skip(3))
        .map(|(((a, b), c), d)| ((a.0, b.0, c.0, d.0), d.1))
        .collect()
}
