use std::isize;

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let reports = parse_input(INPUT);

    println!(
        "Part 1: {}",
        reports
            .iter()
            .filter(|r| check_safety(r.iter().map(|e| *e)))
            .count()
    );

    println!(
        "Part 2: {}",
        reports.iter().filter(|r| check_safety_part2(r)).count()
    );
}

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|e| {
            e.split_ascii_whitespace()
                .map(|e| e.parse().unwrap())
                .collect()
        })
        .collect()
}

fn check_safety<T: Iterator<Item = isize> + Clone>(report: T) -> bool {
    let diffs = compute_diffs(report);
    if diffs.len() <= 1 {
        return true;
    }

    let main_signum = diffs.iter().map(|e| e.signum()).sum::<isize>().signum();

    diffs
        .iter()
        .all(|diff| diff.abs() >= 1 && diff.abs() <= 3 && diff.signum() == main_signum)
}

fn compute_diffs<T: Iterator<Item = isize> + Clone>(report: T) -> Vec<isize> {
    report
        .clone()
        .zip(report.skip(1))
        .map(|(a, b)| b - a)
        .collect()
}

fn check_safety_part2(report: &[isize]) -> bool {
    for i in 0..report.len() {
        let iter = report.iter().take(i).chain(report.iter().skip(i + 1));

        if check_safety(iter.map(|e| *e)) {
            return true;
        }
    }

    return false;
}
