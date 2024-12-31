use regex::Regex;
use std::{str::FromStr, sync::LazyLock};

use crate::util::Ratio;

const INPUT: &'static str = include_str!("./input.in");

static BUTTON_LINE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r".*X\+(\d+).*Y\+(\d+)").unwrap());
static PRIZE_LINE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r".*X=(\d+).*Y=(\d+)").unwrap());

const A_COST: isize = 3;
const B_COST: isize = 1;

pub fn run() {
    let claw_machines = parse_input(INPUT);

    let cost = claw_machines
        .iter()
        .filter_map(|cm| find_prize(cm))
        .sum::<isize>();

    println!("Part 1: {}", cost);

    let cost = claw_machines
        .iter()
        .filter_map(|cm| {
            find_prize(&ClawMachine {
                prize: (cm.prize.0 + 10000000000000, cm.prize.1 + 10000000000000),
                ..*cm
            })
        })
        .sum::<isize>();

    println!("Part 2: {}", cost);
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    input
        .split("\n\n")
        .filter(|it| it.trim().len() > 0)
        .map(|cm| cm.parse().unwrap())
        .collect()
}

fn find_prize(
    &ClawMachine {
        a: (ax, ay),
        b: (bx, by),
        prize: (px, py),
    }: &ClawMachine,
) -> Option<isize> {
    let det = ax * by - bx * ay;

    let a = Ratio::new(px * by - bx * py, det);
    let b = Ratio::new(ax * py - px * ay, det);

    if a.is_integer() && b.is_integer() {
        Some(A_COST * a.as_integer() + B_COST * b.as_integer())
    } else {
        None
    }
}

#[derive(Debug)]
struct ClawMachine {
    prize: (isize, isize),
    a: (isize, isize),
    b: (isize, isize),
}

impl FromStr for ClawMachine {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let a = parse_button_line(lines.next().unwrap());
        let b = parse_button_line(lines.next().unwrap());
        let prize = parse_prize_line(lines.next().unwrap());
        Ok(ClawMachine { a, b, prize })
    }
}

fn parse_button_line(line: &str) -> (isize, isize) {
    let caps = BUTTON_LINE_RE.captures(line).unwrap();

    let (_, [x, y]) = caps.extract();

    (x.parse().unwrap(), y.parse().unwrap())
}

fn parse_prize_line(line: &str) -> (isize, isize) {
    let caps = PRIZE_LINE_RE.captures(line).unwrap();

    let (_, [x, y]) = caps.extract();

    (x.parse().unwrap(), y.parse().unwrap())
}
