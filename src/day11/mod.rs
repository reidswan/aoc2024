use crate::util::Counter;

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let mut stones_count = parse_input(INPUT);

    for _ in 0..25 {
        stones_count = blink(stones_count);
    }

    println!("Part 1: {}", stones_count.total(),);

    for _ in 0..50 {
        stones_count = blink(stones_count);
    }

    println!("Part 2: {}", stones_count.total(),);
}

pub fn parse_input(input: &str) -> Counter<usize> {
    input.split(" ").map(|it| it.parse().unwrap()).collect()
}

fn number_len(stone: usize) -> u32 {
    (stone as f64).log10().floor() as u32 + 1
}

fn blink_one(stone: usize) -> (usize, Option<usize>) {
    if stone == 0 {
        return (1, None);
    }

    let len = number_len(stone);
    if len % 2 == 0 {
        return (
            stone / 10usize.pow(len / 2),
            Some(stone % 10usize.pow(len / 2)),
        );
    }

    (stone * 2024, None)
}

fn blink(counts: Counter<usize>) -> Counter<usize> {
    let mut new_counts = Counter::new();

    for (stone, count) in counts.iter() {
        let (a, maybe_b) = blink_one(*stone);
        new_counts.count_n(a, count);
        if let Some(b) = maybe_b {
            new_counts.count_n(b, count);
        };
    }

    new_counts
}
