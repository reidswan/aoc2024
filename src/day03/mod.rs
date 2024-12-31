use regex;

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    println!("Part 1: {}", run_part_1(INPUT));

    println!("Part 2: {}", run_part_2(INPUT));
}

fn run_part_1(input: &str) -> usize {
    let mul_re = regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    mul_re
        .captures_iter(input)
        .map(|cap| {
            let (_, [left, right]) = cap.extract();

            left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap()
        })
        .sum::<usize>()
}

fn run_part_2(input: &str) -> usize {
    input
        .split("do()")
        .map(|section| section.split("don't()").next().unwrap())
        .map(|section| run_part_1(section))
        .sum::<usize>()
}
