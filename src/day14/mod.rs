use std::{collections::HashMap, sync::LazyLock};

use regex::Regex;

static ROBOT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap());

const INPUT: &'static str = include_str!("./input.in");

pub fn run() {
    let robots = parse_input(INPUT);
    let mut map = init_map(&robots);
    for _ in 0..100 {
        map = step(map)
    }

    println!("Part 1: {}", count_quadrants(&map));

    let mut i = 100;
    while !is_maybe_christmas_tree(&map) {
        i += 1;
        map = step(map);
    }
    println!("Part 2: {}", i);
}

#[derive(Clone, Copy)]
struct Robot {
    position: (usize, usize),
    velocity: (isize, isize),
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let parsed = ROBOT_RE.captures(line).unwrap();
            let (_, [px, py, vx, vy]) = parsed.extract();

            Robot {
                position: (px.parse().unwrap(), py.parse().unwrap()),
                velocity: (vx.parse().unwrap(), vy.parse().unwrap()),
            }
        })
        .collect()
}

type TileMap = HashMap<(usize, usize), Vec<Robot>>;

fn init_map(robots: &Vec<Robot>) -> TileMap {
    let mut map = HashMap::<(usize, usize), Vec<Robot>>::new();

    for robot in robots {
        map.entry(robot.position).or_default().push(*robot);
    }

    map
}

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

fn step(map: TileMap) -> TileMap {
    let mut new_map: TileMap = HashMap::new();
    for (pos, robots) in map {
        let (px, py) = pos;
        for robot in robots {
            let (vx, vy) = robot.velocity;

            let mut px = px as isize + vx;
            let mut py = py as isize + vy;

            if px < 0 {
                px += WIDTH;
            } else if px >= WIDTH {
                px -= WIDTH;
            }
            if py < 0 {
                py += HEIGHT
            } else if py >= HEIGHT {
                py -= HEIGHT
            }

            new_map
                .entry((px as usize, py as usize))
                .or_default()
                .push(robot)
        }
    }

    new_map
}

fn determine_quadrant(x: usize, y: usize) -> Option<usize> {
    let midwidth = (WIDTH as usize) / 2;
    let midheight = (HEIGHT as usize) / 2;

    if x == midwidth || y == midheight {
        None
    } else if x < midwidth {
        if y < midheight {
            Some(0)
        } else {
            Some(1)
        }
    } else if y < midheight {
        Some(2)
    } else {
        Some(3)
    }
}

fn count_quadrants(map: &TileMap) -> usize {
    let mut counts = [0; 4];

    for ((x, y), robots) in map {
        if let Some(i) = determine_quadrant(*x, *y) {
            counts[i] += robots.len()
        }
    }

    counts[0] * counts[1] * counts[2] * counts[3]
}

#[allow(dead_code)]
fn show_map(map: &TileMap) {
    for y in 0..HEIGHT as usize {
        let mut line = String::new();
        for x in 0..WIDTH as usize {
            if map.contains_key(&(x, y)) {
                line.push('*');
            } else {
                line.push(' ');
            }
        }
        println!("{}", line);
    }
}

fn is_maybe_christmas_tree(map: &TileMap) -> bool {
    for y in 0..HEIGHT as usize - 2 {
        for x in 0..WIDTH as usize - 1 {
            /*
            Look for
                *
               ***
              *****
            */
            if map.contains_key(&(x, y))
                && map.contains_key(&(x - 1, y + 1))
                && map.contains_key(&(x, y + 1))
                && map.contains_key(&(x + 1, y + 1))
                && map.contains_key(&(x - 1, y + 2))
                && map.contains_key(&(x, y + 2))
                && map.contains_key(&(x + 1, y + 2))
            {
                return true;
            }
        }
    }

    false
}
