use std::collections::HashMap;

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let (mut l1, mut l2) = parse_input(INPUT);

    l1.sort();
    l2.sort();

    println!(
        "Part 1: {}",
        &l1.iter()
            .zip(&l2)
            .map(|(&a, &b)| (a - b).abs())
            .sum::<isize>()
    );

    let counts = count_occurrences(&l2);

    println!(
        "Part 2: {}",
        &l1.iter()
            .map(|i| *i * (*counts.get(i).unwrap_or(&0) as isize))
            .sum::<isize>()
    )
}

fn parse_input(input: &str) -> (Vec<isize>, Vec<isize>) {
    let mut l1: Vec<isize> = Vec::new();
    let mut l2: Vec<isize> = Vec::new();

    for line in input.lines() {
        let mut splits = line.split(' ');
        l1.push(splits.next().and_then(|i| i.parse().ok()).unwrap());
        l2.push(splits.last().and_then(|i| i.parse().ok()).unwrap());
    }

    (l1, l2)
}

fn count_occurrences(l: &[isize]) -> HashMap<isize, usize> {
    let mut res = HashMap::new();
    for i in l {
        res.entry(*i)
            .and_modify(|count| *count = *count + 1)
            .or_insert(1);
    }

    res
}
