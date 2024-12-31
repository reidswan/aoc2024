use std::collections::{HashMap, HashSet};

use crate::util::gcd;

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let input = parse_input(&INPUT);

    println!("Part 1: {}", find_basic_antinodes(&input).len());

    println!("Part 2: {}", find_harmonic_antinodes(&input).len());
}

type Coord = (usize, usize);

struct AntennaMap {
    width: usize,
    height: usize,
    antennae: HashMap<char, Vec<Coord>>,
}

fn parse_input(input: &str) -> AntennaMap {
    let mut antennae = HashMap::<char, Vec<Coord>>::new();
    let mut width = 0;
    let mut height = 0;
    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c != '.' {
                antennae.entry(c).or_default().push((x, y));
            }
            width = x
        }
        height = y
    }

    AntennaMap {
        width: width + 1,
        height: height + 1,
        antennae: antennae,
    }
}

fn find_basic_antinodes(input: &AntennaMap) -> HashSet<Coord> {
    let mut antinodes = HashSet::new();

    for ants in input.antennae.values() {
        for i in 0..ants.len() {
            for j in i + 1..ants.len() {
                let (x1, y1) = ants[i];
                let (x2, y2) = ants[j];

                let (ax1, ay1) = (2 * x1 as isize - x2 as isize, 2 * y1 as isize - y2 as isize);
                let (ax2, ay2) = (2 * x2 as isize - x1 as isize, 2 * y2 as isize - y1 as isize);

                if is_within(ax1, ay1, input.width as isize, input.height as isize) {
                    antinodes.insert((ax1 as usize, ay1 as usize));
                }
                if is_within(ax2, ay2, input.width as isize, input.height as isize) {
                    antinodes.insert((ax2 as usize, ay2 as usize));
                }
            }
        }
    }

    antinodes
}

fn find_harmonic_antinodes(input: &AntennaMap) -> HashSet<Coord> {
    let mut antinodes = HashSet::new();

    for ants in input.antennae.values() {
        for i in 0..ants.len() {
            for j in 0..ants.len() {
                if i == j {
                    continue;
                }

                let (x1, y1) = ants[i];
                let (x2, y2) = ants[j];

                let (dx, dy) = {
                    let dx = x1 as isize - x2 as isize;
                    let dy = y1 as isize - y2 as isize;

                    let factor = gcd(dx.abs(), dy.abs());

                    (dx / factor, dy / factor)
                };

                let (mut curr_x, mut curr_y) = (x1 as isize, y1 as isize);
                while is_within(curr_x, curr_y, input.width as isize, input.height as isize) {
                    antinodes.insert((curr_x as usize, curr_y as usize));
                    curr_x += dx;
                    curr_y += dy;
                }
            }
        }
    }

    antinodes
}

fn is_within(x: isize, y: isize, width: isize, height: isize) -> bool {
    0 <= x && x < width && 0 <= y && y < height
}
