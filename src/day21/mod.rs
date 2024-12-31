use std::{collections::HashMap, iter, usize};

use crate::util::{permutations, Coord, Direction};

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let sequences = parse_input(INPUT);
    let mut cache = HashMap::new();
    println!(
        "Part 1: {}",
        sequences
            .iter()
            .map(|seq| parse_code(&seq) * count_button_presses(&seq, 0, 3, &mut cache))
            .sum::<usize>()
    );

    println!(
        "Part 1: {}",
        sequences
            .iter()
            .map(|seq| parse_code(&seq) * count_button_presses(&seq, 0, 26, &mut cache))
            .sum::<usize>()
    );
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn parse_code(code: &[char]) -> usize {
    code.iter()
        .filter_map(|c| c.to_digit(10))
        .fold(0, |acc, digit| acc * 10 + digit as usize)
}

fn count_button_presses(
    path: &[char],
    depth: usize,
    limit: usize,
    seen: &mut HashMap<(String, usize, usize), usize>,
) -> usize {
    let key = {
        let mut s = String::with_capacity(path.len());
        s.extend(path);
        s
    };

    if seen.contains_key(&(key.clone(), depth, limit)) {
        return *seen.get(&(key, depth, limit)).unwrap();
    }

    if depth == limit {
        return path.len();
    }

    let ch_map = if depth == 0 {
        keypad_button_to_coord
    } else {
        directionpad_to_coord
    };

    let deadzone = ch_map('#');
    let start_pos = ch_map('A');

    let result = path
        .iter()
        .fold((start_pos, 0), |(from, sum), curr| {
            let loc = ch_map(*curr);

            let rights = loc.0 - from.0;
            let downs = loc.1 - from.1;
            let mut moves = iter::repeat_n('>', rights.max(0) as usize)
                .chain(iter::repeat_n('<', (-rights).max(0) as usize))
                .chain(iter::repeat_n('v', downs.max(0) as usize))
                .chain(iter::repeat_n('^', (-downs).max(0) as usize))
                .collect::<Vec<_>>();
            let moves_c = moves.len();
            let shortest = permutations(&mut moves, moves_c)
                .iter_mut()
                .filter(|p| {
                    let mut start = from;
                    for c in p.iter() {
                        start = start.move_(Direction::from_char(*c).unwrap());
                        if start == deadzone {
                            return false;
                        }
                    }

                    true
                })
                .map(|p| {
                    p.push('A');
                    count_button_presses(&p, depth + 1, limit, seen)
                })
                .min()
                .unwrap();

            (loc, sum + shortest)
        })
        .1;

    seen.insert((key, depth, limit), result);

    result
}

fn keypad_button_to_coord(button: char) -> Coord {
    match button {
        '7' => Coord(0, 0),
        '8' => Coord(1, 0),
        '9' => Coord(2, 0),
        '4' => Coord(0, 1),
        '5' => Coord(1, 1),
        '6' => Coord(2, 1),
        '1' => Coord(0, 2),
        '2' => Coord(1, 2),
        '3' => Coord(2, 2),
        '0' => Coord(1, 3),
        'A' => Coord(2, 3),
        '#' => Coord(0, 3),
        _ => unreachable!(),
    }
}

fn directionpad_to_coord(button: char) -> Coord {
    match button {
        'A' => Coord(2, 0),
        '#' => Coord(0, 0),
        '^' => Coord(1, 0),
        'v' => Coord(1, 1),
        '<' => Coord(0, 1),
        '>' => Coord(2, 1),
        _ => unreachable!(),
    }
}
